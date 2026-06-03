use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};

use crate::{config::SettingsConfig, setup::SetupSystems};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<ToolAction>::default(),
            InputManagerPlugin::<ToolStateActions>::default(),
        ))
        .add_systems(Startup, load_settings.in_set(SetupSystems::Tools));
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ToolAction {
    Primary,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum ToolStateActions {
    Resize,
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let mut input_map = InputMap::default();
    settings_config
        .keys
        .painter
        .draw
        .insert_into_input_map(&mut input_map, ToolAction::Primary);
    commands.spawn(input_map);

    let mut input_map = InputMap::default();
    settings_config
        .keys
        .ui
        .general
        .resize_tool
        .insert_into_input_map(&mut input_map, ToolStateActions::Resize);
    commands.spawn(input_map);
    commands.insert_resource(settings_config.get().keys.ui.clone());
}
