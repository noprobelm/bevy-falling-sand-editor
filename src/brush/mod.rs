use bevy::prelude::*;

pub struct BrushPlugin;

impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app.insert_gizmo_config(
            BrushGizmos,
            GizmoConfig {
                enabled: true,
                ..default()
            },
        );
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct BrushGizmos;

#[derive(Component)]
pub struct Brush;

#[derive(Component)]
pub struct BrushSize(pub usize);

#[derive(Component)]
pub struct BrushColor(pub Color);
