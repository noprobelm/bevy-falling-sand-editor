use bevy::prelude::*;
use bevy_falling_sand::core::ChunkLoader;
use bevy_persistent::Persistent;
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{InputMap, MouseScrollAxis},
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{SettingsConfig, WorldConfig},
    setup::SetupSystems,
};

use super::{MainCamera, ZoomTarget};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
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
    #[actionlike(Axis)]
    Zoom,
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct CameraKeyBindings {
    pub pan_camera_up: KeyCode,
    pub pan_camera_left: KeyCode,
    pub pan_camera_down: KeyCode,
    pub pan_camera_right: KeyCode,
}

impl Default for CameraKeyBindings {
    fn default() -> Self {
        CameraKeyBindings {
            pan_camera_up: KeyCode::KeyW,
            pan_camera_left: KeyCode::KeyA,
            pan_camera_down: KeyCode::KeyS,
            pan_camera_right: KeyCode::KeyD,
        }
    }
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
    let position = world_config.get().camera.position;
    commands.entity(camera.0).insert((
        Camera2d,
        Transform::from_xyz(position.x, position.y, 0.0),
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
    let input_map = InputMap::default()
        .with_axis(CameraAction::Zoom, MouseScrollAxis::Y)
        .with(
            CameraAction::PanUp,
            settings_config.get().camera.pan_camera_up,
        )
        .with(
            CameraAction::PanLeft,
            settings_config.get().camera.pan_camera_left,
        )
        .with(
            CameraAction::PanDown,
            settings_config.get().camera.pan_camera_down,
        )
        .with(
            CameraAction::PanRight,
            settings_config.get().camera.pan_camera_right,
        );

    commands.entity(camera.0).insert(input_map);
    commands.insert_resource(settings_config.get().camera.clone());
}
