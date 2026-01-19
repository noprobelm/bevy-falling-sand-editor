use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

use crate::camera::ZoomSpeed;

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ConfigPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ActiveWorldPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ParticleTypesFile(pub PathBuf);

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
