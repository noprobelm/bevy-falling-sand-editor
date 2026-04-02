use std::path::PathBuf;

use bevy::prelude::*;
use bevy_falling_sand::prelude::ChunkLoader;
use bevy_persistent::Persistent;

use crate::{
    camera::{MainCamera, ZoomSpeed},
    config::{CameraConfig, ParticleTypesFile, WorldConfig},
};

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldConfigBuilder>()
            .add_observer(on_prepare_save_camera)
            .add_observer(on_prepare_save_particle_types)
            .add_observer(on_save_world)
            .add_observer(on_prepare_save_world);
    }
}

/// Trigger this event to begin the world save process.
#[derive(Event, Default, Debug)]
pub struct PrepareSaveWorldConfigEvent;

/// Triggered automatically after PrepareWorldSaveEvent handlers complete.
#[derive(Event, Default, Debug)]
pub struct SaveWorldConfigEvent;

#[derive(Resource, Default)]
pub struct WorldConfigBuilder {
    pub camera: Option<CameraConfig>,
    pub particle_types_file: Option<PathBuf>,
}

fn on_prepare_save_camera(
    _trigger: On<PrepareSaveWorldConfigEvent>,
    mut builder: ResMut<WorldConfigBuilder>,
    query: Single<(&Transform, &ZoomSpeed, &Projection, Option<&ChunkLoader>), With<MainCamera>>,
) {
    let scale = match query.2 {
        Projection::Orthographic(ortho) => ortho.scale,
        _ => unreachable!(),
    };
    let position = Vec2::new(query.0.translation.x, query.0.translation.y);
    let chunk_loader_enabled = query.3.is_some();

    builder.camera = Some(CameraConfig {
        scale,
        zoom_speed: query.1.clone(),
        position,
        chunk_loader_enabled,
    });
}

fn on_prepare_save_particle_types(
    _trigger: On<PrepareSaveWorldConfigEvent>,
    mut builder: ResMut<WorldConfigBuilder>,
    particle_types_file: Res<ParticleTypesFile>,
) {
    builder.particle_types_file = Some(PathBuf::from(
        particle_types_file
            .0
            .file_name()
            .expect("ParticleTypesFile has no file name"),
    ));
}

fn on_prepare_save_world(_trigger: On<PrepareSaveWorldConfigEvent>, mut commands: Commands) {
    commands.trigger(SaveWorldConfigEvent);
}

fn on_save_world(
    _trigger: On<SaveWorldConfigEvent>,
    mut builder: ResMut<WorldConfigBuilder>,
    mut persistent: ResMut<Persistent<WorldConfig>>,
) {
    let world_config = WorldConfig {
        camera: builder.camera.take().expect("Camera config not set"),
        particle_types_file: builder
            .particle_types_file
            .take()
            .expect("Particle types file not set"),
    };

    persistent
        .set(world_config)
        .expect("Failed to save world config");
    persistent
        .persist()
        .expect("Failed to write world data to disk");
}
