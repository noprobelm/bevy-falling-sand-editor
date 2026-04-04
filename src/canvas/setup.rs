use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};

use crate::{
    canvas::brush::Brush,
    config::SettingsConfig,
    setup::SetupSystems,
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            InputManagerPlugin::<CanvasAction>::default(),
            InputManagerPlugin::<CanvasStateActions>::default(),
        ))
        .add_systems(
            Startup,
            load_settings.in_set(SetupSystems::Brush),
        );
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CanvasAction {
    Draw,
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CanvasStateActions {
    Modify,
}

fn load_settings(
    mut commands: Commands,
    brush: Single<Entity, With<Brush>>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let mut input_map = InputMap::default();
    settings_config
        .keys
        .brush
        .draw
        .insert_into_input_map(&mut input_map, CanvasAction::Draw);
    commands.entity(brush.entity()).insert(input_map);

    let mut input_map = InputMap::default();
    settings_config
        .keys
        .ui
        .general
        .hold_canvas_mode_edit
        .insert_into_input_map(&mut input_map, CanvasStateActions::Modify);
    commands.spawn(input_map);
    commands.insert_resource(settings_config.get().keys.ui.clone());
}
