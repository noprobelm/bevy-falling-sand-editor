use bevy::prelude::*;
use leafwing_input_manager::{
    common_conditions::{action_just_pressed, action_pressed},
    prelude::ActionState,
};

use crate::{
    Cursor,
    tools::{
        SelectedTool, ToolAction, ToolStateActions,
        earthquake::{
            Earthquake, EarthquakeRegion,
            components::{EarthquakeBrush, EarthquakeBrushSize},
            setup::EarthquakeAction,
            states::EarthquakeRegionState,
        },
    },
};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            trigger_earthquake
                .run_if(action_just_pressed(ToolAction::Primary))
                .run_if(in_state(SelectedTool::Earthquake)),
        )
        .add_systems(
            Update,
            resize_earthquake_brush
                .run_if(action_pressed(ToolStateActions::Resize))
                .run_if(in_state(SelectedTool::Earthquake)),
        );
    }
}

fn trigger_earthquake(
    mut commands: Commands,
    cursor: Res<Cursor>,
    brush: Single<&EarthquakeBrushSize, With<EarthquakeBrush>>,
    region_state: Res<State<EarthquakeRegionState>>,
) {
    commands.trigger(Earthquake {
        region: EarthquakeRegion::from_brush_state(*region_state.get(), cursor.current, brush.0),
    });
}

fn resize_earthquake_brush(
    actions: Single<&ActionState<EarthquakeAction>>,
    mut brush: Single<&mut EarthquakeBrushSize, With<EarthquakeBrush>>,
) {
    let delta = actions.value(&EarthquakeAction::ChangeSize);
    if delta > 0.0 {
        brush.0 += 1.0;
    } else if delta < 0.0 {
        brush.0 = (brush.0 - 1.0).max(1.0);
    }
}
