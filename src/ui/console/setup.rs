use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};

use crate::{
    config::SettingsConfig,
    directive::DirectiveRegistry,
    setup::SetupSystems,
    ui::{ExitDirective, ParticlesDirective},
};

use super::{ConsoleCache, ConsoleState, HelpDirective};

pub struct ConsoleSetupPlugin;

impl Plugin for ConsoleSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ConsoleAction>::default())
            .init_resource::<ConsoleCache>()
            .init_resource::<ConsoleState>()
            .add_systems(
                Startup,
                (load_settings, setup_directive_registry).in_set(SetupSystems::Console),
            );
    }
}

#[derive(Actionlike, Resource, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ConsoleAction {
    ToggleInformationArea,
    SubmitInputText,
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let mut input_map = InputMap::default();
    input_map.insert(
        ConsoleAction::ToggleInformationArea,
        settings_config.get().console.toggle_information_area,
    );
    input_map.insert(ConsoleAction::SubmitInputText, KeyCode::Enter);
    commands.spawn(input_map);
}

fn setup_directive_registry(mut commands: Commands) {
    let mut registry = DirectiveRegistry::default();
    registry.register(HelpDirective);
    registry.register(ExitDirective);
    registry.register(ParticlesDirective);
    commands.insert_resource(registry);
}
