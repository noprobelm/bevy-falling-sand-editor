use bevy::prelude::*;
use bevy_persistent::Persistent;

use crate::{
    camera::CameraConfig,
    config::{SaveWorldSystems, WorldConfig, WorldConfigReadyState},
};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveWorldSignal>().add_systems(
            Update,
            msgr_save_world
                .run_if(in_state(WorldConfigReadyState::Complete))
                .after(SaveWorldSystems),
        );
    }
}

#[derive(Event, Message, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct SaveWorldSignal;

fn msgr_save_world(
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
