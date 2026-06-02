use bevy::prelude::*;

use crate::{
    Cursor,
    tools::{
        SelectedTool,
        earthquake::{
            components::{EarthquakeBrush, EarthquakeBrushColor, EarthquakeBrushSize},
            states::EarthquakeRegionState,
        },
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
    region_state: Res<State<EarthquakeRegionState>>,
    mut brush_gizmos: Gizmos<EarthquakeBrushGizmos>,
    brush_query: Query<(&EarthquakeBrushSize, &EarthquakeBrushColor), With<EarthquakeBrush>>,
) -> Result {
    let (size, color) = brush_query.single()?;

    match region_state.get() {
        EarthquakeRegionState::Circle => {
            brush_gizmos.circle_2d(cursor_position.current, size.0, color.0);
        }
        EarthquakeRegionState::Rect => {
            brush_gizmos.rect_2d(
                Isometry2d::from_translation(cursor_position.current),
                Vec2::splat(size.0 * 2.0),
                color.0,
            );
        }
        EarthquakeRegionState::Polygon => {
            let vertices = [
                Vec2::new(0.0, size.0),
                Vec2::new(size.0, 0.0),
                Vec2::new(0.0, -size.0),
                Vec2::new(-size.0, 0.0),
            ]
            .map(|vertex| cursor_position.current + vertex);

            for (&start, &end) in vertices.iter().zip(vertices.iter().cycle().skip(1)) {
                brush_gizmos.line_2d(start, end, color.0);
            }
        }
    }

    Ok(())
}
