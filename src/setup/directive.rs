use bevy::prelude::*;

use crate::{directive::DirectiveRegistry, setup::SetupSystems};

pub(super) struct DirectiveSetupPlugin;

impl Plugin for DirectiveSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            init_command_registry.in_set(SetupSystems::Commands),
        );
    }
}

fn init_command_registry(mut commands: Commands) {
    let mut registry = DirectiveRegistry::default();
    //registry.register::<HelpCommand>();
    commands.insert_resource(registry);
}
