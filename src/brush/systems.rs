use bevy::prelude::*;

use crate::brush::BrushModeState;

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, handle_brush_mode_state);
    }
}

pub fn handle_brush_mode_state(
    brush_spawn_state: Res<State<BrushModeState>>,
    mut brush_spawn_state_next: ResMut<NextState<BrushModeState>>,
) {
    match brush_spawn_state.get() {
        BrushModeState::Spawn => brush_spawn_state_next.set(BrushModeState::Despawn),
        BrushModeState::Despawn => brush_spawn_state_next.set(BrushModeState::Spawn),
    }
}
