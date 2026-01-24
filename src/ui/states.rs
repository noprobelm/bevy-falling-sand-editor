use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass};

pub struct UiStatePlugin;

impl Plugin for UiStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<UiState>()
            .add_systems(EguiPrimaryContextPass, handle_ui_state);
    }
}

pub fn toggle_resource<T: Resource + Default>(mut commands: Commands, resource: Option<Res<T>>) {
    if resource.is_some() {
        commands.remove_resource::<T>();
    } else {
        commands.init_resource::<T>();
    }
}

#[derive(States, Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum UiState {
    #[default]
    Canvas,
    Menu,
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
