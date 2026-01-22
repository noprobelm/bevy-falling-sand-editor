mod resources;
mod signals;

use bevy::prelude::*;

pub use resources::*;
pub use signals::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SignalsPlugin));
    }
}
