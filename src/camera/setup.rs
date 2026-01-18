use bevy::prelude::*;
use bevy_falling_sand::prelude::ChunkLoader;
use bevy_persistent::Persistent;

use crate::{
    camera::{MainCamera, ZoomTarget},
    config::{CameraInitState, WorldConfig, WorldConfigState},
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(WorldConfigState::Initialized), setup_camera);
    }
}

fn setup_camera(
    mut commands: Commands,
    world_config: Res<Persistent<WorldConfig>>,
    mut state: ResMut<NextState<CameraInitState>>,
) {
    commands.insert_resource(world_config.camera.clone());
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: world_config.get().camera.scale,
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        ChunkLoader,
        ZoomTarget {
            target_scale: world_config.get().camera.scale,
            current_scale: world_config.get().camera.scale,
        },
        world_config.get().camera.zoom_speed.clone(),
    ));
    state.set(CameraInitState::Initialized)
}
