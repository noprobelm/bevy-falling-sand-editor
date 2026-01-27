#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(nonstandard_style, rustdoc::broken_intra_doc_links)]

mod camera;
mod config;
mod directive;
mod particles;
mod setup;
mod ui;

use avian2d::prelude::PhysicsDebugPlugin;
use avian2d::prelude::PhysicsGizmos;
use bevy_falling_sand::prelude::{
    FallingSandDebugPlugin, FallingSandPersistencePlugin, FallingSandPlugin,
};

use camera::CameraPlugin;
use config::*;
use directive::*;

use bevy::{log::LogPlugin, prelude::*, window::WindowMode};

use crate::particles::ParticlesPlugin;
use crate::setup::SetupPlugin;
use crate::ui::UiPlugin;
use crate::ui::console_capture_layer;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "Falling Sand Editor".into(),
                        mode: WindowMode::BorderlessFullscreen(MonitorSelection::Primary),
                        fit_canvas_to_parent: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(LogPlugin {
                    custom_layer: console_capture_layer,
                    ..default()
                }),
            SetupPlugin,
            ConfigPlugin::default(),
            DirectivePlugin,
            UiPlugin,
            CameraPlugin,
            ParticlesPlugin,
            FallingSandPlugin::default().with_chunk_size(64),
            FallingSandDebugPlugin,
            // We'll overwrite this path with the active world path as soon as the active world configuration is loaded.
            FallingSandPersistencePlugin::new("/tmp/bfs"),
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
