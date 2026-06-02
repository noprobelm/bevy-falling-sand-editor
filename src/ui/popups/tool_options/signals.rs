use bevy::prelude::*;
use crate::ui::{ PopupState, ToolOptionsWindowState};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_toggle_tool_options);
    }
}

#[derive(Event)]
pub struct UiToggleToolOptionsEvent;

fn on_toggle_tool_options(
    _trigger: On<UiToggleToolOptionsEvent>,
    current_tool_options_state: Res<State<PopupState<ToolOptionsWindowState>>>,
    mut next_tool_options_state: ResMut<NextState<PopupState<ToolOptionsWindowState>>>,
) {
    next_tool_options_state.set(current_tool_options_state.get_next());
}
