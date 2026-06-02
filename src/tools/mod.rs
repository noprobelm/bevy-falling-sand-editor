pub mod brush;
pub mod earthquake;
pub mod select;
mod setup;
mod signals;
mod states;

use bevy::prelude::*;

pub use setup::*;
pub use signals::*;
pub use states::*;

pub struct ToolsPlugin;

impl Plugin for ToolsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            setup::SetupPlugin,
            states::StatesPlugin,
            signals::SignalsPlugin,
            brush::BrushToolPlugin,
            select::SelectToolPlugin,
            earthquake::EarthquakePlugin,
        ));
    }
}
