use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};
use leafwing_input_manager::prelude::ActionState;

use crate::ui::CanvasStateActions;

pub struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UiState>()
            .add_sub_state::<CanvasState>()
            .add_systems(Update, handle_canvas_state)
            .add_systems(EguiPrimaryContextPass, handle_ui_state);
    }
}

#[derive(States, Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum UiState {
    #[default]
    Canvas,
    Menu,
}

#[derive(SubStates, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[source(UiState = UiState::Canvas)]
pub enum CanvasState {
    #[default]
    Interact,
    Edit,
}

fn handle_canvas_state(
    actions: Single<&ActionState<CanvasStateActions>>,
    mut state: ResMut<NextState<CanvasState>>,
) -> Result {
    if actions.just_pressed(&CanvasStateActions::Modify) {
        state.set(CanvasState::Edit);
    }
    if actions.just_released(&CanvasStateActions::Modify) {
        state.set(CanvasState::Interact);
    }

    Ok(())
}

fn handle_ui_state(
    mut contexts: EguiContexts,
    current_state: Res<State<UiState>>,
    mut next_state: ResMut<NextState<UiState>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let is_using_pointer = ctx.is_using_pointer();
    let wants_keyboard_input = ctx.wants_keyboard_input();

    let should_be_ui = is_using_pointer || wants_keyboard_input;

    match current_state.get() {
        UiState::Canvas => {
            if should_be_ui {
                next_state.set(UiState::Menu);
            }
        }
        UiState::Menu => {
            if !should_be_ui {
                next_state.set(UiState::Canvas);
            }
        }
    }

    Ok(())
}
