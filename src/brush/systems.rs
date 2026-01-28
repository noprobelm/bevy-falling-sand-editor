use bevy::prelude::*;
use bevy_falling_sand::core::{Particle, SpawnParticleSignal};
use leafwing_input_manager::{common_conditions::action_pressed, prelude::ActionState};

use crate::{
    CursorPosition,
    brush::{BrushAction, BrushTypeState, components::BrushSize},
    ui::CanvasState,
};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            change_brush_size.run_if(in_state(CanvasState::Edit)),
        )
        .add_systems(
            Update,
            (
                brush_action_circle.run_if(in_state(BrushTypeState::Circle)),
                brush_action_line.run_if(in_state(BrushTypeState::Line)),
                brush_action_cursor.run_if(in_state(BrushTypeState::Cursor)),
            )
                .run_if(action_pressed(BrushAction::Draw)),
        );
    }
}

fn change_brush_size(mut single: Single<(&ActionState<BrushAction>, &mut BrushSize)>) {
    let (action_state, brush_size) = (single.0, &mut single.1);
    let delta = action_state.value(&BrushAction::ChangeSize);
    if delta > 0.0 {
        brush_size.0 = brush_size.0.saturating_add(1);
    } else if delta < 0.0 {
        brush_size.0 = brush_size.0.saturating_sub(1).max(1);
    }
}

fn brush_action_circle(
    mut msgw_spawn_particle_signal: MessageWriter<SpawnParticleSignal>,
    brush_size: Single<&BrushSize>,
    cursor_position: Res<CursorPosition>,
) {
    let mut positions = vec![];
    [
        (cursor_position.current, cursor_position.previous),
        (cursor_position.previous, cursor_position.previous_previous),
    ]
    .iter()
    .for_each(|(start, end)| {
        positions.extend(alg::get_interpolated_circle_points(
            *start,
            *end,
            brush_size.0 as f32,
        ));
    });

    positions.iter().for_each(|pos| {
        msgw_spawn_particle_signal
            .write(SpawnParticleSignal::new(Particle::new("Dirt Wall"), *pos));
    });
}

fn brush_action_line(
    mut msgw_spawn_particle_signal: MessageWriter<SpawnParticleSignal>,
    brush_size: Single<&BrushSize>,
    cursor_position: Res<CursorPosition>,
) {
    let mut positions = vec![];
    [
        (cursor_position.current, cursor_position.previous),
        (cursor_position.previous, cursor_position.previous_previous),
    ]
    .iter()
    .for_each(|(start, end)| {
        positions.extend(alg::get_interpolated_line_points(
            *start,
            *end,
            brush_size.0 as f32,
        ));
    });

    positions.iter().for_each(|pos| {
        msgw_spawn_particle_signal
            .write(SpawnParticleSignal::new(Particle::new("Dirt Wall"), *pos));
    });
}

fn brush_action_cursor(
    mut msgw_spawn_particle_signal: MessageWriter<SpawnParticleSignal>,
    cursor_position: Res<CursorPosition>,
) {
    let mut positions = vec![];
    [
        (cursor_position.current, cursor_position.previous),
        (cursor_position.previous, cursor_position.previous_previous),
    ]
    .iter()
    .for_each(|(start, end)| {
        positions.extend(alg::get_interpolated_cursor_points(*start, *end));
    });

    positions.iter().for_each(|pos| {
        msgw_spawn_particle_signal
            .write(SpawnParticleSignal::new(Particle::new("Dirt Wall"), *pos));
    });
}

pub(super) mod alg {
    use bevy::prelude::*;
    /// Find all horizontal lines interpolated between a start and end position.
    pub(super) fn get_interpolated_line_points(
        start: Vec2,
        end: Vec2,
        line_length: f32,
    ) -> Vec<IVec2> {
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
    pub(super) fn get_interpolated_cursor_points(start: Vec2, end: Vec2) -> Vec<IVec2> {
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
    pub(super) fn get_interpolated_circle_points(
        start: Vec2,
        end: Vec2,
        radius: f32,
    ) -> Vec<IVec2> {
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
                    // clamp it.
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
