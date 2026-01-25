use bevy::prelude::*;

use crate::{
    camera::ZoomSpeed,
    config::{CameraConfig, PrepareWorldSaveEvent, WorldSaveBuilder},
};

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_prepare_camera_save);
    }
}

fn on_prepare_camera_save(
    _trigger: On<PrepareWorldSaveEvent>,
    mut builder: ResMut<WorldSaveBuilder>,
    query: Single<(&Transform, &ZoomSpeed, &Projection)>,
) {
    let scale = match query.2 {
        Projection::Orthographic(ortho) => ortho.scale,
        _ => unreachable!(),
    };
    let position = Vec2::new(query.0.translation.x, query.0.translation.y);

    builder.camera = Some(CameraConfig {
        scale,
        zoom_speed: query.1.clone(),
        position,
    });
}
