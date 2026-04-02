mod setup;
mod systems;

use bevy::prelude::*;

pub use setup::*;
use systems::*;

pub struct QuickActionsPlugin;

impl Plugin for QuickActionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, SystemsPlugin));
    }
}
