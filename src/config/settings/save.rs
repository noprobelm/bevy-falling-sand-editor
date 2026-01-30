use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::config::SettingsConfig;

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_settings);
    }
}

/// Event to trigger saving settings data
#[derive(Event, Default, Debug)]
pub struct SaveSettingsEvent;

fn on_save_settings(
    _trigger: On<SaveSettingsEvent>,
    persistent: ResMut<Persistent<SettingsConfig>>,
) {
    persistent
        .persist()
        .expect("Failed to write settings to disk");
}
