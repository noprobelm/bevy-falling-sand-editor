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
    tools::earthquake::{
        components::{EarthquakeBrush, EarthquakeBrushColor, EarthquakeBrushSize},
        debug::DebugEarthquake,
        gizmos::EarthquakeBrushGizmos,
        states::EarthquakeRegionState,
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
            .add_systems(
                Startup,
                (spawn_earthquake_brush, load_settings)
                    .chain()
                    .in_set(SetupSystems::Tools),
            );
    }
}

fn spawn_earthquake_brush(mut commands: Commands) {
    commands.spawn((
        EarthquakeBrush,
        EarthquakeBrushSize(24.0),
        EarthquakeBrushColor(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        input_map(),
    ));
}

fn load_settings(
    mut commands: Commands,
    mut next_region_state: ResMut<NextState<EarthquakeRegionState>>,
    brush: Single<Entity, With<EarthquakeBrush>>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    commands
        .entity(brush.entity())
        .insert((input_map(), settings_config.earthquake.size));
    next_region_state.set(settings_config.earthquake.region);

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
