mod components;
mod gizmos;
mod setup;
mod states;
mod systems;

use bevy::prelude::*;

use crate::brush::{gizmos::GizmosPlugin, setup::SetupPlugin, systems::SystemsPlugin};
pub use components::SelectedBrushParticle;
pub use setup::*;
pub use states::*;

pub struct BrushPlugin;

impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, StatesPlugin, SystemsPlugin, GizmosPlugin));
    }
}
