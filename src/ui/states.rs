use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};

pub struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UiState>()
            .add_sub_state::<CanvasState>()
            .init_resource::<PreviousCanvasState>()
            .add_observer(on_set_canvas_state)
            .add_systems(OnEnter(UiState::Canvas), apply_pending_canvas_state)
            .add_systems(EguiPrimaryContextPass, handle_ui_state);
    }
}

#[derive(Resource, Default)]
pub struct PreviousCanvasState(pub CanvasState);

#[derive(States, Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum UiState {
    #[default]
    Canvas,
    Menu,
}

#[derive(SubStates, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[source(UiState = UiState::Canvas)]
pub enum CanvasState {
    Select,
    #[default]
    Brush,
}

fn handle_ui_state(
    mut contexts: EguiContexts,
    current_ui_state: Res<State<UiState>>,
    mut next_ui_state: ResMut<NextState<UiState>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let is_pointer_over_area = ctx.is_pointer_over_area();
    let is_using_pointer = ctx.is_using_pointer();
    let wants_keyboard_input = ctx.wants_keyboard_input();

    let should_be_ui = is_using_pointer || wants_keyboard_input || is_pointer_over_area;

    match current_ui_state.get() {
        UiState::Canvas => {
            if should_be_ui {
                next_ui_state.set(UiState::Menu);
            }
        }
        UiState::Menu => {
            if !should_be_ui {
                next_ui_state.set(UiState::Canvas);
            }
        }
    }

    Ok(())
}

#[derive(Event)]
pub struct SetCanvasStateEvent(pub CanvasState);

fn on_set_canvas_state(
    trigger: On<SetCanvasStateEvent>,
    mut pending: ResMut<PreviousCanvasState>,
    mut state: ResMut<NextState<CanvasState>>,
) {
    let desired = trigger.event().0;
    pending.0 = desired;
    state.set(desired);
}

fn apply_pending_canvas_state(
    pending: Res<PreviousCanvasState>,
    mut state: ResMut<NextState<CanvasState>>,
) {
    state.set(pending.0);
}
