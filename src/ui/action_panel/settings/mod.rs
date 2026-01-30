mod states;
mod ui;

pub use states::*;
use ui::*;

use bevy::prelude::*;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, StatesPlugin));
    }
}
