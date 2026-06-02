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

pub use components::*;
pub use debug::DebugEarthquake;
pub use states::*;

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
pub struct EarthquakeOptions<'w, 's> {
    pub size: Single<'w, 's, &'static mut EarthquakeBrushSize>,
    pub current_region_state: Res<'w, State<EarthquakeRegionState>>,
    pub next_region_state: ResMut<'w, NextState<EarthquakeRegionState>>,
    pub debug: Option<Res<'w, DebugEarthquake>>,
}
