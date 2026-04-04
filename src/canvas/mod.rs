pub mod brush;
mod select;
mod setup;

use bevy::prelude::*;

pub use setup::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((setup::SetupPlugin, brush::BrushPlugin, select::SelectPlugin));
    }
}
