use bevy::prelude::*;
use bevy_falling_sand::core::ChunkLoader;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};

use crate::{
    camera::{MainCamera, ZoomTarget},
    config::{SettingsConfig, WorldConfig},
    setup::SetupSystems,
};

pub(super) struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<CameraAction>::default())
            .add_systems(
                Startup,
                (
                    // spawn_camera,
                    spawn_camera,
                    // Load camera state information from `world.toml`
                    load_world_state,
                    // Load camera settings from `settings.toml`
                    load_settings,
                )
                    .in_set(SetupSystems::Camera)
                    .chain(),
            );
    }
}

#[derive(Actionlike, Resource, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum CameraAction {
    PanUp,
    PanRight,
    PanDown,
    PanLeft,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn load_world_state(
    mut commands: Commands,
    camera: Single<(Entity, &MainCamera)>,
    world_config: Res<Persistent<WorldConfig>>,
) {
    commands.insert_resource(world_config.camera.clone());
    commands.entity(camera.0).insert((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: world_config.get().camera.scale,
            ..OrthographicProjection::default_2d()
        }),
        ChunkLoader,
        ZoomTarget {
            target_scale: world_config.get().camera.scale,
            current_scale: world_config.get().camera.scale,
        },
        world_config.get().camera.zoom_speed.clone(),
    ));
}

fn load_settings(
    mut commands: Commands,
    camera: Single<(Entity, &MainCamera)>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let mut input_map = InputMap::default();
    input_map.insert(
        CameraAction::PanUp,
        settings_config.get().camera.pan_camera_up,
    );
    input_map.insert(
        CameraAction::PanLeft,
        settings_config.get().camera.pan_camera_left,
    );
    input_map.insert(
        CameraAction::PanDown,
        settings_config.get().camera.pan_camera_down,
    );
    input_map.insert(
        CameraAction::PanRight,
        settings_config.get().camera.pan_camera_right,
    );

    commands.entity(camera.0).insert(input_map);
}
