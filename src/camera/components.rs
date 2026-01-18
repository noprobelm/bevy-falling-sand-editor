use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct ComponentsPlugin;

impl Plugin for ComponentsPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<MainCamera>()
            .register_type::<ZoomTarget>()
            .register_type::<ZoomSpeed>();
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
