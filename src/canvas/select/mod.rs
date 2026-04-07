mod gizmos;
mod resources;
mod setup;
pub mod states;
mod systems;

use bevy::prelude::*;
use gizmos::*;
use resources::*;
use setup::*;
use states::*;
use systems::*;

pub struct SelectPlugin;

impl Plugin for SelectPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ResourcesPlugin,
            SetupPlugin,
            StatesPlugin,
            SystemsPlugin,
            GizmosPlugin,
        ));
    }
}
