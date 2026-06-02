use avian2d::prelude::*;
use bevy::platform::collections::HashSet;
use bevy::{platform::collections::HashMap, prelude::*};
use bevy_falling_sand::utils::mesh_from_grid_cells;
use bevy_falling_sand::{prelude::*, utils::connected_components};
use bevy_turborand::GlobalRng;
use bevy_turborand::TurboRand;
use voronoice::{BoundingBox, Point, VoronoiBuilder};

use crate::tools::earthquake::debug::{DebugEarthquake, DebugEarthquakeInfo};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_earthquake)
            .add_observer(on_remove_fracture_body_cell_at_world_position)
            .add_observer(on_remove_fracture_body_cells_at_world_positions)
            .add_observer(on_remove_fracture_body_cells);
    }
}

const EARTHQUAKE_RIGID_BODY_RENDER_Z: f32 = 1.0;
const MIN_FRACTURE_BODY_CELLS: usize = 2;

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

struct BuiltFractureBody {
    source_centroid: Vec2,
    collider: Collider,
    particle_collider: ParticleCollider,
    sprites: Vec<(Vec2, Color)>,
}

#[derive(Component, Clone)]
struct FractureBody {
    cells: HashMap<IVec2, Color>,
    source_centroid: Vec2,
}

#[derive(Clone, Debug)]
pub enum EarthquakeRegion {
    Circle {
        center: Vec2,
        radius: f32,
    },
    Rect {
        center: Vec2,
        half_extents: Vec2,
        rotation: f32,
    },
    Polygon {
        vertices: Vec<Vec2>,
    },
}

impl EarthquakeRegion {
    pub fn circle(center: Vec2, radius: f32) -> Self {
        Self::Circle { center, radius }
    }

    pub fn rect(center: Vec2, half_extents: Vec2, rotation: f32) -> Self {
        Self::Rect {
            center,
            half_extents,
            rotation,
        }
    }

    pub fn polygon(vertices: Vec<Vec2>) -> Self {
        Self::Polygon { vertices }
    }

    fn area_hint(&self) -> f32 {
        match self {
            Self::Circle { radius, .. } => std::f32::consts::PI * radius * radius,
            Self::Rect { half_extents, .. } => half_extents.x * half_extents.y * 4.0,
            Self::Polygon { vertices } => polygon_signed_area(vertices).abs() * 0.5,
        }
    }

    fn bounds(&self) -> IRect {
        match self {
            Self::Circle { center, radius } => {
                let half = Vec2::splat(*radius);
                IRect::from_corners(
                    (*center - half).floor().as_ivec2(),
                    (*center + half).ceil().as_ivec2(),
                )
            }
            Self::Rect {
                center,
                half_extents,
                rotation,
            } => {
                let rot = Rot2::radians(*rotation);
                let corners = [
                    Vec2::new(-half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, half_extents.y),
                    Vec2::new(-half_extents.x, half_extents.y),
                ]
                .map(|corner| *center + rot * corner);
                points_bounds(corners)
                    .unwrap_or_else(|| IRect::from_corners(IVec2::ZERO, IVec2::ZERO))
            }
            Self::Polygon { vertices } => points_bounds(vertices.iter().copied())
                .unwrap_or_else(|| IRect::from_corners(IVec2::ZERO, IVec2::ZERO)),
        }
        .inflate(1)
    }

    fn contains_point(&self, point: Vec2) -> bool {
        match self {
            Self::Circle { center, radius } => point.distance_squared(*center) <= radius * radius,
            Self::Rect {
                center,
                half_extents,
                rotation,
            } => {
                let local = Rot2::radians(-*rotation) * (point - *center);
                local.x.abs() <= half_extents.x && local.y.abs() <= half_extents.y
            }
            Self::Polygon { vertices } => polygon_contains_point(vertices, point),
        }
    }
}

fn points_bounds<I>(points: I) -> Option<IRect>
where
    I: IntoIterator<Item = Vec2>,
{
    let mut points = points.into_iter();
    let first = points.next()?;
    let mut min = first;
    let mut max = first;
    for point in points {
        min = min.min(point);
        max = max.max(point);
    }
    Some(IRect::from_corners(
        min.floor().as_ivec2(),
        max.ceil().as_ivec2(),
    ))
}

fn polygon_signed_area(vertices: &[Vec2]) -> f32 {
    if vertices.len() < 3 {
        return 0.0;
    }

    vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum()
}

fn polygon_contains_point(vertices: &[Vec2], point: Vec2) -> bool {
    if vertices.len() < 3 {
        return false;
    }

    let mut inside = false;
    for (a, b) in vertices.iter().zip(vertices.iter().cycle().skip(1)) {
        let crosses_y = (a.y > point.y) != (b.y > point.y);
        if crosses_y {
            let x_intersection = (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x;
            if point.x < x_intersection {
                inside = !inside;
            }
        }
    }
    inside
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

    let fracture_edges: Vec<(Vec2, Vec2)> = cells.iter().flat_map(cell_boundary_edges).collect();

    for cell in &cells {
        spawn_fracture_body(&mut commands, &particle_colors, &by_position, cell);
    }

    for cell in &cells {
        for pos in cell {
            if let Some(&entity) = by_position.get(pos) {
                despawn_writer.write(DespawnParticleSignal::from_entity(entity));
            }
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
        // JAB TODO: Despawn this entity after duration using `DelayedCommands` once Bevy releases it.
        commands.spawn(DebugEarthquakeInfo {
            region: region.clone(),
            fracture_edges,
            timer: Timer::from_seconds(5., TimerMode::Once),
        });
    }
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

    for cell in &removed {
        mark_fracture_cell_dirty(
            *cell,
            fracture_body.source_centroid,
            global_transform,
            &chunk_index,
            &mut chunk_query,
        );
    }

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

    let retained_cells = cell_colors_for_component(&original.cells, &components[0]);
    fracture_body.cells = retained_cells;
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
        let Some(built) = build_fracture_body(&cell_colors) else {
            continue;
        };

        let split_transform = shifted_fracture_transform(
            original_transform,
            original_source_centroid,
            built.source_centroid,
        );

        let split_entity = commands
            .spawn((
                split_transform,
                Visibility::default(),
                body_kind,
                built.collider,
                built.particle_collider.with_resting(resting),
                FractureBody {
                    cells: cell_colors,
                    source_centroid: built.source_centroid,
                },
            ))
            .with_children(|p| {
                spawn_fracture_body_sprites(p, built.source_centroid, &built.sprites);
            })
            .id();

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

fn spawn_fracture_body(
    commands: &mut Commands,
    particle_colors: &Query<&ParticleColor>,
    by_position: &HashMap<IVec2, Entity>,
    cell: &HashSet<IVec2>,
) {
    if cell.is_empty() {
        return;
    }

    let collider_cell = erode_outer_perimeter_layers(cell, 2);
    if collider_cell.is_empty() {
        return;
    }

    let cell_colors: HashMap<IVec2, Color> = collider_cell
        .iter()
        .map(|grid_pos| {
            let color = by_position
                .get(grid_pos)
                .and_then(|e| particle_colors.get(*e).ok())
                .map_or(Color::WHITE, |pc| pc.0);
            (*grid_pos, color)
        })
        .collect();

    spawn_fracture_body_from_cells(commands, cell_colors, None);
}

fn generate_voronoi_cells(
    rng: &mut GlobalRng,
    region: &EarthquakeRegion,
    bounds: IRect,
    particles: &[IVec2],
) -> Vec<HashSet<IVec2>> {
    if particles.is_empty() {
        return Vec::new();
    }

    let target_count = ((region.area_hint() * 0.01) as usize).clamp(8, 256);
    let site_count = target_count.min(particles.len());

    let sites: Vec<Point> = rng
        .as_mut()
        .sample_multiple(particles, site_count)
        .into_iter()
        .map(|p| Point {
            x: p.x as f64,
            y: p.y as f64,
        })
        .collect();

    let bbox = voronoi_bounding_box(bounds);

    let Some(voronoi) = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(bbox)
        .build()
    else {
        return Vec::new();
    };

    let final_sites = voronoi.sites();
    let mut cells: HashMap<usize, HashSet<IVec2>> = HashMap::default();
    for &p in particles {
        let mut best_idx: usize = 0;
        let mut best_dist = f32::INFINITY;
        for (i, site) in final_sites.iter().enumerate() {
            let dx = p.x as f32 - site.x as f32;
            let dy = p.y as f32 - site.y as f32;
            let d = dx * dx + dy * dy;
            if d < best_dist {
                best_dist = d;
                best_idx = i;
            }
        }
        cells.entry(best_idx).or_default().insert(p);
    }

    cells.into_values().collect()
}

fn voronoi_bounding_box(bounds: IRect) -> BoundingBox {
    let min = bounds.min.as_vec2();
    let max = bounds.max.as_vec2();
    let center = (min + max) * 0.5;
    let size = (max - min).max(Vec2::splat(1.0));
    BoundingBox::new(
        Point {
            x: center.x as f64,
            y: center.y as f64,
        },
        size.x as f64,
        size.y as f64,
    )
}

fn cell_boundary_edges(cell: &HashSet<IVec2>) -> Vec<(Vec2, Vec2)> {
    let mut edges = Vec::new();
    for &p in cell {
        let fx = p.x as f32;
        let fy = p.y as f32;
        if !cell.contains(&IVec2::new(p.x, p.y + 1)) {
            edges.push((Vec2::new(fx, fy + 1.0), Vec2::new(fx + 1.0, fy + 1.0)));
        }
        if !cell.contains(&IVec2::new(p.x, p.y - 1)) {
            edges.push((Vec2::new(fx, fy), Vec2::new(fx + 1.0, fy)));
        }
        if !cell.contains(&IVec2::new(p.x + 1, p.y)) {
            edges.push((Vec2::new(fx + 1.0, fy), Vec2::new(fx + 1.0, fy + 1.0)));
        }
        if !cell.contains(&IVec2::new(p.x - 1, p.y)) {
            edges.push((Vec2::new(fx, fy), Vec2::new(fx, fy + 1.0)));
        }
    }
    edges
}

fn erode_outer_perimeter_layers(cell: &HashSet<IVec2>, layers: usize) -> HashSet<IVec2> {
    let mut eroded = cell.clone();
    for _ in 0..layers {
        eroded = erode_outer_perimeter(&eroded);
        if eroded.is_empty() {
            break;
        }
    }
    eroded
}

fn erode_outer_perimeter(cell: &HashSet<IVec2>) -> HashSet<IVec2> {
    cell.iter()
        .copied()
        .filter(|p| {
            [
                IVec2::new(p.x, p.y + 1),
                IVec2::new(p.x, p.y - 1),
                IVec2::new(p.x + 1, p.y),
                IVec2::new(p.x - 1, p.y),
            ]
            .into_iter()
            .all(|neighbor| cell.contains(&neighbor))
        })
        .collect()
}

fn particle_world_positions<'a, I>(cell: I) -> Vec<(IVec2, Vec2)>
where
    I: IntoIterator<Item = &'a IVec2>,
{
    cell.into_iter()
        .map(|&p| (p, Vec2::new(p.x as f32 + 0.5, p.y as f32 + 0.5)))
        .collect()
}

fn particle_centroid(particle_world: &[(IVec2, Vec2)]) -> Vec2 {
    particle_world.iter().map(|(_, v)| *v).sum::<Vec2>() / particle_world.len() as f32
}

fn compound_particle_collider(particle_world: &[(IVec2, Vec2)], centroid: Vec2) -> Collider {
    let shapes: Vec<(Vec2, f32, Collider)> = particle_world
        .iter()
        .map(|(_, world)| (*world - centroid, 0.0, Collider::rectangle(1.0, 1.0)))
        .collect();
    Collider::compound(shapes)
}

fn convex_particle_collider(
    cell: &HashSet<IVec2>,
    particle_world: &[(IVec2, Vec2)],
    centroid: Vec2,
) -> Collider {
    let mesh = mesh_from_grid_cells(cell.iter().copied(), 0.0);
    if mesh.vertices.is_empty() {
        return compound_particle_collider(particle_world, centroid);
    }

    let vertices: Vec<Vec2> = mesh
        .vertices
        .into_iter()
        .map(|vertex| vertex - centroid)
        .collect();

    Collider::convex_hull(vertices)
        .unwrap_or_else(|| compound_particle_collider(particle_world, centroid))
}

fn spawn_fracture_body_from_cells(
    commands: &mut Commands,
    cell_colors: HashMap<IVec2, Color>,
    transform: Option<Transform>,
) -> Option<Entity> {
    let built = build_fracture_body(&cell_colors)?;

    Some(
        commands
            .spawn((
                transform.unwrap_or_else(|| {
                    Transform::from_xyz(
                        built.source_centroid.x,
                        built.source_centroid.y,
                        EARTHQUAKE_RIGID_BODY_RENDER_Z,
                    )
                }),
                Visibility::default(),
                RigidBody::Dynamic,
                built.collider,
                built.particle_collider.with_default_resting(),
                FractureBody {
                    cells: cell_colors,
                    source_centroid: built.source_centroid,
                },
            ))
            .with_children(|p| {
                spawn_fracture_body_sprites(p, built.source_centroid, &built.sprites);
            })
            .id(),
    )
}

fn build_fracture_body(cell_colors: &HashMap<IVec2, Color>) -> Option<BuiltFractureBody> {
    if cell_colors.len() < MIN_FRACTURE_BODY_CELLS {
        return None;
    }

    let cells: HashSet<IVec2> = cell_colors.keys().copied().collect();
    let particle_world = particle_world_positions(cells.iter());
    let centroid = particle_centroid(&particle_world);
    let collider = convex_particle_collider(&cells, &particle_world, centroid);
    let sprites = particle_world
        .iter()
        .map(|(grid_pos, world)| {
            let color = cell_colors.get(grid_pos).copied().unwrap_or(Color::WHITE);
            (*world, color)
        })
        .collect();

    Some(BuiltFractureBody {
        source_centroid: centroid,
        collider,
        particle_collider: ParticleCollider::from_grid_cells(cells, centroid),
        sprites,
    })
}

fn spawn_fracture_body_sprites(
    parent: &mut ChildSpawnerCommands,
    centroid: Vec2,
    sprites: &[(Vec2, Color)],
) {
    for (world, color) in sprites {
        let local = *world - centroid;
        parent.spawn((
            Sprite::from_color(*color, Vec2::ONE),
            Transform::from_xyz(local.x, local.y, 0.0),
        ));
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
    let Some(built) = build_fracture_body(&cell_colors) else {
        commands.entity(entity).despawn();
        return;
    };

    let old_centroid = fracture_body.source_centroid;
    fracture_body.cells = cell_colors;
    fracture_body.source_centroid = built.source_centroid;
    *collider = built.collider;
    *particle_collider = built
        .particle_collider
        .with_resting(particle_collider.resting);
    *transform = shifted_fracture_transform(*transform, old_centroid, built.source_centroid);

    if let Ok(children) = children.get(entity) {
        for child in children {
            commands.entity(*child).despawn();
        }
    }

    commands.entity(entity).with_children(|p| {
        spawn_fracture_body_sprites(p, built.source_centroid, &built.sprites);
    });

    mark_fracture_cells_dirty(
        component.iter().copied(),
        built.source_centroid,
        &GlobalTransform::from(*transform),
        chunk_index,
        chunk_query,
    );
}

fn shifted_fracture_transform(
    mut transform: Transform,
    old_source_centroid: Vec2,
    new_source_centroid: Vec2,
) -> Transform {
    let local_delta = new_source_centroid - old_source_centroid;
    transform.translation += transform.rotation * local_delta.extend(0.0);
    transform
}

fn cell_colors_for_component(
    colors: &HashMap<IVec2, Color>,
    component: &[IVec2],
) -> HashMap<IVec2, Color> {
    component
        .iter()
        .filter_map(|cell| colors.get(cell).copied().map(|color| (*cell, color)))
        .collect()
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
