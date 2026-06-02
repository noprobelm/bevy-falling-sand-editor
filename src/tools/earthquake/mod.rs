mod components;
pub mod debug;
mod gizmos;
mod setup;
mod signals;
mod states;
mod systems;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
pub use signals::*;

use crate::tools::earthquake::debug::DebugEarthquake;

pub(super) struct EarthquakePlugin;

impl Plugin for EarthquakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debug::DebugPlugin,
            gizmos::GizmosPlugin,
            setup::SetupPlugin,
            signals::SignalsPlugin,
            states::StatesPlugin,
            systems::SystemsPlugin,
        ));
    }
}

#[derive(SystemParam)]
pub struct EarthquakeOptions<'w> {
    pub debug: Option<Res<'w, DebugEarthquake>>,
}
