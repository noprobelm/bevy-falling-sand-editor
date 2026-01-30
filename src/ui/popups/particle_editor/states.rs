use bevy::prelude::*;

use crate::ui::PopupState;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PopupState<ParticleEditorApplicationState>>();
    }
}

#[derive(Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ParticleEditorApplicationState {
    #[default]
    Closed,
    Open,
}
