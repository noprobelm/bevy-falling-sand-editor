mod gizmos;
mod resources;
mod systems;

use bevy::prelude::*;
use gizmos::*;
use resources::*;
use systems::*;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ResourcesPlugin, SystemsPlugin, GizmosPlugin));
    }
}
