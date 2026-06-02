mod brush;
mod select;
mod states;
mod ui;
mod signals;

pub use brush::*;
pub use select::*;
pub use states::*;
use ui::*;
pub use signals::*;

use bevy::prelude::*;

pub(super) struct ToolOptionsPlugin;

impl Plugin for ToolOptionsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StatesPlugin, UiPlugin, SignalsPlugin));
    }
}
