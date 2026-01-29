mod setup;
mod ui;

use bevy::prelude::*;

pub use setup::*;
use ui::*;

pub struct ActionPanelPlugin;

impl Plugin for ActionPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, UiPlugin));
    }
}
