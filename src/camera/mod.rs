mod components;
mod save;
pub mod setup;
mod systems;

use bevy::prelude::*;

pub use components::*;
use save::SavePlugin;
pub use setup::*;
pub use systems::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, SystemsPlugin, SavePlugin));
    }
}
