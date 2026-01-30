use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{InputMap, MouseScrollAxis},
};
use serde::{Deserialize, Serialize};

use crate::{
    brush::{
        BrushModeSpawnState, BrushModeState, BrushTypeState,
        components::{Brush, BrushColor, BrushSize},
        gizmos::BrushGizmos,
    },
    config::SettingsConfig,
    setup::SetupSystems,
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<BrushAction>::default())
            .init_state::<BrushTypeState>()
            .init_state::<BrushModeState>()
            .add_sub_state::<BrushModeSpawnState>()
            .insert_gizmo_config(
                BrushGizmos,
                GizmoConfig {
                    enabled: true,
                    ..default()
                },
            )
            .add_systems(
                Startup,
                (spawn_brush, load_settings)
                    .chain()
                    .in_set(SetupSystems::Brush),
            );
    }
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct BrushKeyBindings {
    pub draw: MouseButton,
    pub toggle_brush_mode: MouseButton,
}

impl Default for BrushKeyBindings {
    fn default() -> Self {
        Self {
            draw: MouseButton::Left,
            toggle_brush_mode: MouseButton::Right,
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum BrushAction {
    ToggleMode,
    ToggleType,
    #[actionlike(Axis)]
    ChangeSize,
    Draw,
}

fn spawn_brush(mut commands: Commands) {
    commands.spawn((
        Brush,
        BrushSize(2),
        BrushColor(Color::Srgba(Srgba::new(1., 1., 1., 0.3))),
    ));
}

fn load_settings(
    mut commands: Commands,
    brush: Single<Entity, With<Brush>>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let input_map = InputMap::default()
        .with_axis(BrushAction::ChangeSize, MouseScrollAxis::Y)
        .with(
            BrushAction::ToggleMode,
            settings_config.brush.toggle_brush_mode,
        )
        .with(BrushAction::Draw, settings_config.brush.draw);

    commands.entity(brush.entity()).insert(input_map);
}
