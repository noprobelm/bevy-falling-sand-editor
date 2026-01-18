mod camera;
mod config;
mod particles;
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
use particles::*;

use bevy::{prelude::*, window::WindowMode};

use crate::ui::UiPlugin;

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
            UiPlugin,
            ConfigPlugin::default(),
            CameraPlugin,
            ParticlesPlugin::default(),
            FallingSandPlugin::default().with_chunk_size(64),
            FallingSandDebugPlugin,
            // Fall back to /tmp until `WorldPathReady` state indicates `Complete`
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
