use bevy::prelude::*;

use crate::{
    tools::{PreviousSelectedTool, SelectedTool},
    ui::UiState,
};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_selected_tool)
            .add_systems(OnEnter(UiState::Canvas), apply_pending_selected_tool);
    }
}

#[derive(Event)]
pub struct SetSelectedToolEvent(pub SelectedTool);

fn on_set_selected_tool(
    trigger: On<SetSelectedToolEvent>,
    mut pending: ResMut<PreviousSelectedTool>,
    mut state: ResMut<NextState<SelectedTool>>,
) {
    let desired = trigger.event().0;
    pending.0 = desired;
    state.set(desired);
}

fn apply_pending_selected_tool(
    pending: Res<PreviousSelectedTool>,
    mut state: ResMut<NextState<SelectedTool>>,
) {
    state.set(pending.0);
}
