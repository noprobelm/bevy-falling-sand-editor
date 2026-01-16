use bevy::prelude::*;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ConfigPathLoadedState>();
    }
}

#[derive(States, Copy, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum ConfigPathLoadedState {
    #[default]
    Incomplete,
    Complete,
}

#[derive(SubStates, Copy, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathLoadedState = ConfigPathLoadedState::Complete)]
pub enum SettingsLoadedState {
    #[default]
    Incomplete,
    Complete,
}

#[derive(SubStates, Copy, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathLoadedState = ConfigPathLoadedState::Complete)]
pub enum WorldLoadedState {
    #[default]
    Incomplete,
    Complete,
}

#[derive(States, Copy, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum ConfigLoadedState {
    #[default]
    Incomplete,
    Complete,
}
