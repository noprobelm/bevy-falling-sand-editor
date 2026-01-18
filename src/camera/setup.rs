use bevy::prelude::*;
use bevy_falling_sand::prelude::ChunkLoader;

use crate::camera::{MainCamera, ZoomSpeed, ZoomTarget};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let initial_scale = 0.25;
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: initial_scale,
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        ChunkLoader,
        ZoomTarget {
            target_scale: initial_scale,
            current_scale: initial_scale,
        },
        ZoomSpeed(8.0),
    ));
}
