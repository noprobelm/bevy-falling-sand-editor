use bevy::prelude::*;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<CameraConfigFileLoaded>();
    }
}

#[derive(States, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum CameraConfigFileLoaded {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}
