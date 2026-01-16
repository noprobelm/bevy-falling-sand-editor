mod config;

use std::path::PathBuf;

use bevy::prelude::*;

pub use config::*;

pub struct StartupPlugin {
    pub config_path: PathBuf,
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        app.add_plugins(ConfigPlugin { config_path });
    }
}

impl Default for StartupPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self { config_path }
    }
}
