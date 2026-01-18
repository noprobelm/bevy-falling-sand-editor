mod resources;
mod setup;
mod states;

use std::path::PathBuf;

use bevy::asset::io::AssetSource;
use bevy::prelude::*;

pub use resources::*;
pub use setup::*;
pub use states::*;

pub struct ConfigPlugin {
    pub config_path: PathBuf,
    pub settings_path: PathBuf,
    pub world_path: PathBuf,
    pub particle_types_path: PathBuf,
}

impl Default for ConfigPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self {
            settings_path: config_path.join("settings"),
            world_path: config_path.join("world"),
            particle_types_path: config_path.join("particles"),
            config_path,
        }
    }
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        let settings_path = self.settings_path.clone();
        let world_path = self.world_path.clone();
        let particle_types_path = self.particle_types_path.clone();

        let config_path_str = config_path.to_string_lossy().to_string();
        app.register_asset_source(
            "config",
            AssetSource::build()
                .with_reader(move || AssetSource::get_default_reader(config_path_str.clone())()),
        );

        app.add_plugins((
            SetupPlugin {
                config_path,
                settings_path,
                world_path,
                particle_types_path,
            },
            StatesPlugin,
        ));
    }
}
