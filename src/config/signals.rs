use std::path::PathBuf;

use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{
    camera::ZoomSpeed,
    config::{CameraConfig, ParticleTypesFile, WorldConfig},
};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveWorldSignal>()
            .add_systems(Update, (save_camera, save_world).chain());
    }
}

#[derive(Event, Message, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct SaveWorldSignal;

fn save_camera(
    msgr_save_world: MessageReader<SaveWorldSignal>,
    mut commands: Commands,
    query: Single<(&Transform, &ZoomSpeed, &Projection)>,
) {
    if !msgr_save_world.is_empty() {
        commands.insert_resource(CameraConfig::from_query(&query));
    }
}

fn save_world(
    msgr_save_world: MessageReader<SaveWorldSignal>,
    mut persistent: ResMut<Persistent<WorldConfig>>,
    camera_config: Res<CameraConfig>,
    particle_types_file: Res<ParticleTypesFile>,
) -> Result {
    if !msgr_save_world.is_empty() {
        let world_config = WorldConfig {
            camera: camera_config.clone(),
            particle_types_file: PathBuf::from(particle_types_file.0.file_name().unwrap()),
        };
        persistent.set(world_config)?;
    }
    Ok(())
}
