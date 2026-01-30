mod base;
mod settings;
pub mod setup;
mod world;

use std::path::PathBuf;

use bevy::prelude::*;

pub use base::*;
pub use settings::*;
pub use setup::*;
pub use world::*;

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
        app.add_plugins((
            ConfigSetupPlugin {
                config_path: self.config_path.clone(),
            },
            WorldConfigPlugin,
            SettingsConfigPlugin,
        ));
    }
}
