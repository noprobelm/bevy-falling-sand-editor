use std::time::Duration;

use avian2d::prelude::*;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
use bevy_falling_sand::{prelude::*, utils::connected_components};
use bevy_turborand::{DelegatedRng, GlobalRng};

use crate::tools::earthquake::{
    EarthquakeRegion,
    debug::{DebugEarthquake, DebugEarthquakeInfo},
    fracture::{
        FractureBody, MIN_FRACTURE_BODY_CELLS, apply_built_fracture_body, build_fracture_body,
        cell_colors_for_component, cell_colors_for_voronoi_cell, fracture_debug_edges,
        shifted_fracture_transform, spawn_built_fracture_body, spawn_fracture_body_from_cells,
        trim_outer_perimeter_cells,
    },
    states::EarthquakeFractureShapeState,
    voronoi::generate_voronoi_cells,
};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_earthquake)
            .add_observer(on_remove_fracture_body_cell_at_world_position)
            .add_observer(on_remove_fracture_body_cells_at_world_positions)
            .add_observer(on_remove_fracture_body_cells)
            .add_systems(Update, process_pending_exact_voronoi_fractures);
    }
}

const PENDING_EXACT_VORONOI_FRACTURE_TICK: Duration = Duration::from_millis(100);
const PENDING_EXACT_VORONOI_PERIMETER_MIN_BATCH: usize = 1;
const PENDING_EXACT_VORONOI_PERIMETER_MAX_BATCH: usize = 6;

#[derive(Component)]
struct PendingExactVoronoiFracture {
    cell_colors: HashMap<IVec2, Color>,
    source_entities: HashMap<IVec2, Entity>,
    perimeter: Vec<IVec2>,
    timer: Timer,
}

#[derive(Event)]
pub(super) struct RemoveFractureBodyCells {
    entity: Entity,
    cells: Vec<IVec2>,
}

#[derive(Event)]
pub(crate) struct RemoveFractureBodyCellAtWorldPosition {
    pub position: IVec2,
}

#[derive(Event)]
pub(crate) struct RemoveFractureBodyCellsAtWorldPositions {
    pub positions: Vec<IVec2>,
}

#[derive(Event)]
pub struct Earthquake {
    pub region: EarthquakeRegion,
}

fn on_earthquake(
    trigger: On<Earthquake>,
    mut commands: Commands,
    mut rng: ResMut<GlobalRng>,
    mut despawn_writer: MessageWriter<DespawnParticleSignal>,
    map: Res<ParticleMap>,
    static_particles: Query<&StaticRigidBodyParticle>,
    particle_colors: Query<&ParticleColor>,
    debug_earthquake: Option<Res<DebugEarthquake>>,
    fracture_shape_state: Res<State<EarthquakeFractureShapeState>>,
) {
    let region = &trigger.region;
    let bounds = region.bounds();
    let mut by_position: HashMap<IVec2, Entity> = HashMap::default();
    map.within_rect(bounds).for_each(|(pos, entity)| {
        if static_particles.contains(entity) {
            let cell_center = pos.as_vec2() + Vec2::splat(0.5);
            if region.contains_point(cell_center) {
                by_position.insert(pos, entity);
            }
        }
    });

    let shape_count = connected_components(by_position.keys().copied()).len();
    let particle_positions: Vec<IVec2> = by_position.keys().copied().collect();
    let cells = generate_voronoi_cells(&mut rng, region, bounds, &particle_positions);
    let fracture_shape = **fracture_shape_state;

    let fracture_edges: Vec<(Vec2, Vec2)> = cells
        .iter()
        .flat_map(|cell| fracture_debug_edges(cell, fracture_shape))
        .collect();

    for cell in &cells {
        let mut cell_colors =
            cell_colors_for_voronoi_cell(cell, fracture_shape, &particle_colors, &by_position);
        if fracture_shape == EarthquakeFractureShapeState::ExactVoronoiCells {
            let source_entities = source_entities_for_cells(&by_position, cell_colors.keys());
            let mut perimeter = trim_outer_perimeter_cells(&mut cell_colors);
            rng.shuffle(&mut perimeter);
            commands.spawn(PendingExactVoronoiFracture {
                cell_colors,
                source_entities,
                perimeter,
                timer: Timer::new(PENDING_EXACT_VORONOI_FRACTURE_TICK, TimerMode::Repeating),
            });
            continue;
        }

        spawn_fracture_body_from_cells(
            &mut commands,
            cell_colors,
            fracture_shape == EarthquakeFractureShapeState::ExactVoronoiCells,
        );
    }

    if fracture_shape != EarthquakeFractureShapeState::ExactVoronoiCells {
        for entity in by_position.values() {
            despawn_writer.write(DespawnParticleSignal::from_entity(*entity));
        }
    }

    debug!(
        "earthquake in {:?}: {} shapes, {} cells, {} fracture edges",
        region,
        shape_count,
        cells.len(),
        fracture_edges.len(),
    );

    if debug_earthquake.is_some() {
        commands.spawn(DebugEarthquakeInfo {
            region: region.clone(),
            fracture_edges,
            timer: Timer::from_seconds(5., TimerMode::Once),
        });
    }
}

fn source_entities_for_cells<'a, I>(
    by_position: &HashMap<IVec2, Entity>,
    cells: I,
) -> HashMap<IVec2, Entity>
where
    I: IntoIterator<Item = &'a IVec2>,
{
    cells
        .into_iter()
        .filter_map(|cell| by_position.get(cell).copied().map(|entity| (*cell, entity)))
        .collect()
}

fn process_pending_exact_voronoi_fractures(
    mut commands: Commands,
    time: Res<Time>,
    mut rng: ResMut<GlobalRng>,
    mut despawn_writer: MessageWriter<DespawnParticleSignal>,
    mut pending_fractures: Query<(Entity, &mut PendingExactVoronoiFracture)>,
) {
    for (entity, mut pending) in &mut pending_fractures {
        pending.timer.tick(time.delta());
        if !pending.timer.just_finished() {
            continue;
        }

        release_pending_perimeter_batch(&mut commands, &mut rng, &mut pending);

        if pending.perimeter.is_empty() {
            finish_pending_exact_voronoi_fracture(
                &mut commands,
                entity,
                &mut despawn_writer,
                &mut pending,
            );
        }
    }
}

fn release_pending_perimeter_batch(
    commands: &mut Commands,
    rng: &mut GlobalRng,
    pending: &mut PendingExactVoronoiFracture,
) {
    let batch_size = rng
        .usize(
            PENDING_EXACT_VORONOI_PERIMETER_MIN_BATCH..=PENDING_EXACT_VORONOI_PERIMETER_MAX_BATCH,
        )
        .min(pending.perimeter.len());

    for _ in 0..batch_size {
        let Some(position) = pending.perimeter.pop() else {
            break;
        };
        if let Some(entity) = pending.source_entities.remove(&position) {
            insert_movable_solid_behavior(commands, entity);
        }
    }
}

fn finish_pending_exact_voronoi_fracture(
    commands: &mut Commands,
    pending_entity: Entity,
    despawn_writer: &mut MessageWriter<DespawnParticleSignal>,
    pending: &mut PendingExactVoronoiFracture,
) {
    let cell_colors = std::mem::take(&mut pending.cell_colors);
    let source_entities = std::mem::take(&mut pending.source_entities);
    let interior_cells: Vec<IVec2> = cell_colors.keys().copied().collect();

    let spawned = spawn_fracture_body_from_cells(commands, cell_colors, true).is_some();
    for cell in interior_cells {
        let Some(entity) = source_entities.get(&cell).copied() else {
            continue;
        };
        if spawned {
            despawn_writer.write(DespawnParticleSignal::from_entity(entity));
        } else {
            insert_movable_solid_behavior(commands, entity);
        }
    }

    commands.entity(pending_entity).despawn();
}

fn insert_movable_solid_behavior(commands: &mut Commands, entity: Entity) {
    let Ok(mut entity_commands) = commands.get_entity(entity) else {
        return;
    };

    entity_commands.insert((
        Density(1250),
        Momentum(IVec2::ZERO),
        movable_solid_movement(),
        AirResistance::new([0.0, 0.9]),
        Speed::new(5, 10),
    ));
}

fn movable_solid_movement() -> Movement {
    Movement::new(
        vec![
            NeighborGroup::new(vec![IVec2::new(0, -1)].into()),
            NeighborGroup::new(vec![IVec2::new(-1, -1), IVec2::new(1, -1)].into()),
        ]
        .into(),
    )
}

fn on_remove_fracture_body_cell_at_world_position(
    trigger: On<RemoveFractureBodyCellAtWorldPosition>,
    mut commands: Commands,
) {
    commands.trigger(RemoveFractureBodyCellsAtWorldPositions {
        positions: vec![trigger.position],
    });
}

fn on_remove_fracture_body_cells_at_world_positions(
    trigger: On<RemoveFractureBodyCellsAtWorldPositions>,
    mut commands: Commands,
    bodies: Query<(Entity, &ParticleCollider, &GlobalTransform), With<FractureBody>>,
) {
    let mut cells_by_entity: HashMap<Entity, HashSet<IVec2>> = HashMap::default();

    for position in &trigger.positions {
        let world_point = position.as_vec2() + Vec2::splat(0.5);

        for (entity, particle_collider, global_transform) in &bodies {
            let cell = particle_collider.cell_at_world_point(world_point, global_transform);
            if particle_collider.contains_cell(cell) {
                cells_by_entity.entry(entity).or_default().insert(cell);
                break;
            }
        }
    }

    for (entity, cells) in cells_by_entity {
        commands.trigger(RemoveFractureBodyCells {
            entity,
            cells: cells.into_iter().collect(),
        });
    }
}

#[allow(clippy::too_many_arguments, clippy::type_complexity)]
fn on_remove_fracture_body_cells(
    trigger: On<RemoveFractureBodyCells>,
    mut commands: Commands,
    mut bodies: Query<(
        Entity,
        &mut FractureBody,
        &mut Collider,
        &mut ParticleCollider,
        &mut Transform,
        &GlobalTransform,
        &RigidBody,
        Option<&LinearVelocity>,
        Option<&AngularVelocity>,
    )>,
    children: Query<&Children>,
    chunk_index: Res<bevy_falling_sand::prelude::ChunkIndex>,
    mut chunk_query: Query<&mut bevy_falling_sand::prelude::ChunkDirtyState>,
) {
    let Ok((
        entity,
        mut fracture_body,
        mut collider,
        mut particle_collider,
        mut transform,
        global_transform,
        rigid_body,
        linear_velocity,
        angular_velocity,
    )) = bodies.get_mut(trigger.entity)
    else {
        return;
    };

    let mut removed = Vec::new();
    for cell in &trigger.cells {
        if fracture_body.cells.remove(cell).is_some() {
            removed.push(*cell);
        }
    }

    if removed.is_empty() {
        return;
    }

    mark_fracture_cells_dirty(
        removed,
        fracture_body.source_centroid,
        global_transform,
        &chunk_index,
        &mut chunk_query,
    );

    if fracture_body.cells.is_empty() {
        commands.entity(entity).despawn();
        return;
    }

    let mut components: Vec<Vec<IVec2>> = connected_components(fracture_body.cells.keys().copied())
        .into_iter()
        .filter(|component| {
            if component.len() < MIN_FRACTURE_BODY_CELLS {
                mark_fracture_cells_dirty(
                    component.iter().copied(),
                    fracture_body.source_centroid,
                    global_transform,
                    &chunk_index,
                    &mut chunk_query,
                );
                false
            } else {
                true
            }
        })
        .collect();
    components.sort_by_key(|component| std::cmp::Reverse(component.len()));

    if components.is_empty() {
        commands.entity(entity).despawn();
        return;
    }

    if components.len() == 1 {
        rebuild_fracture_body(
            &mut commands,
            entity,
            &children,
            &mut fracture_body,
            &mut collider,
            &mut particle_collider,
            &mut transform,
            &components[0],
            &chunk_index,
            &mut chunk_query,
        );
        return;
    }

    let original = fracture_body.clone();
    let original_transform = *transform;
    let original_global_transform = *global_transform;
    let original_source_centroid = original.source_centroid;
    let resting = particle_collider.resting;
    let body_kind = *rigid_body;
    let linear_velocity = linear_velocity.copied();
    let angular_velocity = angular_velocity.copied();

    rebuild_fracture_body(
        &mut commands,
        entity,
        &children,
        &mut fracture_body,
        &mut collider,
        &mut particle_collider,
        &mut transform,
        &components[0],
        &chunk_index,
        &mut chunk_query,
    );

    for component in components.iter().skip(1) {
        let cell_colors = cell_colors_for_component(&original.cells, component);
        let Some(built) = build_fracture_body(&cell_colors, false) else {
            continue;
        };

        let split_transform = shifted_fracture_transform(
            original_transform,
            original_source_centroid,
            built.source_centroid,
        );

        let split_entity = spawn_built_fracture_body(
            &mut commands,
            cell_colors,
            built,
            split_transform,
            body_kind,
            |particle_collider| particle_collider.with_resting(resting),
        );

        if let Some(linear_velocity) = linear_velocity {
            commands.entity(split_entity).insert(linear_velocity);
        }
        if let Some(angular_velocity) = angular_velocity {
            commands.entity(split_entity).insert(angular_velocity);
        }

        mark_fracture_cells_dirty(
            component.iter().copied(),
            original_source_centroid,
            &original_global_transform,
            &chunk_index,
            &mut chunk_query,
        );
    }
}

#[allow(clippy::too_many_arguments)]
fn rebuild_fracture_body(
    commands: &mut Commands,
    entity: Entity,
    children: &Query<&Children>,
    fracture_body: &mut FractureBody,
    collider: &mut Collider,
    particle_collider: &mut ParticleCollider,
    transform: &mut Transform,
    component: &[IVec2],
    chunk_index: &bevy_falling_sand::prelude::ChunkIndex,
    chunk_query: &mut Query<&mut bevy_falling_sand::prelude::ChunkDirtyState>,
) {
    let cell_colors = cell_colors_for_component(&fracture_body.cells, component);
    let Some(built) = build_fracture_body(&cell_colors, false) else {
        commands.entity(entity).despawn();
        return;
    };

    let source_centroid = apply_built_fracture_body(
        commands,
        entity,
        children,
        fracture_body,
        collider,
        particle_collider,
        transform,
        cell_colors,
        built,
    );

    mark_fracture_cells_dirty(
        component.iter().copied(),
        source_centroid,
        &GlobalTransform::from(*transform),
        chunk_index,
        chunk_query,
    );
}

fn mark_fracture_cells_dirty<I>(
    cells: I,
    source_centroid: Vec2,
    transform: &GlobalTransform,
    chunk_index: &bevy_falling_sand::prelude::ChunkIndex,
    chunk_query: &mut Query<&mut bevy_falling_sand::prelude::ChunkDirtyState>,
) where
    I: IntoIterator<Item = IVec2>,
{
    for cell in cells {
        mark_fracture_cell_dirty(cell, source_centroid, transform, chunk_index, chunk_query);
    }
}

fn mark_fracture_cell_dirty(
    cell: IVec2,
    source_centroid: Vec2,
    transform: &GlobalTransform,
    chunk_index: &bevy_falling_sand::prelude::ChunkIndex,
    chunk_query: &mut Query<&mut bevy_falling_sand::prelude::ChunkDirtyState>,
) {
    let local = cell.as_vec2() + Vec2::splat(0.5) - source_centroid;
    let world = transform.transform_point(local.extend(0.0)).truncate();
    let position = world.floor().as_ivec2();
    let chunk_coord = chunk_index.world_to_chunk_coord(position);
    if let Some(chunk_entity) = chunk_index.get(chunk_coord)
        && let Ok(mut dirty_state) = chunk_query.get_mut(chunk_entity)
    {
        dirty_state.mark_dirty(position);
    }
}
