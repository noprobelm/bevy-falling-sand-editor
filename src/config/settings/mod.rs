mod save;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::{camera::CameraKeyBindings, ui::UiKeyBindings};

pub struct SettingsConfigPlugin;

impl Plugin for SettingsConfigPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(save::SavePlugin);
    }
}

#[derive(Resource, Clone, Default, Debug, Serialize, Deserialize)]
pub struct SettingsConfig {
    pub camera: CameraKeyBindings,
    pub ui: UiKeyBindings,
}
