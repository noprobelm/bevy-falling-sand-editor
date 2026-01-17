use bevy::prelude::*;
use bevy_falling_sand::prelude::ChunkLoader;
use serde::{Deserialize, Serialize};

pub struct CameraSetupPlugin;

impl Plugin for CameraSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_camera);
    }
}

#[derive(
    Component,
    Clone,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Reflect,
    Serialize,
    Deserialize,
)]
#[reflect(Component)]
pub struct MainCamera;

#[derive(
    Component, Clone, Default, PartialEq, PartialOrd, Debug, Reflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ZoomTarget {
    pub target_scale: f32,
    pub current_scale: f32,
}

#[derive(
    Component, Clone, Default, PartialEq, PartialOrd, Debug, Reflect, Serialize, Deserialize,
)]
#[reflect(Component)]
pub struct ZoomSpeed(pub f32);

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
