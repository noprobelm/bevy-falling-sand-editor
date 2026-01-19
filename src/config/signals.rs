use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{
    camera::ZoomSpeed,
    config::{CameraConfig, WorldConfig},
};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveWorldSignal>()
            .add_systems(
                Update,
                (extract_camera_config, persist_world_config).chain(),
            )
            .configure_sets(Update, SaveWorldSystems);
    }
}

/// System set for saving the world to disk.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SaveWorldSystems;

#[derive(Event, Message, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct SaveWorldSignal;

fn extract_camera_config(
    msgr_save_world: MessageReader<SaveWorldSignal>,
    mut commands: Commands,
    query: Single<(&Transform, &ZoomSpeed, &Projection)>,
) {
    if !msgr_save_world.is_empty() {
        commands.insert_resource(CameraConfig::from_query(&query));
    }
}

fn persist_world_config(
    msgr_save_world: MessageReader<SaveWorldSignal>,
    mut persistent: ResMut<Persistent<WorldConfig>>,
    camera_config: Res<CameraConfig>,
) -> Result {
    if !msgr_save_world.is_empty() {
        let world_config = WorldConfig {
            camera: camera_config.clone(),
        };
        persistent.set(world_config)?;
    }
    Ok(())
}
