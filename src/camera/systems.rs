use bevy::{input::mouse::MouseWheel, prelude::*};
use leafwing_input_manager::prelude::ActionState;

use crate::ui::AppState;

use super::{CameraAction, MainCamera, ZoomSpeed, ZoomTarget};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (pan_camera, update_zoom_target)
                .chain()
                .run_if(in_state(AppState::Canvas))
                .in_set(CameraSystems),
        );
        app.add_systems(Update, smooth_zoom);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSystems;

fn pan_camera(
    mut camera_query: Query<(&mut Transform, &ZoomTarget), With<MainCamera>>,
    action_state: Single<&ActionState<CameraAction>>,
) -> Result {
    let (mut transform, zoom_target) = camera_query.single_mut()?;
    let pan_speed = 10. * zoom_target.current_scale;

    if action_state.pressed(&CameraAction::PanUp) {
        transform.translation.y += pan_speed;
    }

    if action_state.pressed(&CameraAction::PanLeft) {
        transform.translation.x -= pan_speed;
    }

    if action_state.pressed(&CameraAction::PanDown) {
        transform.translation.y -= pan_speed;
    }

    if action_state.pressed(&CameraAction::PanRight) {
        transform.translation.x += pan_speed;
    }
    Ok(())
}

fn update_zoom_target(
    mut msgr_scroll: MessageReader<MouseWheel>,
    mut camera_query: Query<&mut ZoomTarget, With<MainCamera>>,
) {
    const ZOOM_IN_FACTOR: f32 = 0.9;
    const ZOOM_OUT_FACTOR: f32 = 1.1;
    const MIN_SCALE: f32 = 0.01;
    const MAX_SCALE: f32 = 10.0;

    if !msgr_scroll.is_empty() {
        let mut zoom_target = match camera_query.single_mut() {
            Ok(z) => z,
            Err(_) => return,
        };

        msgr_scroll.read().for_each(|ev| {
            if ev.y < 0. {
                zoom_target.target_scale =
                    (zoom_target.target_scale * ZOOM_OUT_FACTOR).min(MAX_SCALE);
            } else if ev.y > 0. {
                zoom_target.target_scale =
                    (zoom_target.target_scale * ZOOM_IN_FACTOR).max(MIN_SCALE);
            }
        });
    }
}

fn smooth_zoom(
    mut camera_query: Query<(&mut Projection, &mut ZoomTarget, &ZoomSpeed), With<MainCamera>>,
    time: Res<Time>,
) {
    let (mut projection, mut zoom_target, zoom_speed) = match camera_query.single_mut() {
        Ok(q) => q,
        Err(_) => return,
    };

    let Projection::Orthographic(orthographic) = projection.as_mut() else {
        return;
    };

    let diff = zoom_target.target_scale - zoom_target.current_scale;
    if diff.abs() > 0.0001 {
        let delta = diff * zoom_speed.0 * time.delta_secs();
        zoom_target.current_scale += delta;
        orthographic.scale = zoom_target.current_scale;
    } else {
        zoom_target.current_scale = zoom_target.target_scale;
        orthographic.scale = zoom_target.target_scale;
    }
}
