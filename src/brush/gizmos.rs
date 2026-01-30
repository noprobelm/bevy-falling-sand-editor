use bevy::prelude::*;

use crate::{
    Cursor,
    brush::{
        components::{Brush, BrushColor, BrushSize},
        states::BrushTypeState,
    },
};

pub struct GizmosPlugin;

impl Plugin for GizmosPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_brush_gizmos);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct BrushGizmos;

fn update_brush_gizmos(
    cursor_position: Res<Cursor>,
    mut brush_gizmos: Gizmos<BrushGizmos>,
    brush_type: Res<State<BrushTypeState>>,
    brush_query: Query<(&BrushSize, &BrushColor), With<Brush>>,
) -> Result {
    let (size, color) = brush_query.single()?;

    match brush_type.get() {
        BrushTypeState::Line => brush_gizmos.line_2d(
            Vec2::new(
                cursor_position.current.x - size.0 as f32 * 3. / 2.,
                cursor_position.current.y,
            ),
            Vec2::new(
                cursor_position.current.x + size.0 as f32 * 3. / 2.,
                cursor_position.current.y,
            ),
            color.0,
        ),
        BrushTypeState::Circle => {
            brush_gizmos.circle_2d(cursor_position.current, size.0 as f32, color.0);
        }
        BrushTypeState::Cursor => brush_gizmos.cross_2d(cursor_position.current, 1., color.0),
    }
    Ok(())
}
