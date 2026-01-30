use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{brush::BrushKeyBindings, camera::CameraKeyBindings, ui::UiKeyBindings};

#[derive(Resource, Clone, Default, Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub camera: CameraKeyBindings,
    pub ui: UiKeyBindings,
    pub brush: BrushKeyBindings,
}
