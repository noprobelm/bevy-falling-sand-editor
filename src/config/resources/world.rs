use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::camera::ZoomSpeed;

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
