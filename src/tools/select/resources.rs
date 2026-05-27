use std::collections::HashMap;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SelectedParticles>()
            .init_resource::<DragOrigins>()
            .init_resource::<LastClickTime>()
            .init_resource::<SelectedRegion>();
    }
}

#[derive(Resource, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Reflect)]
pub(super) struct SelectedRegion {
    pub start: Vec2,
    pub stop: Vec2,
}

#[derive(Resource, Clone, Default, PartialEq, Debug, Serialize, Deserialize, Reflect)]
pub struct SelectedParticles {
    pub particles: Vec<Entity>,
}

/// Stores drag start state: cursor position and original grid positions of selected particles.
#[derive(Resource, Clone, Default, Debug)]
pub(super) struct DragOrigins {
    pub cursor_start: IVec2,
    pub origins: HashMap<Entity, IVec2>,
}

/// Tracks the last click time for double-click detection.
#[derive(Resource, Clone, Default, Debug)]
pub(super) struct LastClickTime(pub f64);
