mod resources;
mod setup;
mod signals;

use std::path::PathBuf;

use bevy::asset::io::AssetSource;
use bevy::prelude::*;

pub use resources::*;
pub use setup::*;
pub use signals::*;

pub struct ConfigPlugin {
    pub config_path: PathBuf,
}

impl Default for ConfigPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self { config_path }
    }
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        let config_path_str = config_path.to_string_lossy().to_string();
        app.register_asset_source(
            "config",
            AssetSource::build()
                .with_reader(move || AssetSource::get_default_reader(config_path_str.clone())()),
        );
        app.add_plugins((SetupPlugin { config_path }, SignalsPlugin));
    }
}
