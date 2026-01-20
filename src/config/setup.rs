use std::{fs, path::PathBuf};

use crate::{
    camera::{MainCamera, ZoomTarget},
    config::{ActiveWorldPath, ConfigPath, InitConfig, WorldConfig},
};
use bevy::prelude::*;
use bevy_falling_sand::{
    core::ChunkLoader, persistence::ParticlePersistenceConfig, prelude::LoadParticleTypesSignal,
};
use bevy_persistent::{Persistent, StorageFormat};

const SETTINGS_PATH: &str = "settings";
const WORLD_PATH: &str = "world";
const DATA_PATH: &str = "data";
const PARTICLE_TYPES_FILE: &str = "particles.scn.ron";
const DEFAULT_PARTICLES_ASSET: &str = "assets/particles/particles.scn.ron";

pub struct SetupPlugin {
    pub config_path: PathBuf,
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();

        // Create base config path
        app.add_systems(
            Startup,
            (
                move |mut commands: Commands| {
                    fs::create_dir_all(&config_path).unwrap_or_else(|_| {
                        panic!("Failed to create config directory {:?}", config_path)
                    });
                    commands.insert_resource(ConfigPath(config_path.clone()));
                },
                // Setup subpaths in base config path
                setup_settings_path,
                setup_world_path,
                // Load init.toml for startup information
                load_init_config,
                // From init.toml, load the necessary config subpaths
                setup_active_world_path,
                setup_particle_types_path,
                // Load the particle types file from the active world path
                load_particle_types_file,
                // Configure bfs persistence to update from fallback path to active world path
                configure_bfs_persistence,
                // Load camera settings from init.toml. This has the `ChunkLoader` component, so it
                // must be ran after we configure the `ParticlePersistenceConfig` resource.
                load_camera_config,
            )
                .chain(),
        );
    }
}

/// Load init.toml to an `InitConfig` resource.
///
/// # Panics
///
/// Panics if `init.toml` fails to load.
fn load_init_config(mut commands: Commands, config_path: Res<ConfigPath>) {
    commands.insert_resource(
        Persistent::<InitConfig>::builder()
            .name("init")
            .format(StorageFormat::Toml)
            .path(config_path.0.clone().join("init.toml"))
            .default(InitConfig::default())
            .build()
            .expect("Failed to load init.toml"),
    );
}

/// Try to create the `settings` subpath.
///
///# Panics
///
/// Panics if path creation fails.
fn setup_settings_path(config_path: Res<ConfigPath>) {
    let settings_path = config_path.0.clone().join(SETTINGS_PATH);
    fs::create_dir_all(&settings_path)
        .unwrap_or_else(|_| panic!("Failed to create settings path {:?}", settings_path));
}

/// Try to create the `world` subpath.
///
/// # Panics
///
/// Panics if path creation fails.
fn setup_world_path(config_path: Res<ConfigPath>) {
    let world_path = config_path.0.clone().join(WORLD_PATH);
    fs::create_dir_all(&world_path)
        .unwrap_or_else(|_| panic!("Failed to create world path {:?}", world_path));
}

/// Try to load the config data for this world.
///
/// # Panics
///
/// Panics if we fail to create any critical path or load the `world.toml` file.
fn setup_active_world_path(
    mut commands: Commands,
    config_path: Res<ConfigPath>,
    init_config: Res<Persistent<InitConfig>>,
) {
    let active_world_path = config_path
        .0
        .join(WORLD_PATH)
        .join(init_config.get().active_world_path());

    fs::create_dir_all(&active_world_path).unwrap_or_else(|_| {
        panic!(
            "Failed to create active world directory {:?}",
            active_world_path
        )
    });

    let data_path = active_world_path.join(DATA_PATH);
    fs::create_dir_all(&data_path)
        .unwrap_or_else(|_| panic!("Failed to create data directory {:?}", data_path));

    commands.insert_resource(ActiveWorldPath(active_world_path.clone()));

    commands.insert_resource(
        Persistent::<WorldConfig>::builder()
            .name("world_meta")
            .format(StorageFormat::Toml)
            .path(active_world_path.join("world.toml"))
            .default(WorldConfig::default())
            .build()
            .expect("Failed to load world.toml"),
    );
}

/// Try to load the particle types set for this world.
fn setup_particle_types_path(active_world_path: Res<ActiveWorldPath>) {
    let particle_types_file = active_world_path.0.join(PARTICLE_TYPES_FILE);
    if !particle_types_file.exists() {
        let default_path = PathBuf::from(DEFAULT_PARTICLES_ASSET);
        if default_path.exists() {
            if let Err(e) = std::fs::copy(&default_path, &particle_types_file) {
                warn!(
                    "Failed to copy default particles file to {:?}: {}",
                    particle_types_file, e
                );
                return;
            }
            info!("Copied default particles file to {:?}", particle_types_file);
        } else {
            warn!("Default particles file not found at {:?}", default_path);
        }
    }
}

fn load_particle_types_file(
    active_world_path: Res<ActiveWorldPath>,
    mut msgw_load_particles_scene: MessageWriter<LoadParticleTypesSignal>,
) {
    let particle_types_file = active_world_path.0.join(PARTICLE_TYPES_FILE);
    msgw_load_particles_scene.write(LoadParticleTypesSignal(particle_types_file));
}

fn configure_bfs_persistence(
    active_world_path: Res<ActiveWorldPath>,
    mut persistence_config: ResMut<ParticlePersistenceConfig>,
) {
    persistence_config.save_path = active_world_path.0.join(DATA_PATH);
}

fn load_camera_config(mut commands: Commands, world_config: Res<Persistent<WorldConfig>>) {
    commands.insert_resource(world_config.camera.clone());
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: world_config.get().camera.scale,
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        ChunkLoader,
        ZoomTarget {
            target_scale: world_config.get().camera.scale,
            current_scale: world_config.get().camera.scale,
        },
        world_config.get().camera.zoom_speed.clone(),
    ));
}
