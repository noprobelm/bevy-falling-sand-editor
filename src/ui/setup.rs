use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};
use serde::{Deserialize, Serialize};

use crate::{
    config::SettingsConfig,
    setup::SetupSystems,
    ui::{
        ConsoleKeyBindings, ConsoleSetupPlugin, QuickActionsKeyBindings, QuickActionsSetupPlugin,
    },
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ConsoleSetupPlugin,
            QuickActionsSetupPlugin,
            InputManagerPlugin::<CanvasStateActions>::default(),
        ))
        .add_systems(Startup, load_settings.in_set(SetupSystems::Ui));
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
    hold_canvas_mode_edit: KeyCode,
}

impl Default for GeneralKeyBindings {
    fn default() -> Self {
        Self {
            hold_canvas_mode_edit: KeyCode::AltLeft,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CanvasStateActions {
    Modify,
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let input_map = InputMap::default().with(
        CanvasStateActions::Modify,
        settings_config.get().ui.general.hold_canvas_mode_edit,
    );
    commands.spawn(input_map);
}
