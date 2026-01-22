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
use bevy_falling_sand::core::Particle;
use bevy_falling_sand::core::SpawnParticleSignal;
use bevy_falling_sand::prelude::{
    FallingSandDebugPlugin, FallingSandPersistencePlugin, FallingSandPlugin,
};

use camera::*;
use config::*;
use directive::*;
use particles::*;
use setup::*;

use bevy::{log::LogPlugin, prelude::*, window::WindowMode};

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
            SetupPlugin::default(),
            ConfigPlugin,
            DirectivePlugin,
            UiPlugin,
            CameraPlugin,
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
        .add_systems(Update, spawn_particles)
        .run();
}

fn spawn_particles(mut msgw_spawn_particle: MessageWriter<SpawnParticleSignal>) {
    for y in 0..10 {
        msgw_spawn_particle.write(SpawnParticleSignal::new(
            Particle::new("Water"),
            IVec2::new(0, y),
        ));
    }
}
