use bevy::prelude::*;
use bevy_falling_sand::core::{
    DespawnParticleSignal, ParticleType, ParticleTypeRegistry, SpawnParticleSignal,
};
use leafwing_input_manager::{common_conditions::action_pressed, prelude::ActionState};

use crate::{
    Cursor,
    canvas::brush::{
        BrushAction, BrushModeSpawnState, BrushSpawnState, BrushState, BrushTypeState,
        components::{BrushSize, SelectedParticle, SelectedParticleType},
    },
};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                sync_selected_particle_type,
                sync_selected_particle_name.after(sync_selected_particle_type),
            ),
        )
        .add_systems(Update, resize_brush.run_if(in_state(BrushState::Edit)))
        .add_systems(
            Update,
            (
                brush_action_spawn_particles
                    .run_if(action_pressed(BrushAction::Draw))
                    .run_if(in_state(BrushState::Draw))
                    .run_if(in_state(BrushModeSpawnState::Particles)),
                brush_action_despawn_particles
                    .run_if(action_pressed(BrushAction::Draw))
                    .run_if(in_state(BrushState::Draw))
                    .run_if(in_state(BrushSpawnState::Despawn)),
            ),
        );
    }
}

/// Keeps [`SelectedParticleType`] in sync when [`SelectedParticle`] changes
/// (e.g. user picks a different particle in the editor or samples one from the canvas).
fn sync_selected_particle_type(
    mut brush_query: Query<
        (&SelectedParticle, &mut SelectedParticleType),
        Changed<SelectedParticle>,
    >,
    registry: Res<ParticleTypeRegistry>,
) {
    for (selected, mut tracked) in &mut brush_query {
        if let Some(&entity) = registry.get(&selected.0.name) {
            if tracked.0 != entity {
                tracked.0 = entity;
            }
        }
    }
}

/// Updates the brush's [`SelectedParticle`] name when the tracked [`ParticleType`] is renamed.
fn sync_selected_particle_name(
    mut brush_query: Query<(&mut SelectedParticle, &SelectedParticleType)>,
    changed_types: Query<&ParticleType, Changed<ParticleType>>,
) {
    for (mut selected, tracked) in &mut brush_query {
        if let Ok(particle_type) = changed_types.get(tracked.0) {
            if selected.0.name != particle_type.name {
                selected.0.name = particle_type.name.clone();
            }
        }
    }
}

fn resize_brush(mut single: Single<(&ActionState<BrushAction>, &mut BrushSize)>) {
    let (action_state, brush_size) = (single.0, &mut single.1);
    let delta = action_state.value(&BrushAction::ChangeSize);
    if delta > 0.0 {
        brush_size.0 = brush_size.0.saturating_add(1);
    } else if delta < 0.0 {
        brush_size.0 = brush_size.0.saturating_sub(1).max(1);
    }
}

fn brush_action_spawn_particles(
    mut msgw_spawn: MessageWriter<SpawnParticleSignal>,
    brush: Single<(&BrushSize, &SelectedParticle)>,
    cursor: Res<Cursor>,
    brush_type: Res<State<BrushTypeState>>,
) {
    alg::get_positions(
        cursor.current,
        cursor.previous,
        cursor.previous_previous,
        brush.0.0 as f32,
        &brush_type,
    )
    .iter()
    .for_each(|pos| {
        msgw_spawn.write(SpawnParticleSignal::new(brush.1.0.clone(), *pos));
    });
}

fn brush_action_despawn_particles(
    mut msgw_despawn: MessageWriter<DespawnParticleSignal>,
    brush_size: Single<&BrushSize>,
    cursor: Res<Cursor>,
    brush_type: Res<State<BrushTypeState>>,
) {
    alg::get_positions(
        cursor.current,
        cursor.previous,
        cursor.previous_previous,
        brush_size.0 as f32,
        &brush_type,
    )
    .iter()
    .for_each(|pos| {
        msgw_despawn.write(DespawnParticleSignal::from_position(*pos));
    });
}

pub mod alg {
    use bevy::prelude::*;

    use crate::canvas::brush::BrushTypeState;

    pub fn get_positions(
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
        brush_size: f32,
        brush_type: &BrushTypeState,
    ) -> Vec<IVec2> {
        let cursor_pairs = [(p1, p2), (p2, p3)];

        cursor_pairs
            .iter()
            .flat_map(|(start, end)| match brush_type {
                BrushTypeState::Circle => get_interpolated_circle_points(*start, *end, brush_size),
                BrushTypeState::Line => get_interpolated_line_points(*start, *end, brush_size),
                BrushTypeState::Cursor => get_interpolated_cursor_points(*start, *end),
            })
            .collect()
    }

    /// Find all horizontal lines interpolated between a start and end position.
    fn get_interpolated_line_points(start: Vec2, end: Vec2, line_length: f32) -> Vec<IVec2> {
        let mut positions = vec![];

        let min_x = -((line_length as i32) / 2) * 3;
        let max_x = (line_length as i32 / 2) * 3;

        let direction = (end - start).normalize();
        let length = (end - start).length();
        let num_samples = (length.ceil() as usize).max(1);

        for i in 0..=num_samples {
            let t = i as f32 / num_samples as f32;
            let sample_point = start + direction * length * t;

            for x_offset in min_x..=max_x {
                let position = IVec2::new(
                    (sample_point.x + x_offset as f32).floor() as i32,
                    sample_point.y.floor() as i32,
                );
                positions.push(position);
            }
        }

        positions
    }

    /// Find all cursor points interpolated between a start and end position.
    fn get_interpolated_cursor_points(start: Vec2, end: Vec2) -> Vec<IVec2> {
        if start == end {
            return vec![start.as_ivec2()];
        }

        let mut positions = vec![];
        let direction = (end - start).normalize();
        let length = (end - start).length();
        let num_samples = (length.ceil() as usize).max(1);

        for i in 0..=num_samples {
            let t = i as f32 / num_samples as f32;
            positions.push((start + direction * length * t).as_ivec2());
        }
        positions
    }

    /// Find all circles interpolated between a start and end position.
    fn get_interpolated_circle_points(start: Vec2, end: Vec2, radius: f32) -> Vec<IVec2> {
        let mut positions = vec![];
        if start == end {
            let min_x = (start.x - radius).floor() as i32;
            let max_x = (start.x + radius).ceil() as i32;
            let min_y = (start.y - radius).floor() as i32;
            let max_y = (start.y + radius).ceil() as i32;
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let pos = Vec2::new(x as f32, y as f32);
                    if (pos - start).length() <= radius {
                        positions.push(pos.as_ivec2());
                    }
                }
            }
            return positions;
        } else {
            let length = (end - start).length();
            let direction = (end - start).normalize();

            let min_x = (start.x.min(end.x) - radius).floor() as i32;
            let max_x = (start.x.max(end.x) + radius).ceil() as i32;
            let min_y = (start.y.min(end.y) - radius).floor() as i32;
            let max_y = (start.y.max(end.y) + radius).ceil() as i32;

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let point = Vec2::new(x as f32, y as f32);

                    let to_point = point - start;
                    let projected_length = to_point.dot(direction);
                    // Sometimes projected length will exceed the actual length of the capsule, so
                    // we need to clamp it.
                    let clamped_length = projected_length.clamp(0.0, length);

                    let closest_point = start + direction * clamped_length;
                    let distance_to_line = (point - closest_point).length();

                    if distance_to_line <= radius {
                        positions.push(IVec2::new(x, y));
                    }
                }
            }
        }

        positions
    }
}
