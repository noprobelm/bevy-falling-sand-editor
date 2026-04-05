use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};
use serde::{Deserialize, Serialize};

use crate::{
    config::{ConfigPath, InputButton, SettingsConfig},
    console_command::ConsoleCommandRegistry,
    setup::SetupSystems,
    ui::{
        BrushConsoleCommand, CanvasCommand, CommandHistory, ConsoleInformationAreaState,
        ConsolePromptState, ConwayConsoleCommand, ExitConsoleCommand, ParticlesConsoleCommand,
        SceneConsoleCommand,
    },
};

use super::{ConsoleCache, HelpConsoleCommand};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ConsoleAction>::default())
            .init_resource::<ConsoleCache>()
            .init_resource::<ConsoleInformationAreaState>()
            .init_resource::<ConsolePromptState>()
            .add_systems(
                Startup,
                (
                    load_settings,
                    setup_console_command_registry,
                    load_command_history,
                )
                    .in_set(SetupSystems::Ui),
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
    pub toggle_information_area: InputButton,
    #[serde(skip, default = "ConsoleKeyBindings::default_submit_input_text")]
    pub submit_input_text: InputButton,
}

impl ConsoleKeyBindings {
    pub const fn default_submit_input_text() -> InputButton {
        InputButton::Key(KeyCode::Enter)
    }
}

impl Default for ConsoleKeyBindings {
    fn default() -> Self {
        Self {
            toggle_information_area: KeyCode::Backquote.into(),
            submit_input_text: KeyCode::Enter.into(),
        }
    }
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let keys = &settings_config.get().keys.ui.console;
    let mut input_map = InputMap::default();
    keys.toggle_information_area
        .insert_into_input_map(&mut input_map, ConsoleAction::ToggleInformationArea);
    keys.submit_input_text
        .insert_into_input_map(&mut input_map, ConsoleAction::SubmitInputText);
    commands.spawn(input_map);
}

fn load_command_history(mut commands: Commands, config_path: Res<ConfigPath>) {
    commands.insert_resource(CommandHistory::load(&config_path.0));
}

fn setup_console_command_registry(mut commands: Commands) {
    let mut registry = ConsoleCommandRegistry::default();
    registry.register(HelpConsoleCommand);
    registry.register(ExitConsoleCommand);
    registry.register(ParticlesConsoleCommand);
    registry.register(BrushConsoleCommand);
    registry.register(ConwayConsoleCommand);
    registry.register(SceneConsoleCommand);
    registry.register(CanvasCommand);
    commands.insert_resource(registry);
}
