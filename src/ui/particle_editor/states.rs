use bevy::prelude::*;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ParticleEditorState>();
    }
}

#[derive(States, Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ParticleEditorState {
    #[default]
    Closed,
    Open,
}
