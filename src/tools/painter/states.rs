use bevy::prelude::*;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::ActionState};
use serde::{Deserialize, Serialize};

use crate::tools::{SelectedTool, ToolStateActions, painter::PainterAction};

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PainterShape>()
            .init_state::<PainterSpawnState>()
            .add_sub_state::<PainterBrushState>()
            .add_sub_state::<PainterModeState>()
            .add_systems(
                Update,
                (
                    handle_brush_state.run_if(in_state(SelectedTool::Painter)),
                    handle_brush_mode_state.run_if(action_just_pressed(PainterAction::ToggleMode)),
                ),
            )
            .add_observer(on_set_painter_shape)
            .add_observer(on_set_painter_mode);
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
#[source(SelectedTool = SelectedTool::Painter)]
pub enum PainterBrushState {
    #[default]
    Draw,
    Resize,
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
pub enum PainterShape {
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
pub enum PainterSpawnState {
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
#[source(PainterSpawnState = PainterSpawnState::Spawn)]
pub enum PainterModeState {
    #[default]
    Particles,
    Conway,
}

#[derive(Event, Message, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct SetPainterShape(pub PainterShape);

#[derive(Event, Message, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct SetPainterMode(pub PainterModeState);

fn handle_brush_state(
    actions: Single<&ActionState<ToolStateActions>>,
    mut state: ResMut<NextState<PainterBrushState>>,
) -> Result {
    if actions.just_pressed(&ToolStateActions::Resize) {
        state.set(PainterBrushState::Resize);
    }
    if actions.just_released(&ToolStateActions::Resize) {
        state.set(PainterBrushState::Draw);
    }

    Ok(())
}

pub fn handle_brush_mode_state(
    brush_spawn_state: Res<State<PainterSpawnState>>,
    mut brush_spawn_state_next: ResMut<NextState<PainterSpawnState>>,
) {
    match brush_spawn_state.get() {
        PainterSpawnState::Spawn => brush_spawn_state_next.set(PainterSpawnState::Despawn),
        PainterSpawnState::Despawn => brush_spawn_state_next.set(PainterSpawnState::Spawn),
    }
}

fn on_set_painter_shape(
    trigger: On<SetPainterShape>,
    mut next_shape: ResMut<NextState<PainterShape>>,
) {
    next_shape.set(trigger.event().0);
}

fn on_set_painter_mode(
    trigger: On<SetPainterMode>,
    mut next_mode: ResMut<NextState<PainterModeState>>,
) {
    next_mode.set(trigger.event().0);
}
