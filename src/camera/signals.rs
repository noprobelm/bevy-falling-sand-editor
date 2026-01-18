use bevy::prelude::*;

use crate::{
    camera::{CameraConfig, ZoomSpeed},
    config::{SaveWorldSignal, SaveWorldSystems, WorldConfigReadyState},
};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            msgr_save_world
                .run_if(in_state(WorldConfigReadyState::Complete))
                .in_set(SaveWorldSystems),
        );
    }
}

fn msgr_save_world(
    msgr_save_world: MessageReader<SaveWorldSignal>,
    mut commands: Commands,
    query: Single<(&Transform, &ZoomSpeed, &Projection)>,
) {
    if !msgr_save_world.is_empty() {
        commands.insert_resource(CameraConfig::from_query(&query));
    }
}
