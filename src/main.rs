mod app_state;
mod startup;
mod ui;

use avian2d::prelude::PhysicsDebugPlugin;
use avian2d::prelude::PhysicsGizmos;
use bevy_falling_sand::prelude::{
    FallingSandDebugPlugin, FallingSandPersistencePlugin, FallingSandPlugin,
};

use app_state::*;
use startup::*;
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
            AppStatePlugin,
            StartupPlugin::default(),
            FallingSandPlugin::default().with_chunk_size(64),
            // Fall back to /tmp until `WorldPathReady` state indicates `Complete`
            FallingSandPersistencePlugin::new("/tmp/bfs_fallback"),
            FallingSandDebugPlugin,
            PhysicsDebugPlugin,
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
