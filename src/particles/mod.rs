mod setup;

use bevy::prelude::*;
pub use setup::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SetupPlugin);
    }
}
