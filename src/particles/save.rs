use std::path::PathBuf;

use bevy::prelude::*;

use crate::config::{ParticleTypesFile, PrepareWorldSaveEvent, WorldSaveBuilder};

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_prepare_world_save);
    }
}

fn on_prepare_world_save(
    _trigger: On<PrepareWorldSaveEvent>,
    mut builder: ResMut<WorldSaveBuilder>,
    particle_types_file: Res<ParticleTypesFile>,
) {
    builder.particle_types_file = Some(PathBuf::from(
        particle_types_file
            .0
            .file_name()
            .expect("ParticleTypesFile resource has no file specified"),
    ));
}
