use bevy::prelude::*;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<ConfigPathState>()
            .add_sub_state::<InitConfigState>()
            .add_sub_state::<WorldConfigState>()
            .add_sub_state::<ParticleTypesConfigState>()
            .init_state::<CameraInitState>();
    }
}

#[derive(States, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum ConfigPathState {
    #[default]
    Initializing,
    Initialized,
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathState = ConfigPathState::Initialized)]
pub enum InitConfigState {
    #[default]
    Initializing,
    Initialized,
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(InitConfigState = InitConfigState::Initialized)]
pub enum WorldConfigState {
    #[default]
    Initializing,
    Initialized,
}

#[derive(SubStates, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
#[source(ConfigPathState = ConfigPathState::Initialized)]
pub enum ParticleTypesConfigState {
    #[default]
    Initializing,
    Initialized,
}

#[derive(States, Clone, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub enum CameraInitState {
    #[default]
    Initializing,
    Initialized,
}
