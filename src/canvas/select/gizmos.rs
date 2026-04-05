use bevy::prelude::*;
use leafwing_input_manager::common_conditions::action_pressed;

use crate::{
    canvas::{CanvasAction, select::SelectedRegion},
    ui::CanvasState,
};

pub(super) struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.insert_gizmo_config(
            SelectGizmos,
            GizmoConfig {
                enabled: false,
                ..default()
            },
        )
        .add_systems(
            Update,
            update_select_gizmos
                .run_if(action_pressed(CanvasAction::Draw))
                .run_if(in_state(CanvasState::Select)),
        );
        //app.add_systems(Update, update_brush_gizmos);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct SelectGizmos;

fn update_select_gizmos(region: Res<SelectedRegion>, mut gizmos: Gizmos<SelectGizmos>) {
    let rect = Rect::from_corners(region.start, region.stop);
    let center = rect.center();
    let size = rect.size();
    gizmos.rect_2d(Isometry2d::from_translation(center), size, Color::WHITE);
}
