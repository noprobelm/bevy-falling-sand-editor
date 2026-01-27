use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;

use crate::{
    brush::{BrushAction, components::BrushSize},
    ui::CanvasState,
};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (change_brush_size.run_if(in_state(CanvasState::Edit)),),
        );
    }
}

fn change_brush_size(mut single: Single<(&ActionState<BrushAction>, &mut BrushSize)>) {
    let (action_state, brush_size) = (single.0, &mut single.1);
    let delta = action_state.value(&BrushAction::ChangeSize);
    if delta > 0.0 {
        brush_size.0 = brush_size.0.saturating_add(1);
    } else if delta < 0.0 {
        brush_size.0 = brush_size.0.saturating_sub(1).max(1);
    }
}
