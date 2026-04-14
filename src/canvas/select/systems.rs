use bevy::prelude::*;
use bevy_falling_sand::prelude::*;
use leafwing_input_manager::{
    common_conditions::{action_just_pressed, action_just_released, action_pressed},
    prelude::ActionState,
};

use crate::{
    Cursor,
    canvas::{
        CanvasAction,
        select::{
            SelectAction,
            gizmos::SelectGizmos,
            resources::{DragOrigins, LastClickTime, SelectedParticles},
            states::{SelectModeState, SelectState},
        },
    },
    ui::CanvasState,
};

use super::resources::SelectedRegion;
use super::setup::OverlayImage;

const DOUBLE_CLICK_THRESHOLD: f64 = 0.3;
const THROW_VELOCITY_SCALE: f32 = 10.0;

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_select_action_pressed
                    .run_if(action_just_pressed(CanvasAction::Draw))
                    .run_if(in_state(SelectState::Idle)),
                (
                    update_selected_region.run_if(action_pressed(CanvasAction::Draw)),
                    commit_selected_region.run_if(action_just_released(CanvasAction::Draw)),
                )
                    .chain()
                    .run_if(in_state(SelectState::ExpandSelection)),
                (
                    update_drag_overlays.run_if(action_pressed(CanvasAction::Draw)),
                    finish_select_action.run_if(action_just_released(CanvasAction::Draw)),
                )
                    .chain()
                    .run_if(in_state(SelectState::DragParticles)),
                sync_overlay_positions
                    .run_if(not(in_state(SelectState::DragParticles)))
                    .run_if(in_state(CanvasState::Select)),
            ),
        )
        .add_systems(OnExit(CanvasState::Select), cleanup_drag_state);
    }
}

// Components

/// Links an overlay sprite to the particle entity it tracks.
#[derive(Component)]
struct SelectionOverlay(Entity);

// Helpers

fn mark_dirty(pos: IVec2, chunk_index: &ChunkIndex, chunk_query: &mut Query<&mut ChunkDirtyState>) {
    let coord = chunk_index.world_to_chunk_coord(pos);
    if let Some(chunk_entity) = chunk_index.get(coord)
        && let Ok(mut dirty_state) = chunk_query.get_mut(chunk_entity)
    {
        dirty_state.mark_dirty(pos);
        match &mut dirty_state.current {
            Some(r) => *r = r.union_point(pos),
            None => dirty_state.current = Some(IRect::from_center_size(pos, IVec2::ONE)),
        }
    }
}

fn deselect_all(
    commands: &mut Commands,
    selected_particles: &mut SelectedParticles,
    overlays: &Query<Entity, With<SelectionOverlay>>,
) {
    for existing in selected_particles.particles.drain(..) {
        commands.trigger(SyncParticleSignal::from_entity(existing));
    }
    for entity in overlays {
        commands.entity(entity).despawn();
    }
}

fn spawn_particle_overlay(
    commands: &mut Commands,
    particle: Entity,
    position: IVec2,
    image: &Handle<Image>,
) {
    commands.spawn((
        SelectionOverlay(particle),
        Sprite {
            image: image.clone(),
            custom_size: Some(Vec2::ONE),
            ..default()
        },
        Transform::from_xyz(position.x as f32 + 0.5, position.y as f32 + 0.5, 10.0),
    ));
}

fn sync_overlays_to_positions(
    overlays: &mut Query<(Entity, &SelectionOverlay, &mut Transform)>,
    positions: &Query<&mut GridPosition>,
) {
    for (_, overlay, mut transform) in overlays.iter_mut() {
        if let Ok(grid_pos) = positions.get(overlay.0) {
            transform.translation.x = grid_pos.0.x as f32 + 0.5;
            transform.translation.y = grid_pos.0.y as f32 + 0.5;
        }
    }
}

// Idle Systems

fn handle_select_action_pressed(
    mut commands: Commands,
    cursor: Res<Cursor>,
    time: Res<Time<Real>>,
    mut selected_particles: ResMut<SelectedParticles>,
    map: Res<ParticleMap>,
    positions: Query<&GridPosition>,
    mut drag_origins: ResMut<DragOrigins>,
    mut last_click_time: ResMut<LastClickTime>,
    mut region: ResMut<SelectedRegion>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut next_state: ResMut<NextState<SelectState>>,
    overlay_image: Res<OverlayImage>,
    overlay_entities: Query<Entity, With<SelectionOverlay>>,
) {
    let cursor_pos = cursor.current.floor().as_ivec2();
    let clicked_entity = map.get(cursor_pos).ok().and_then(|e| e.copied());

    // Click on a selected particle → drag
    if let Some(entity) = clicked_entity
        && selected_particles.particles.contains(&entity)
    {
        drag_origins.cursor_start = cursor_pos;
        drag_origins.origins.clear();

        for entity in &selected_particles.particles {
            if let Ok(grid_position) = positions.get(*entity) {
                drag_origins.origins.insert(*entity, grid_position.0);
            }
            commands.entity(*entity).remove::<Movement>();
        }

        next_state.set(SelectState::DragParticles);
        return;
    }

    // Click on an unselected particle → add to selection
    if let Some(entity) = clicked_entity {
        if !selected_particles.particles.contains(&entity) {
            selected_particles.particles.push(entity);
            spawn_particle_overlay(&mut commands, entity, cursor_pos, &overlay_image.0);
        }
        return;
    }

    // Double-click on empty space → deselect all
    let now = time.elapsed_secs_f64();
    let is_double_click = (now - last_click_time.0) < DOUBLE_CLICK_THRESHOLD;
    last_click_time.0 = now;
    if is_double_click {
        deselect_all(&mut commands, &mut selected_particles, &overlay_entities);
        return;
    }

    // Begin lasso (cleared on commit unless Ctrl held)
    region.start = cursor.current;
    region.stop = cursor.current;
    let (config, _) = config_store.config_mut::<SelectGizmos>();
    config.enabled = true;
    next_state.set(SelectState::ExpandSelection);
}

// ExpandSelection Systems

fn update_selected_region(cursor: Res<Cursor>, mut region: ResMut<SelectedRegion>) {
    region.stop = cursor.current;
}

fn commit_selected_region(
    mut commands: Commands,
    mut selected_particles: ResMut<SelectedParticles>,
    selected_region: Res<SelectedRegion>,
    map: Res<ParticleMap>,
    mut config_store: ResMut<GizmoConfigStore>,
    mut next_state: ResMut<NextState<SelectState>>,
    overlay_image: Res<OverlayImage>,
    select_action: Single<&ActionState<SelectAction>>,
    overlay_entities: Query<Entity, With<SelectionOverlay>>,
) {
    let (config, _) = config_store.config_mut::<SelectGizmos>();
    config.enabled = false;

    // Zero-size region = single click on empty space → do nothing
    if selected_region.stop == selected_region.start {
        next_state.set(SelectState::Idle);
        return;
    }

    // Clear existing selection unless Ctrl is held
    if !select_action.pressed(&SelectAction::AddToSelection) {
        deselect_all(&mut commands, &mut selected_particles, &overlay_entities);
    }

    let rect = IRect::from_corners(
        selected_region.start.floor().as_ivec2(),
        selected_region.stop.floor().as_ivec2(),
    );
    for (pos, entity) in map.within_rect(rect) {
        if !selected_particles.particles.contains(&entity) {
            selected_particles.particles.push(entity);
            spawn_particle_overlay(&mut commands, entity, pos, &overlay_image.0);
        }
    }

    next_state.set(SelectState::Idle);
}

// DragParticles Systems

/// During drag, positions each overlay at its particle's origin + cursor delta.
fn update_drag_overlays(
    cursor: Res<Cursor>,
    drag_origins: Res<DragOrigins>,
    mut overlays: Query<(&SelectionOverlay, &mut Transform)>,
) {
    let delta = cursor.current.floor() - drag_origins.cursor_start.as_vec2();
    for (overlay, mut transform) in &mut overlays {
        if let Some(&origin) = drag_origins.origins.get(&overlay.0) {
            let pos = origin.as_vec2() + delta;
            transform.translation.x = pos.x + 0.5;
            transform.translation.y = pos.y + 0.5;
        }
    }
}

fn finish_select_action(
    mut commands: Commands,
    cursor: Res<Cursor>,
    mut selected_particles: ResMut<SelectedParticles>,
    mut drag_origins: ResMut<DragOrigins>,
    mut map: ResMut<ParticleMap>,
    chunk_index: Res<ChunkIndex>,
    mut chunk_query: Query<&mut ChunkDirtyState>,
    mut positions: Query<&mut GridPosition>,
    mut next_state: ResMut<NextState<SelectState>>,
    mut overlays: Query<(Entity, &SelectionOverlay, &mut Transform)>,
    select_mode: Res<State<SelectModeState>>,
    mut msgw: MessageWriter<PromoteDynamicRigidBodyParticle>,
) {
    let delta = cursor.current.floor().as_ivec2() - drag_origins.cursor_start;

    // No change means we're not dragging -- deselect the current particle.
    if delta == IVec2::ZERO {
        if let Ok(Some(clicked)) = map.get_copied(drag_origins.cursor_start) {
            selected_particles.particles.retain(|e| *e != clicked);
            commands.trigger(SyncParticleSignal::from_entity(clicked));
            for (overlay_entity, overlay, _) in &overlays {
                if overlay.0 == clicked {
                    commands.entity(overlay_entity).despawn();
                    break;
                }
            }
            mark_dirty(drag_origins.cursor_start, &chunk_index, &mut chunk_query);
        }

        for entity in &selected_particles.particles {
            commands.trigger(SyncParticleSignal::from_entity(*entity));
        }
        sync_overlays_to_positions(&mut overlays, &positions);
        drag_origins.origins.clear();
        next_state.set(SelectState::Idle);
        return;
    }

    match select_mode.get() {
        SelectModeState::Drag => {
            finish_drag(
                &mut commands,
                &mut selected_particles,
                &drag_origins,
                &mut map,
                &chunk_index,
                &mut chunk_query,
                &mut positions,
                delta,
            );
            sync_overlays_to_positions(&mut overlays, &positions);
        }
        SelectModeState::Throw => {
            let velocity = (cursor.current - cursor.previous) * THROW_VELOCITY_SCALE;
            finish_throw(
                &mut selected_particles,
                &drag_origins,
                &mut map,
                &chunk_index,
                &mut chunk_query,
                &mut positions,
                &mut msgw,
                delta,
                velocity,
            );
            for (overlay_entity, _, _) in &overlays {
                commands.entity(overlay_entity).despawn();
            }
        }
    }

    drag_origins.origins.clear();
    next_state.set(SelectState::Idle);
}

fn finish_drag(
    commands: &mut Commands,
    selected_particles: &mut SelectedParticles,
    drag_origins: &DragOrigins,
    map: &mut ParticleMap,
    chunk_index: &ChunkIndex,
    chunk_query: &mut Query<&mut ChunkDirtyState>,
    positions: &mut Query<&mut GridPosition>,
    delta: IVec2,
) {
    // Remove all selected particles from the map so they don't block each other
    for entity in &selected_particles.particles {
        if let Ok(grid_position) = positions.get(*entity) {
            let _ = map.remove(grid_position.0);
        }
    }

    // Place each particle at origin + delta if free, otherwise leave at origin
    for entity in &selected_particles.particles {
        let Some(&origin) = drag_origins.origins.get(entity) else {
            continue;
        };
        let Ok(mut grid_position) = positions.get_mut(*entity) else {
            continue;
        };
        let target_pos = origin + delta;
        let target = if map.get_copied(target_pos) == Ok(None) {
            target_pos
        } else {
            origin
        };
        let _ = map.insert(target, *entity);
        grid_position.0 = target;

        mark_dirty(origin, chunk_index, chunk_query);
        mark_dirty(target, chunk_index, chunk_query);
        commands.trigger(SyncParticleSignal::from_entity(*entity));
    }
}

fn finish_throw(
    selected_particles: &mut SelectedParticles,
    drag_origins: &DragOrigins,
    map: &mut ParticleMap,
    chunk_index: &ChunkIndex,
    chunk_query: &mut Query<&mut ChunkDirtyState>,
    positions: &mut Query<&mut GridPosition>,
    msgw: &mut MessageWriter<PromoteDynamicRigidBodyParticle>,
    delta: IVec2,
    velocity: Vec2,
) {
    for entity in &selected_particles.particles {
        if let Ok(grid_position) = positions.get(*entity) {
            let _ = map.remove(grid_position.0);
        }
    }

    for (i, entity) in selected_particles.particles.drain(..).enumerate() {
        let Some(&origin) = drag_origins.origins.get(&entity) else {
            continue;
        };
        let Ok(mut grid_position) = positions.get_mut(entity) else {
            continue;
        };
        let target = origin + delta;
        let _ = map.insert(target, entity);
        grid_position.0 = target;

        mark_dirty(origin, chunk_index, chunk_query);
        mark_dirty(target, chunk_index, chunk_query);

        let hash = (i as f32 * 1.618).sin() * 43758.5453;
        let jitter = Vec2::new(hash.fract(), (hash * 1.37).fract()) * 2.0 - Vec2::ONE;
        let v = velocity + jitter * velocity.length() * 0.15;
        msgw.write(PromoteDynamicRigidBodyParticle::new(entity).with_linear_velocity(v));
    }
}

// Continuous Systems (runs every frame while in CanvasState::Select, except during drag)

/// Keeps overlay sprite positions in sync with their tracked particle's GridPosition.
fn sync_overlay_positions(
    mut overlays: Query<(&SelectionOverlay, &mut Transform)>,
    positions: Query<&GridPosition>,
) {
    for (overlay, mut transform) in &mut overlays {
        if let Ok(grid_pos) = positions.get(overlay.0) {
            transform.translation.x = grid_pos.0.x as f32 + 0.5;
            transform.translation.y = grid_pos.0.y as f32 + 0.5;
        }
    }
}

// Cleanup Systems (OnExit CanvasState::Select)

fn cleanup_drag_state(
    mut commands: Commands,
    selected_particles: Res<SelectedParticles>,
    drag_origins: Res<DragOrigins>,
    overlays: Query<Entity, With<SelectionOverlay>>,
    mut map: ResMut<ParticleMap>,
    chunk_index: Res<ChunkIndex>,
    mut chunk_query: Query<&mut ChunkDirtyState>,
    mut positions: Query<&mut GridPosition>,
) {
    for entity in &overlays {
        commands.entity(entity).despawn();
    }

    if !drag_origins.origins.is_empty() {
        for entity in &selected_particles.particles {
            if let Ok(grid_position) = positions.get(*entity) {
                let _ = map.remove(grid_position.0);
                mark_dirty(grid_position.0, &chunk_index, &mut chunk_query);
            }
        }
        for entity in &selected_particles.particles {
            let Some(&origin) = drag_origins.origins.get(entity) else {
                continue;
            };
            if positions.get(*entity).is_err() {
                commands.entity(*entity).insert(GridPosition(origin));
            } else if let Ok(mut grid_position) = positions.get_mut(*entity) {
                grid_position.0 = origin;
            }
            let _ = map.insert(origin, *entity);
            mark_dirty(origin, &chunk_index, &mut chunk_query);
        }
    }

    for entity in &selected_particles.particles {
        commands.trigger(SyncParticleSignal::from_entity(*entity));
    }
}
