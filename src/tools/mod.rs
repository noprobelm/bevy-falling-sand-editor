pub mod brush;
pub mod select;
mod setup;

use bevy::prelude::*;

pub use setup::*;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            setup::SetupPlugin,
            brush::BrushToolPlugin,
            select::SelectToolPlugin,
        ));
    }
}
