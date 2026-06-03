mod brush;
mod select;
mod signals;
mod states;
mod ui;

pub use brush::*;
pub use select::*;
pub use signals::*;
pub use states::*;
use ui::*;

use bevy::prelude::*;

pub(super) struct ToolOptionsPlugin;

impl Plugin for ToolOptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StatesPlugin, UiPlugin, SignalsPlugin));
    }
}
