use bevy::prelude::*;
use bevy_falling_sand::ParticleMap;
use leafwing_input_manager::common_conditions::{
    action_just_pressed, action_just_released, action_pressed,
};

use crate::{
    Cursor,
    canvas::{CanvasAction, select::gizmos::SelectGizmos},
    ui::CanvasState,
};

use super::resources::SelectedRegion;

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                reset_selected_region
                    .run_if(action_just_pressed(CanvasAction::Draw))
                    .run_if(in_state(CanvasState::Select)),
                update_selected_region
                    .run_if(action_pressed(CanvasAction::Draw))
                    .run_if(in_state(CanvasState::Select)),
                commit_selected_region
                    .run_if(action_just_released(CanvasAction::Draw))
                    .run_if(in_state(CanvasState::Select)),
            )
                .chain(),
        );
    }
}

fn reset_selected_region(
    cursor: Res<Cursor>,
    mut region: ResMut<SelectedRegion>,
    mut config_store: ResMut<GizmoConfigStore>,
) {
    region.start = cursor.current;
    region.stop = cursor.current;
    let (config, _) = config_store.config_mut::<SelectGizmos>();
    config.enabled = true;
}

fn update_selected_region(cursor: Res<Cursor>, mut region: ResMut<SelectedRegion>) {
    region.stop = cursor.current;
}

fn commit_selected_region(map: Res<ParticleMap>, mut config_store: ResMut<GizmoConfigStore>) {
    let (config, _) = config_store.config_mut::<SelectGizmos>();
    config.enabled = false;
}
