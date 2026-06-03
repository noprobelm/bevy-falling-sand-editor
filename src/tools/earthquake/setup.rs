use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{InputMap, MouseScrollAxis},
};

use crate::{
    config::SettingsConfig,
    setup::SetupSystems,
    tools::{
        brush::{ToolBrushColor, ToolBrushSize},
        earthquake::{
            EarthquakeConfiguration,
            components::EarthquakeBrush,
            debug::DebugEarthquake,
            gizmos::EarthquakeBrushGizmos,
            resources::EARTHQUAKE_BRUSH_DEFAULT_SIZE,
            states::{EarthquakeFractureShape, EarthquakeShape},
        },
    },
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<EarthquakeAction>::default())
            .init_resource::<EarthquakeConfiguration>()
            .insert_gizmo_config(
                EarthquakeBrushGizmos,
                GizmoConfig {
                    enabled: true,
                    ..default()
                },
            )
            .add_systems(
                Startup,
                (spawn_earthquake_brush, load_settings)
                    .chain()
                    .in_set(SetupSystems::Tools),
            );
    }
}

fn spawn_earthquake_brush(mut commands: Commands, config: Res<EarthquakeConfiguration>) {
    commands.spawn((
        EarthquakeBrush,
        ToolBrushSize(EARTHQUAKE_BRUSH_DEFAULT_SIZE),
        ToolBrushColor(config.brush.color),
        input_map(),
    ));
}

fn load_settings(
    mut commands: Commands,
    mut next_region_state: ResMut<NextState<EarthquakeShape>>,
    mut next_fracture_shape_state: ResMut<NextState<EarthquakeFractureShape>>,
    brush: Single<Entity, With<EarthquakeBrush>>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    commands.entity(brush.entity()).insert((
        input_map(),
        settings_config.earthquake.size,
        ToolBrushColor(settings_config.earthquake.configuration.brush.color),
    ));
    commands.insert_resource(settings_config.earthquake.configuration.clone());
    next_region_state.set(settings_config.earthquake.region);
    next_fracture_shape_state.set(settings_config.earthquake.fracture_shape);

    if settings_config.earthquake.debug {
        commands.insert_resource(DebugEarthquake);
    }
}

fn input_map() -> InputMap<EarthquakeAction> {
    InputMap::default().with_axis(EarthquakeAction::ChangeSize, MouseScrollAxis::Y)
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum EarthquakeAction {
    #[actionlike(Axis)]
    ChangeSize,
}
