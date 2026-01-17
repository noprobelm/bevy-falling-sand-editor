mod systems;

use bevy::prelude::*;

pub use systems::*;

pub(super) struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SystemsPlugin);
    }
}
