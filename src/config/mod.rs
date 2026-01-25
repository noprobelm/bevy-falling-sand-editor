mod base;
mod save;
mod settings;
pub mod setup;
mod world;

use bevy::prelude::*;

pub use base::*;
pub use save::*;
pub use settings::*;
pub use setup::*;
pub use world::*;

pub struct ConfigPlugin;

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((save::SavePlugin, WorldConfigPlugin, SettingsConfigPlugin));
    }
}
