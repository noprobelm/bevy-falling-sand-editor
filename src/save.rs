use bevy::prelude::*;
use bevy_falling_sand::prelude::{PersistChunksSignal, PersistParticleTypesSignal};

use crate::config::{ParticleTypesFile, PrepareSaveSettingsEvent, PrepareSaveWorldConfigEvent};

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
    mut msgw_persist_particle_types: MessageWriter<PersistParticleTypesSignal>,
    particle_types_file: Res<ParticleTypesFile>,
) {
    commands.trigger(PrepareSaveWorldConfigEvent);
    commands.trigger(PrepareSaveSettingsEvent);
    msgw_persist_chunks.write(PersistChunksSignal);
    msgw_persist_particle_types.write(PersistParticleTypesSignal(particle_types_file.0.clone()));
}
