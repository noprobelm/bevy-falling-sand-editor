use bevy::prelude::*;
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{InputMap, MouseScrollAxis},
};

use crate::{
    setup::SetupSystems,
    tools::earthquake::{
        components::{EarthquakeBrush, EarthquakeBrushColor, EarthquakeBrushSize},
        gizmos::EarthquakeBrushGizmos,
    },
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<EarthquakeAction>::default())
            .insert_gizmo_config(
                EarthquakeBrushGizmos,
                GizmoConfig {
                    enabled: true,
                    ..default()
                },
            )
            .add_systems(Startup, spawn_earthquake_brush.in_set(SetupSystems::Tools));
    }
}

fn spawn_earthquake_brush(mut commands: Commands) {
    let input_map = InputMap::default().with_axis(EarthquakeAction::ChangeSize, MouseScrollAxis::Y);
    commands.spawn((
        EarthquakeBrush,
        EarthquakeBrushSize(24.0),
        EarthquakeBrushColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        input_map,
    ));
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum EarthquakeAction {
    #[actionlike(Axis)]
    ChangeSize,
}
