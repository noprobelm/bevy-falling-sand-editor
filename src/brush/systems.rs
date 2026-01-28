use bevy::prelude::*;
use bevy_falling_sand::core::{Particle, SpawnParticleSignal};
use leafwing_input_manager::{common_conditions::action_pressed, prelude::ActionState};

use crate::{
    CursorPosition,
    brush::{BrushAction, BrushTypeState, components::BrushSize, get_interpolated_line_points},
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
            brush_draw_spawn_line
                .run_if(in_state(BrushTypeState::Line))
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

fn brush_draw_spawn_line(
    mut msgw_spawn_particle_signal: MessageWriter<SpawnParticleSignal>,
    brush_size: Single<&BrushSize>,
    cursor_position: Res<CursorPosition>,
) {
    let mut positions = vec![];
    let min_x = -((brush_size.0 as i32) / 2) * 3;
    let max_x = (brush_size.0 as i32 / 2) * 3;
    [
        (cursor_position.current, cursor_position.previous),
        (cursor_position.previous, cursor_position.previous_previous),
    ]
    .iter()
    .for_each(|(start, end)| {
        positions.extend(get_interpolated_line_points(*start, *end, min_x, max_x));
    });

    positions.iter().for_each(|pos| {
        msgw_spawn_particle_signal
            .write(SpawnParticleSignal::new(Particle::new("Dirt Wall"), *pos));
    });
}
