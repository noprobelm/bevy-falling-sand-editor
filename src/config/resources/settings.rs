use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Default, Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub camera: CameraKeyBindings,
    pub console: ConsoleKeyBindings,
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct ConsoleKeyBindings {
    pub toggle_information_area: KeyCode,
}

impl Default for ConsoleKeyBindings {
    fn default() -> Self {
        Self {
            toggle_information_area: KeyCode::Backquote,
        }
    }
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct CameraKeyBindings {
    pub pan_camera_up: KeyCode,
    pub pan_camera_left: KeyCode,
    pub pan_camera_down: KeyCode,
    pub pan_camera_right: KeyCode,
}

impl Default for CameraKeyBindings {
    fn default() -> Self {
        CameraKeyBindings {
            pan_camera_up: KeyCode::KeyW,
            pan_camera_left: KeyCode::KeyA,
            pan_camera_down: KeyCode::KeyS,
            pan_camera_right: KeyCode::KeyD,
        }
    }
}
