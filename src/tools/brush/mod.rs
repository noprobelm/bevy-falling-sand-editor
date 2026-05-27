mod components;
mod gizmos;
mod setup;
mod signals;
mod states;
pub mod systems;

use bevy::prelude::*;
pub use signals::*;

use crate::tools::brush::{gizmos::GizmosPlugin, setup::SetupPlugin, systems::SystemsPlugin};
pub use components::*;
pub use setup::*;
pub use states::*;

pub struct BrushToolPlugin;

impl Plugin for BrushToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SetupPlugin,
            SignalsPlugin,
            StatesPlugin,
            SystemsPlugin,
            GizmosPlugin,
        ));
    }
}
