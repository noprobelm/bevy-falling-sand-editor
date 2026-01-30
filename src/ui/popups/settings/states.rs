use bevy::prelude::*;

use crate::ui::PopupState;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PopupState<SettingsApplicationState>>();
    }
}

#[derive(Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum SettingsApplicationState {
    #[default]
    Closed,
    Open,
}
