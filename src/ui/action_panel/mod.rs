mod setup;
mod ui;

use bevy::prelude::*;

use setup::*;
use ui::*;

pub struct ActionPanelPlugin;

impl Plugin for ActionPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, SetupPlugin));
    }
}
