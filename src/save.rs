use bevy::prelude::*;

use crate::config::{PrepareSaveSettingsEvent, PrepareSaveWorldConfigEvent};
use bevy_falling_sand::prelude::PersistChunksSignal;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_application);
    }
}

#[derive(Event)]
pub struct SaveApplicationEvent;

fn on_save_application(_trigger: On<SaveApplicationEvent>, mut commands: Commands) {
    commands.trigger(PrepareSaveWorldConfigEvent);
    commands.trigger(PrepareSaveSettingsEvent);
}
