use bevy::prelude::*;
use leafwing_input_manager::{common_conditions::action_pressed, prelude::ActionState};

use crate::ui::{CanvasState, UiState};

use super::{CameraAction, MainCamera, ZoomSpeed, ZoomTarget};

pub(super) struct SystemsPlugin;

const PAN_SPEED_FACTOR: f32 = 10.;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                pan_up.run_if(action_pressed(CameraAction::PanUp)),
                pan_left.run_if(action_pressed(CameraAction::PanLeft)),
                pan_down.run_if(action_pressed(CameraAction::PanDown)),
                pan_right.run_if(action_pressed(CameraAction::PanRight)),
                handle_zoom_target.run_if(in_state(CanvasState::Interact)),
            )
                .chain()
                .run_if(in_state(UiState::Canvas))
                .in_set(CameraSystems),
        );
        app.add_systems(Update, smooth_zoom);
    }
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CameraSystems;

fn pan_speed(current_scale: f32) -> f32 {
    PAN_SPEED_FACTOR * current_scale
}

fn pan_up(camera: Single<(&mut Transform, &ZoomTarget)>) {
    let (mut transform, zoom_target) = camera.into_inner();
    transform.translation.y += pan_speed(zoom_target.current_scale);
}

fn pan_left(camera: Single<(&mut Transform, &ZoomTarget)>) {
    let (mut transform, zoom_target) = camera.into_inner();
    transform.translation.x -= pan_speed(zoom_target.current_scale);
}

fn pan_down(camera: Single<(&mut Transform, &ZoomTarget)>) {
    let (mut transform, zoom_target) = camera.into_inner();
    transform.translation.y -= pan_speed(zoom_target.current_scale);
}

fn pan_right(camera: Single<(&mut Transform, &ZoomTarget)>) {
    let (mut transform, zoom_target) = camera.into_inner();
    transform.translation.x += pan_speed(zoom_target.current_scale);
}

fn handle_zoom_target(
    camera_query: Single<(&mut ZoomTarget, &ActionState<CameraAction>), With<MainCamera>>,
) {
    const ZOOM_IN_FACTOR: f32 = 0.9;
    const ZOOM_OUT_FACTOR: f32 = 1.1;
    const MIN_SCALE: f32 = 0.01;
    const MAX_SCALE: f32 = 10.0;

    let (mut zoom_target, action_state) = camera_query.into_inner();
    let zoom_delta = action_state.value(&CameraAction::Zoom);
    if zoom_delta > 0. {
        zoom_target.target_scale = (zoom_target.target_scale * ZOOM_OUT_FACTOR).min(MAX_SCALE);
    } else if zoom_delta < 0. {
        zoom_target.target_scale = (zoom_target.target_scale * ZOOM_IN_FACTOR).max(MIN_SCALE);
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
