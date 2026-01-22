use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};

use crate::{
    config::SettingsConfig,
    directive::DirectiveRegistry,
    setup::SetupSystems,
    ui::{ConsoleCache, ConsoleConfiguration, ConsoleState, HelpDirective},
};

pub(super) struct DirectiveSetupPlugin;

impl Plugin for DirectiveSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<ConsoleAction>::default())
            .init_resource::<ConsoleCache>()
            .init_resource::<ConsoleState>()
            .add_systems(
                Startup,
                (
                    load_settings,
                    setup_directive_registry,
                    setup_console_configuration,
                )
                    .in_set(SetupSystems::Console),
            );
    }
}

#[derive(Actionlike, Resource, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ConsoleAction {
    ToggleInformationArea,
}

#[derive(Component)]
struct ConsoleActionEntity;

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let mut input_map = InputMap::default();

    input_map.insert(
        ConsoleAction::ToggleInformationArea,
        settings_config.get().console.toggle_information_area,
    );
    commands.spawn((ConsoleActionEntity, input_map));
}

fn setup_directive_registry(mut commands: Commands) {
    let mut registry = DirectiveRegistry::default();
    registry.register::<HelpDirective>();
    commands.insert_resource(registry);
}

fn setup_console_configuration(mut commands: Commands) {
    let mut config = ConsoleConfiguration::default();
    config.register_directive::<HelpDirective>();
    commands.insert_resource(config);
}
