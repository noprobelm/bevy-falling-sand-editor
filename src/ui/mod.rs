mod directives;
mod states;

use bevy::prelude::*;

pub use directives::*;
pub use states::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((StatesPlugin, DirectivesPlugin));
    }
}
