mod persistence;
mod save;

use bevy::prelude::*;

pub use persistence::*;
pub use save::PrepareSaveSettingsEvent;

pub(super) struct SettingsPersistencePlugin;

impl Plugin for SettingsPersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(save::SavePlugin);
    }
}
