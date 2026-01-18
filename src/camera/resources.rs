use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::camera::ZoomSpeed;

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct CameraConfig {
    #[serde(skip)]
    pub projection: Projection,
    pub scale: f32,
    pub zoom_speed: ZoomSpeed,
    pub position: Vec2,
}

impl Default for CameraConfig {
    fn default() -> Self {
        let scale = 0.25;
        Self {
            projection: Projection::Orthographic(OrthographicProjection {
                near: -1000.0,
                scale,
                ..OrthographicProjection::default_2d()
            }),
            scale,
            zoom_speed: ZoomSpeed(8.0),
            position: Vec2::ZERO,
        }
    }
}
