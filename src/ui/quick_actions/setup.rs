use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};
use serde::{Deserialize, Serialize};

use crate::{config::SettingsConfig, setup::SetupSystems};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<QuickAction>::default())
            .add_systems(Startup, load_settings.in_set(SetupSystems::Ui));
    }
}

#[derive(Actionlike, Resource, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum QuickAction {
    ToggleUi,
    ToggleMapOverlay,
    ToggleDirtyChunksOverlay,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuickActionsKeyBindings {
    pub toggle_ui: KeyCode,
    pub toggle_map_overlay: KeyCode,
    pub toggle_dirty_chunks_overlay: KeyCode,
}

impl Default for QuickActionsKeyBindings {
    fn default() -> Self {
        Self {
            toggle_ui: KeyCode::KeyH,
            toggle_map_overlay: KeyCode::F1,
            toggle_dirty_chunks_overlay: KeyCode::F2,
        }
    }
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let input_map = InputMap::default()
        .with(
            QuickAction::ToggleUi,
            settings_config.get().ui.quick_actions.toggle_ui,
        )
        .with(
            QuickAction::ToggleMapOverlay,
            settings_config.get().ui.quick_actions.toggle_map_overlay,
        )
        .with(
            QuickAction::ToggleDirtyChunksOverlay,
            settings_config
                .get()
                .ui
                .quick_actions
                .toggle_dirty_chunks_overlay,
        );
    commands.spawn(input_map);
}
