mod save;

use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub use save::WorldSaveBuilder;

use crate::camera::ZoomSpeed;

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

pub struct WorldConfigPlugin;

impl Plugin for WorldConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(save::SavePlugin);
    }
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct WorldConfig {
    pub camera: CameraConfig,
    pub particle_types_file: PathBuf,
}

impl Default for WorldConfig {
    fn default() -> Self {
        Self {
            camera: CameraConfig::default(),
            particle_types_file: PathBuf::from("particles.scn.ron"),
        }
    }
}
