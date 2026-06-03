use bevy::prelude::*;

use crate::{
    Cursor,
    tools::{
        SelectedTool,
        brush::{ToolBrushColor, ToolBrushSize},
        earthquake::{EarthquakeRegion, components::EarthquakeBrush, states::EarthquakeShape},
    },
};

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_earthquake_brush_gizmos.run_if(in_state(SelectedTool::Earthquake)),
        );
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct EarthquakeBrushGizmos;

fn update_earthquake_brush_gizmos(
    cursor_position: Res<Cursor>,
    region_state: Res<State<EarthquakeShape>>,
    mut brush_gizmos: Gizmos<EarthquakeBrushGizmos>,
    brush_query: Query<(&ToolBrushSize, &ToolBrushColor), With<EarthquakeBrush>>,
) -> Result {
    let (size, color) = brush_query.single()?;

    EarthquakeRegion::from_brush_state(*region_state.get(), cursor_position.current, size.0)
        .draw_gizmo(&mut brush_gizmos, color.0);

    Ok(())
}
