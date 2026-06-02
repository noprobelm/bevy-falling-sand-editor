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
            components::{EarthquakeBrush, EarthquakeBrushSize},
            setup::EarthquakeAction,
            signals::{Earthquake, EarthquakeRegion},
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
        region: earthquake_region(*region_state.get(), cursor.current, brush.0),
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

fn earthquake_region(state: EarthquakeRegionState, center: Vec2, size: f32) -> EarthquakeRegion {
    match state {
        EarthquakeRegionState::Circle => EarthquakeRegion::circle(center, size),
        EarthquakeRegionState::Rect => EarthquakeRegion::rect(center, Vec2::splat(size), 0.0),
        EarthquakeRegionState::Polygon => {
            let vertices = [
                Vec2::new(0.0, size),
                Vec2::new(size, 0.0),
                Vec2::new(0.0, -size),
                Vec2::new(-size, 0.0),
            ]
            .into_iter()
            .map(|vertex| center + vertex)
            .collect();
            EarthquakeRegion::polygon(vertices)
        }
    }
}
