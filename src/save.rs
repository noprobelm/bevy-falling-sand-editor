use bevy::prelude::*;
use bevy_falling_sand::prelude::PersistChunksSignal;

use crate::config::{PrepareSaveSettingsEvent, PrepareSaveWorldConfigEvent};

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_save_application);
    }
}

#[derive(Event)]
pub struct SaveApplicationEvent;

fn on_save_application(
    _trigger: On<SaveApplicationEvent>,
    mut commands: Commands,
    mut msgw_persist_chunks: MessageWriter<PersistChunksSignal>,
) {
    commands.trigger(PrepareSaveWorldConfigEvent);
    commands.trigger(PrepareSaveSettingsEvent);
    msgw_persist_chunks.write(PersistChunksSignal);
}
