use bevy::prelude::*;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::ActionState};
use serde::{Deserialize, Serialize};

use crate::{
    brush::{BrushAction, CanvasStateActions},
    ui::CanvasState,
};

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BrushTypeState>()
            .init_state::<BrushSpawnState>()
            .add_sub_state::<BrushState>()
            .add_sub_state::<BrushModeSpawnState>()
            .add_systems(
                Update,
                (
                    handle_brush_state,
                    handle_brush_mode_state.run_if(action_just_pressed(BrushAction::ToggleMode)),
                ),
            );
    }
}

#[derive(
    SubStates,
    Reflect,
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[source(CanvasState = CanvasState::Brush)]
pub enum BrushState {
    #[default]
    Draw,
    Edit,
}

#[derive(
    States,
    Reflect,
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum BrushTypeState {
    Line,
    #[default]
    Circle,
    Cursor,
}

#[derive(
    States,
    Reflect,
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum BrushSpawnState {
    #[default]
    Spawn,
    Despawn,
}

#[derive(
    SubStates,
    Reflect,
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[source(BrushSpawnState = BrushSpawnState::Spawn)]
pub enum BrushModeSpawnState {
    #[default]
    Particles,
    Conway,
}

fn handle_brush_state(
    actions: Single<&ActionState<CanvasStateActions>>,
    mut state: ResMut<NextState<BrushState>>,
) -> Result {
    if actions.just_pressed(&CanvasStateActions::Modify) {
        state.set(BrushState::Edit);
    }
    if actions.just_released(&CanvasStateActions::Modify) {
        state.set(BrushState::Draw);
    }

    Ok(())
}

pub fn handle_brush_mode_state(
    brush_spawn_state: Res<State<BrushSpawnState>>,
    mut brush_spawn_state_next: ResMut<NextState<BrushSpawnState>>,
) {
    match brush_spawn_state.get() {
        BrushSpawnState::Spawn => brush_spawn_state_next.set(BrushSpawnState::Despawn),
        BrushSpawnState::Despawn => brush_spawn_state_next.set(BrushSpawnState::Spawn),
    }
}
