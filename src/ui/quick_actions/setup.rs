use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};
use serde::{Deserialize, Serialize};

use crate::{config::SettingsConfig, setup::SetupSystems};

pub struct QuickActionsSetupPlugin;

impl Plugin for QuickActionsSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<QuickAction>::default())
            .add_systems(Startup, load_settings.in_set(SetupSystems::Ui));
    }
}

#[derive(Actionlike, Resource, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum QuickAction {
    ToggleUi,
    ToggleMapOverlay,
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct QuickActionsKeyBindings {
    pub toggle_map_overlay: KeyCode,
    pub toggle_ui: KeyCode,
}

impl Default for QuickActionsKeyBindings {
    fn default() -> Self {
        Self {
            toggle_map_overlay: KeyCode::F1,
            toggle_ui: KeyCode::KeyH,
        }
    }
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let mut input_map = InputMap::default();
    input_map.insert(
        QuickAction::ToggleMapOverlay,
        settings_config.get().ui.quick_actions.toggle_map_overlay,
    );
    input_map.insert(
        QuickAction::ToggleUi,
        settings_config.get().ui.quick_actions.toggle_ui,
    );
    commands.spawn(input_map);
}
