use bevy::prelude::*;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ConfigPathReadyState>()
            .add_sub_state::<ParticleTypesPathReadyState>()
            .add_sub_state::<ParticleTypesInitFileReadyState>()
            .add_sub_state::<WorldBasePathReadyState>()
            .add_sub_state::<WorldConfigReadyState>()
            .add_sub_state::<SettingsPathReadyState>()
            .add_sub_state::<ParticleTypesLoadedState>();
    }
}

#[derive(States, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum ConfigPathReadyState {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathReadyState = ConfigPathReadyState::Complete)]
pub enum ParticleTypesPathReadyState {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ParticleTypesPathReadyState = ParticleTypesPathReadyState::Complete)]
pub enum ParticleTypesInitFileReadyState {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathReadyState = ConfigPathReadyState::Complete)]
pub enum WorldBasePathReadyState {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(WorldBasePathReadyState = WorldBasePathReadyState::Complete)]
pub enum WorldConfigReadyState {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathReadyState = ConfigPathReadyState::Complete)]
pub enum SettingsPathReadyState {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathReadyState = ConfigPathReadyState::Complete)]
pub enum ParticleTypesLoadedState {
    #[default]
    Incomplete,
    Complete,
    Failed(String),
}
