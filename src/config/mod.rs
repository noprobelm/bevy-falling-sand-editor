mod resources;
pub mod setup;
mod signals;

use bevy::prelude::*;

pub use resources::*;
pub use setup::*;
pub use signals::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SignalsPlugin);
    }
}
