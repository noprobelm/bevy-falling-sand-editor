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
