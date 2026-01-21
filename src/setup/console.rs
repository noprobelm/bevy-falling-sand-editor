use bevy::prelude::*;

use crate::{
    directive::DirectiveRegistry,
    setup::SetupSystems,
    ui::{ConsoleCache, ConsoleConfiguration, ConsoleState, HelpDirective},
};

pub(super) struct DirectiveSetupPlugin;

impl Plugin for DirectiveSetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConsoleCache>()
            .init_resource::<ConsoleState>()
            .add_systems(
                Startup,
                (setup_directive_registry, setup_console_configuration)
                    .in_set(SetupSystems::Console),
            );
    }
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
