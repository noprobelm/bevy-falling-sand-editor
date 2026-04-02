use bevy::prelude::*;
use bevy_falling_sand::prelude::{ChunkLoader, ParticleMap};
use bevy_persistent::Persistent;
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{ActionState, InputMap, MouseScrollAxis},
};
use serde::{Deserialize, Serialize};

use crate::{
    config::{InputButton, SettingsConfig, WorldConfig},
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
                    load_camera_state,
                    // Load camera settings from `settings.toml`
                    load_camera_keybindings,
                )
                    .in_set(SetupSystems::Camera)
                    .chain(),
            )
            .add_systems(
                Update,
                load_chunk_loader
                    .run_if(condition_particle_map_ready)
                    .run_if(run_once),
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
    pub pan_camera_up: InputButton,
    pub pan_camera_left: InputButton,
    pub pan_camera_down: InputButton,
    pub pan_camera_right: InputButton,
}

impl Default for CameraKeyBindings {
    fn default() -> Self {
        CameraKeyBindings {
            pan_camera_up: KeyCode::KeyW.into(),
            pan_camera_left: KeyCode::KeyA.into(),
            pan_camera_down: KeyCode::KeyS.into(),
            pan_camera_right: KeyCode::KeyD.into(),
        }
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn load_camera_state(
    mut commands: Commands,
    camera: Single<(Entity, &MainCamera)>,
    world_config: Res<Persistent<WorldConfig>>,
) {
    let position = world_config.get().camera.position;
    commands.entity(camera.0).insert((
        Camera2d,
        Transform::from_xyz(position.x, position.y, 0.0),
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: world_config.get().camera.scale,
            ..OrthographicProjection::default_2d()
        }),
        // ChunkLoader is initially added here to allow the ParticleMap to perform an initial load.
        // If the world config dictates it should be disabled, it will be removed once the particle
        // map is ready.
        ChunkLoader,
        ZoomTarget {
            target_scale: world_config.get().camera.scale,
            current_scale: world_config.get().camera.scale,
        },
        world_config.get().camera.zoom_speed.clone(),
    ));
}

fn load_chunk_loader(
    mut commands: Commands,
    camera: Single<(Entity, &MainCamera)>,
    world_config: Res<Persistent<WorldConfig>>,
) {
    if world_config.get().camera.chunk_loader_enabled {
        commands.entity(camera.0).insert(ChunkLoader);
    } else {
        commands.entity(camera.0).remove::<ChunkLoader>();
    }
}

fn load_camera_keybindings(
    mut commands: Commands,
    camera: Single<(Entity, &MainCamera)>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let keys = &settings_config.get().keys.camera;
    let mut input_map = InputMap::default().with_axis(CameraAction::Zoom, MouseScrollAxis::Y);
    keys.pan_camera_up
        .insert_into_input_map(&mut input_map, CameraAction::PanUp);
    keys.pan_camera_left
        .insert_into_input_map(&mut input_map, CameraAction::PanLeft);
    keys.pan_camera_down
        .insert_into_input_map(&mut input_map, CameraAction::PanDown);
    keys.pan_camera_right
        .insert_into_input_map(&mut input_map, CameraAction::PanRight);

    commands
        .entity(camera.0)
        .insert((input_map, ActionState::<CameraAction>::default()));
    commands.insert_resource(settings_config.get().keys.camera.clone());
}

fn condition_particle_map_ready(map: Option<Res<ParticleMap>>) -> bool {
    map.is_some()
}
