mod app_state;
mod brush;
mod camera;
mod cursor;
mod particles;
mod physics;
mod scenes;
mod ui;

use app_state::StatesPlugin;
use avian2d::prelude::PhysicsDebugPlugin;
use avian2d::prelude::PhysicsGizmos;
use bevy_falling_sand::prelude::{
    FallingSandDebugPlugin, FallingSandPersistencePlugin, FallingSandPlugin,
};
use bevy_framepace::FramepacePlugin;
use brush::*;
use camera::*;
use cursor::*;
use particles::*;
use physics::*;
use scenes::*;
use ui::*;

use bevy::{prelude::*, window::WindowMode};

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins.set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Falling Sand Editor".into(),
                    mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                    fit_canvas_to_parent: true,
                    ..default()
                }),
                ..default()
            }),
            FallingSandPlugin::default().with_chunk_size(64),
            FallingSandPersistencePlugin::new(get_config_dir()),
            FallingSandDebugPlugin,
            PhysicsDebugPlugin,
            FramepacePlugin,
            ParticleSetupPlugin,
            CursorPlugin,
            CameraPlugin,
            BrushPlugin,
            StatesPlugin,
            ScenesPlugin,
            UiPlugin,
            PhysicsPlugin,
        ))
        .insert_gizmo_config(
            PhysicsGizmos {
                collider_color: None,
                ..default()
            },
            GizmoConfig::default(),
        )
        .insert_resource(ClearColor(Color::srgba(0.17, 0.16, 0.15, 1.0)))
        .run();
}
