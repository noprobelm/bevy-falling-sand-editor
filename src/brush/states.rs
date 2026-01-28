use bevy::prelude::*;
use leafwing_input_manager::common_conditions::action_just_pressed;

use crate::brush::BrushAction;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BrushTypeState>()
            .init_state::<BrushModeState>()
            .add_sub_state::<BrushModeSpawnState>()
            .add_systems(
                Update,
                handle_brush_mode_state.run_if(action_just_pressed(BrushAction::ToggleMode)),
            );
    }
}

#[derive(States, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BrushTypeState {
    Line,
    #[default]
    Circle,
    Cursor,
}

#[derive(States, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BrushModeState {
    #[default]
    Spawn,
    Despawn,
}

#[derive(SubStates, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[source(BrushModeState = BrushModeState::Spawn)]
pub enum BrushModeSpawnState {
    #[default]
    Particles,
    DynamicRigidBodies,
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
