mod components;
mod gizmos;
mod setup;
mod states;
mod systems;
mod algs;

use bevy::prelude::*;

use crate::brush::{gizmos::GizmosPlugin, setup::SetupPlugin, systems::SystemsPlugin};
pub use setup::*;
pub use states::*;
pub use algs::*;

pub struct BrushPlugin;

impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, StatesPlugin, SystemsPlugin, GizmosPlugin));
    }
}
