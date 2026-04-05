use bevy::prelude::*;
use bevy_egui::EguiContextSettings;
use serde::{Deserialize, Serialize};

use crate::{
    config::InputButton,
    ui::{ConsoleKeyBindings, QuickActionsKeyBindings},
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, set_default_ui_scale.run_if(run_once));
    }
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct UiKeyBindings {
    pub console: ConsoleKeyBindings,
    pub quick_actions: QuickActionsKeyBindings,
    pub general: GeneralKeyBindings,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GeneralKeyBindings {
    pub hold_canvas_mode_edit: InputButton,
}

impl Default for GeneralKeyBindings {
    fn default() -> Self {
        Self {
            hold_canvas_mode_edit: KeyCode::AltLeft.into(),
        }
    }
}

fn set_default_ui_scale(mut egui_settings: Single<&mut EguiContextSettings>) {
    egui_settings.scale_factor = 1.25;
}
