#![cfg_attr(docsrs, feature(doc_cfg))]
#![warn(nonstandard_style, rustdoc::broken_intra_doc_links)]

mod brush;
mod camera;
mod chunk_effects;
mod config;
mod console_command;
mod cursor;
mod debug;
mod exit;
mod frames;
mod game_of_life;
mod particles;
mod record;
mod setup;
mod ui;

use avian2d::prelude::PhysicsGizmos;
use bevy_falling_sand::prelude::{FallingSandPersistencePlugin, FallingSandPlugin};
use chunk_effects::ChunkEffectsPlugin;
use game_of_life::GameOfLifePlugin;

use brush::*;
use camera::CameraPlugin;
use config::*;
use console_command::*;
pub use cursor::*;
use debug::*;
use exit::*;
use frames::*;
#[cfg(feature = "dev")]
use record::*;

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
            ConfigPlugin::default(),
            SetupPlugin,
            ExitPlugin,
            CursorPlugin,
            CameraPlugin,
            UiPlugin,
            ConsoleCommandPlugin,
            BrushPlugin,
            FallingSandPlugin::default()
                .with_chunk_size(64)
                .with_map_size(32),
            ParticlesPlugin,
            ChunkEffectsPlugin,
            // This path is overwritten with the active world path as soon as the app configuration is loaded.
            FallingSandPersistencePlugin::new("/tmp/bfs"),
            #[cfg(feature = "dev")]
            RecordPlugin,
            DebugPlugin,
        ))
        .add_plugins((GameOfLifePlugin, FramesPlugin))
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
