use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};
use serde::{Deserialize, Serialize};

use crate::{
    config::SettingsConfig,
    directive::DirectiveRegistry,
    setup::SetupSystems,
    ui::{ConsoleInformationAreaState, ConsolePromptState, ExitDirective, ParticlesDirective},
};

use super::{ConsoleCache, HelpDirective};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ConsoleAction>::default())
            .init_resource::<ConsoleCache>()
            .init_resource::<ConsoleInformationAreaState>()
            .init_resource::<ConsolePromptState>()
            .add_systems(
                Startup,
                (load_settings, setup_directive_registry).in_set(SetupSystems::Ui),
            );
    }
}

#[derive(Actionlike, Resource, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ConsoleAction {
    ToggleInformationArea,
    SubmitInputText,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ConsoleKeyBindings {
    pub toggle_information_area: KeyCode,
    #[serde(skip, default = "ConsoleKeyBindings::default_submit_input_text")]
    pub submit_input_text: KeyCode,
}

impl ConsoleKeyBindings {
    pub const fn default_submit_input_text() -> KeyCode {
        KeyCode::Enter
    }
}

impl Default for ConsoleKeyBindings {
    fn default() -> Self {
        Self {
            toggle_information_area: KeyCode::Backquote,
            submit_input_text: KeyCode::Enter,
        }
    }
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let mut input_map = InputMap::default();
    input_map.insert(
        ConsoleAction::ToggleInformationArea,
        settings_config.get().ui.console.toggle_information_area,
    );
    input_map.insert(
        ConsoleAction::SubmitInputText,
        settings_config.get().ui.console.submit_input_text,
    );
    commands.spawn(input_map);
}

fn setup_directive_registry(mut commands: Commands) {
    let mut registry = DirectiveRegistry::default();
    registry.register(HelpDirective);
    registry.register(ExitDirective);
    registry.register(ParticlesDirective);
    commands.insert_resource(registry);
}
