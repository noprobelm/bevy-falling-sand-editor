mod states;
mod ui;

pub use states::*;
use ui::*;

use bevy::prelude::*;

use states::KeybindsListeningState;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, StatesPlugin)).add_systems(
            Update,
            listen_for_keybind.run_if(in_state(KeybindsListeningState::Listening)),
        );
    }
}
