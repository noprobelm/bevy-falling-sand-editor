use bevy::prelude::*;

use crate::brush::{
    components::{Brush, BrushColor, BrushSize},
    gizmos::BrushGizmos,
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.insert_gizmo_config(
            BrushGizmos,
            GizmoConfig {
                enabled: true,
                ..default()
            },
        )
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    commands.spawn((
        Brush,
        BrushSize(2),
        BrushColor(Color::Srgba(Srgba::new(1., 1., 1., 0.3))),
    ));
}
