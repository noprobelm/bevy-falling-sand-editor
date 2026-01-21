use bevy::prelude::*;

use crate::{directive::DirectiveRegistry, setup::SetupSystems, ui::HelpDirective};

pub(super) struct DirectiveSetupPlugin;

impl Plugin for DirectiveSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            init_command_registry.in_set(SetupSystems::Directives),
        );
    }
}

fn init_command_registry(mut commands: Commands) {
    let mut registry = DirectiveRegistry::default();
    registry.register::<HelpDirective>();
    commands.insert_resource(registry);
}
