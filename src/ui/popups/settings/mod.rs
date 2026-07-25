mod signals;
mod states;
mod ui;

use signals::*;
pub use states::*;
use ui::*;

use bevy::prelude::*;
pub use signals::UiToggleSettingsEvent;
use states::KeybindsListeningState;

pub(super) struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, StatesPlugin, SignalsPlugin))
            .add_systems(
                Update,
                listen_for_keybind.run_if(in_state(KeybindsListeningState::Listening)),
            );
    }
}
