use std::path::PathBuf;

use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::config::{
    CameraConfig, ParticleTypesFile, PrepareWorldSaveEvent, WorldConfig, WorldSaveEvent,
};

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldSaveBuilder>()
            .add_observer(on_prepare_particle_types_save)
            .add_observer(on_world_save);
    }
}

#[derive(Resource, Default)]
pub struct WorldSaveBuilder {
    pub camera: Option<CameraConfig>,
    pub particle_types_file: Option<PathBuf>,
}

fn on_prepare_particle_types_save(
    _trigger: On<PrepareWorldSaveEvent>,
    mut builder: ResMut<WorldSaveBuilder>,
    particle_types_file: Res<ParticleTypesFile>,
) {
    builder.particle_types_file = Some(PathBuf::from(
        particle_types_file
            .0
            .file_name()
            .expect("ParticleTypesFile has no file name"),
    ));
}

fn on_world_save(
    _trigger: On<WorldSaveEvent>,
    mut builder: ResMut<WorldSaveBuilder>,
    mut persistent: ResMut<Persistent<WorldConfig>>,
) {
    let world_config = WorldConfig {
        camera: builder
            .camera
            .take()
            .expect("Camera config not set in WorldSaveBuilder"),
        particle_types_file: builder
            .particle_types_file
            .take()
            .expect("Particle types file not set in WorldSaveBuilder"),
    };

    persistent
        .set(world_config)
        .expect("Failed to set world config");
    persistent
        .persist()
        .expect("Failed to write world data to disk");
}
