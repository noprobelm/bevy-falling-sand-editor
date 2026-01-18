mod setup;
mod signals;
mod states;

use std::path::PathBuf;

use bevy::asset::io::AssetSource;
use bevy::prelude::*;

use serde::{Deserialize, Serialize};
pub use setup::*;
pub use signals::*;
pub use states::*;

use crate::camera::ZoomSpeed;

pub struct ConfigPlugin {
    pub config_path: PathBuf,
    pub settings_path: PathBuf,
    pub world_path: PathBuf,
}

impl Default for ConfigPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self {
            settings_path: config_path.join("settings"),
            world_path: config_path.join("world"),
            config_path,
        }
    }
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        let settings_path = self.settings_path.clone();
        let world_path = self.world_path.clone();

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
            },
            StatesPlugin,
            SignalsPlugin,
        ));
        app.configure_sets(Update, SaveWorldSystems);
    }
}

/// System set for saving the world to disk.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaveWorldSystems;

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ConfigPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ActiveWorldPath(pub PathBuf);

#[derive(Resource, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct InitConfig {
    settings_init_file: PathBuf,
    particle_types_init_file: PathBuf,
    active_world_path: PathBuf,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            settings_init_file: PathBuf::from("settings.toml"),
            particle_types_init_file: PathBuf::from("particles.scn.ron"),
            active_world_path: PathBuf::from("default"),
        }
    }
}

impl InitConfig {
    pub fn active_world_path(&self) -> &PathBuf {
        &self.active_world_path
    }
}

#[derive(Resource, Clone, Default, Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub camera: CameraConfig,
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    pub scale: f32,
    pub zoom_speed: ZoomSpeed,
    pub position: Vec2,
}

impl Default for CameraConfig {
    fn default() -> Self {
        let scale = 0.25;
        Self {
            scale,
            zoom_speed: ZoomSpeed(8.0),
            position: Vec2::ZERO,
        }
    }
}

impl CameraConfig {
    pub fn from_query(query: &Single<(&Transform, &ZoomSpeed, &Projection)>) -> Self {
        let scale = match query.2 {
            Projection::Orthographic(ortho) => ortho.scale,
            _ => unreachable!(),
        };
        let position = Vec2::new(query.0.translation.x, query.0.translation.y);
        Self {
            scale,
            zoom_speed: query.1.clone(),
            position,
        }
    }
}
