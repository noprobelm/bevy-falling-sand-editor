pub mod brush;
mod select;

use bevy::prelude::*;

pub use brush::*;
pub use select::*;

pub struct CanvasPlugin;

impl Plugin for CanvasPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((brush::BrushPlugin, select::SelectPlugin));
    }
}
