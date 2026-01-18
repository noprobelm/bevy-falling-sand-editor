mod components;
mod resources;
mod setup;
mod signals;
mod states;
mod systems;

use bevy::prelude::*;

pub use components::*;
pub use resources::*;
use setup::*;
pub use signals::*;
pub use states::*;
pub use systems::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ComponentsPlugin,
            SystemsPlugin,
            SetupPlugin,
            SignalsPlugin,
            StatesPlugin,
        ));
    }
}
