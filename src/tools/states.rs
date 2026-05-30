use bevy::prelude::*;

use crate::ui::UiState;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SelectedTool>()
            .init_resource::<PreviousSelectedTool>();
    }
}

#[derive(SubStates, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[source(UiState = UiState::Canvas)]
pub enum SelectedTool {
    Select,
    #[default]
    Brush,
}

#[derive(Resource, Default)]
pub struct PreviousSelectedTool(pub SelectedTool);
