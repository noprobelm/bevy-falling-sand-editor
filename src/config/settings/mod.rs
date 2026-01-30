mod resources;
mod save;

use bevy::prelude::*;

pub use resources::*;
pub use save::SaveSettingsEvent;

pub struct SettingsConfigPlugin;

impl Plugin for SettingsConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(save::SavePlugin);
    }
}
