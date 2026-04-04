use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedRegion>();
    }
}

#[derive(Resource, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Reflect)]
pub(super) struct SelectedRegion {
    pub start: Vec2,
    pub stop: Vec2,
}

#[derive(Resource, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Reflect)]
pub struct SelectedParticles {
    particles: Vec<Entity>,
}
