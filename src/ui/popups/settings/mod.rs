mod states;
mod ui;
mod signals;

pub use states::*;
use ui::*;
use signals::*;

use bevy::prelude::*;
pub use signals::*;
use states::KeybindsListeningState;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, StatesPlugin, SignalsPlugin)).add_systems(
            Update,
            listen_for_keybind.run_if(in_state(KeybindsListeningState::Listening)),
        );
    }
}
