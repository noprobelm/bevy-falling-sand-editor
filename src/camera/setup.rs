use bevy::prelude::*;
use bevy_falling_sand::prelude::ChunkLoader;
use bevy_persistent::Persistent;

use crate::{
    camera::{CameraLoadedState, MainCamera, ZoomTarget},
    config::{WorldConfig, WorldConfigReadyState},
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(WorldConfigReadyState::Complete), setup_camera);
    }
}

fn setup_camera(
    mut commands: Commands,
    world_config: Res<Persistent<WorldConfig>>,
    mut state: ResMut<NextState<CameraLoadedState>>,
) {
    commands.spawn((
        Camera2d,
        world_config.get().camera.projection.clone(),
        MainCamera,
        ChunkLoader,
        ZoomTarget {
            target_scale: world_config.get().camera.scale,
            current_scale: world_config.get().camera.scale,
        },
        world_config.get().camera.zoom_speed.clone(),
    ));
    state.set(CameraLoadedState::Complete)
}
