use bevy::prelude::*;
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::ActionState};

use crate::{
    Cursor,
    tools::{
        SelectedTool, ToolAction,
        brush::ToolBrushSize,
        earthquake::{
            Earthquake, EarthquakeConfiguration, EarthquakeRegion,
            components::EarthquakeBrush,
            setup::EarthquakeAction,
            states::{EarthquakeBrushState, EarthquakeShape},
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
            resize_earthquake_brush.run_if(in_state(EarthquakeBrushState::Resize)),
        );
    }
}

fn trigger_earthquake(
    mut commands: Commands,
    cursor: Res<Cursor>,
    brush: Single<&ToolBrushSize, With<EarthquakeBrush>>,
    region_state: Res<State<EarthquakeShape>>,
) {
    commands.trigger(Earthquake {
        region: EarthquakeRegion::from_brush_state(*region_state.get(), cursor.current, brush.0),
    });
}

fn resize_earthquake_brush(
    actions: Single<&ActionState<EarthquakeAction>>,
    config: Res<EarthquakeConfiguration>,
    mut brush: Single<&mut ToolBrushSize, With<EarthquakeBrush>>,
) {
    let delta = actions.value(&EarthquakeAction::ChangeSize);
    config.resized_brush_size(&mut brush, delta);
}
