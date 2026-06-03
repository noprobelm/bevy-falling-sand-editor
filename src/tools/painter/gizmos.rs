use bevy::prelude::*;

use crate::{
    Cursor,
    tools::{
        SelectedTool,
        brush::{ToolBrushColor, ToolBrushSize},
        painter::{components::PainterBrush, states::PainterShape},
    },
};

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_brush_gizmos.run_if(in_state(SelectedTool::Painter)),
        );
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct PainterBrushGizmos;

fn update_brush_gizmos(
    cursor_position: Res<Cursor>,
    mut brush_gizmos: Gizmos<PainterBrushGizmos>,
    brush_type: Res<State<PainterShape>>,
    brush_query: Query<(&ToolBrushSize, &ToolBrushColor), With<PainterBrush>>,
) -> Result {
    let (size, color) = brush_query.single()?;

    match brush_type.get() {
        PainterShape::Line => brush_gizmos.line_2d(
            Vec2::new(
                cursor_position.current.x - size.0 * 3. / 2.,
                cursor_position.current.y,
            ),
            Vec2::new(
                cursor_position.current.x + size.0 * 3. / 2.,
                cursor_position.current.y,
            ),
            color.0,
        ),
        PainterShape::Circle => {
            brush_gizmos.circle_2d(cursor_position.current, size.0, color.0);
        }
        PainterShape::Cursor => brush_gizmos.cross_2d(cursor_position.current, 1., color.0),
    }
    Ok(())
}
