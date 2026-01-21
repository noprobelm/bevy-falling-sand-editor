use bevy::prelude::*;

use crate::{commands::CommandRegistry, setup::SetupSystems};

pub(super) struct CommandsSetupPlugin;

impl Plugin for CommandsSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            init_command_registry.in_set(SetupSystems::Commands),
        );
    }
}

fn init_command_registry(mut commands: Commands) {
    let mut registry = CommandRegistry::default();
    //registry.register::<HelpCommand>();
    commands.insert_resource(registry);
}
