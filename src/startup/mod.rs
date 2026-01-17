mod camera;
mod config;
mod particle_types;

use std::path::PathBuf;

use bevy::prelude::*;

pub use camera::*;
pub use config::*;
use particle_types::*;

pub struct StartupPlugin {
    pub config_path: PathBuf,
    pub settings_config_path: PathBuf,
    pub world_config_path: PathBuf,
    pub particle_types_config_path: PathBuf,
    pub particle_types_init_file: PathBuf,
}

impl Plugin for StartupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ConfigStartupPlugin {
                config_path: self.config_path.clone(),
                settings_path: self.settings_config_path.clone(),
                world_path: self.world_config_path.clone(),
                particle_types_path: self.particle_types_config_path.clone(),
            },
            ParticleTypeStartupPlugin {
                particle_types_init_file: self.particle_types_init_file.clone(),
            },
            CameraSetupPlugin,
        ));
    }
}

impl Default for StartupPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        let settings_config_path = config_path.join("settings");
        let world_config_path = config_path.join("world");
        let particle_types_config_path = config_path.join("particles");
        let particle_types_init_file = particle_types_config_path.join("particles.scn.ron");

        Self {
            config_path,
            settings_config_path,
            world_config_path,
            particle_types_config_path,
            particle_types_init_file,
        }
    }
}
