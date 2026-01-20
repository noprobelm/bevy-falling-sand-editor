use std::{fs, path::PathBuf};

use crate::{
    camera::{MainCamera, ZoomTarget},
    config::{
        ActiveWorldPath, CameraAction, ConfigPath, InitConfig, ParticleTypesFile, SettingsConfig,
        WorldConfig,
    },
};
use bevy::prelude::*;
use bevy_falling_sand::{
    core::ChunkLoader, persistence::ParticlePersistenceConfig, prelude::LoadParticleTypesSignal,
};
use bevy_persistent::{Persistent, StorageFormat};
use leafwing_input_manager::prelude::InputMap;

const SETTINGS_PATH: &str = "settings";
const WORLD_PATH: &str = "world";
const DATA_PATH: &str = "data";

const INIT_TOML_FILE: &str = "init.toml";
const WORLD_TOML_FILE: &str = "world.toml";
const SETTINGS_TOML_FILE: &str = "settings.toml";

// JAB TODO: A temporary solution until we have the editor up and running. It's currently helpful
// to have some default particles to fall back to.
const DEFAULT_PARTICLES_ASSET: &str = "assets/particles/particles.scn.ron";

/// Loads the application configuration in several stages:
/// 1. Initial Configuration Setup
///    a. Ensure the base configuration path and subpaths are set up
///    b. Load `init.toml` as a `Persistent<InitConfig>` resource
///    c. From `init.toml`, ensure the active world path is set up and insert the `ActiveWorldPath`
///    resource.
///    d. Update the `ParticlePersistenceConfig` to point to our active world data path for saving
///    chunks/particle spatial information
/// 2. Settings setup
///    a. Load `settings.toml` as a `Persistent<SettingsConfig>` resource
/// 3. World Setup
///    a. Load the active world's `world.toml` file as a `Persistent<WorldConfig>` resource
///    b. Load the active world's particle types file using the `LoadParticleTypesSignal`. Also,
///    insert the `ParticleTypesFile` resource.
///    c. Load the camera configuration from the `WorldConfig` and `SettingsConfig` resources.
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
                load_settings_base_path,
                load_world_base_path,
                // Load init.toml for the startup configuration
                load_init_config_file,
                load_settings_file,
                // From init.toml, load the necessary config subpaths
                load_active_world_path,
                // Configure bfs persistence to update from fallback path to active world path
                configure_bfs_persistence,
                // Load world.toml for the active world configuration
                load_world_config_file,
                // Load the particle types from `world.toml`
                load_world_particle_types_file,
                // Spawn the `MainCamera` entity
                spawn_camera,
                // Load camera state information from `world.toml`
                load_camera_world_state,
                // Load camera settings from `settings.toml`
                load_camera_settings,
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
fn load_init_config_file(mut commands: Commands, config_path: Res<ConfigPath>) {
    commands.insert_resource(
        Persistent::<InitConfig>::builder()
            .name("init")
            .format(StorageFormat::Toml)
            .path(config_path.0.clone().join(INIT_TOML_FILE))
            .default(InitConfig::default())
            .build()
            .expect("Failed to load {INIT_TOML_FILE}"),
    );
}

/// Try to create the `settings` subpath.
///
///# Panics
///
/// Panics if path creation fails.
fn load_settings_base_path(config_path: Res<ConfigPath>) {
    let settings_path = config_path.0.clone().join(SETTINGS_PATH);
    fs::create_dir_all(&settings_path)
        .unwrap_or_else(|_| panic!("Failed to create settings path {:?}", settings_path));
}

/// Try to create the `world` subpath.
///
/// # Panics
///
/// Panics if path creation fails.
fn load_world_base_path(config_path: Res<ConfigPath>) {
    let world_path = config_path.0.clone().join(WORLD_PATH);
    fs::create_dir_all(&world_path)
        .unwrap_or_else(|_| panic!("Failed to create world path {:?}", world_path));
}

/// Try to load the config data for this world.
///
/// # Panics
///
/// Panics if we fail to create any critical path or load the `world.toml` file.
fn load_active_world_path(
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

    commands.insert_resource(ActiveWorldPath(active_world_path));
}

fn configure_bfs_persistence(
    active_world_path: Res<ActiveWorldPath>,
    mut persistence_config: ResMut<ParticlePersistenceConfig>,
) {
    persistence_config.save_path = active_world_path.0.join(DATA_PATH);
}

// Try to load the `settings.toml` file
fn load_settings_file(mut commands: Commands, config_path: Res<ConfigPath>) {
    commands.insert_resource(
        Persistent::<SettingsConfig>::builder()
            .name("settings")
            .format(StorageFormat::Toml)
            .path(config_path.0.join(SETTINGS_TOML_FILE))
            .default(SettingsConfig::default())
            .build()
            .expect("Failed to load {SETTINGS_TOML_FILE}"),
    );
}

// Try to load the `world.toml` file
fn load_world_config_file(mut commands: Commands, active_world_path: Res<ActiveWorldPath>) {
    commands.insert_resource(
        Persistent::<WorldConfig>::builder()
            .name("world_meta")
            .format(StorageFormat::Toml)
            .path(active_world_path.0.join(WORLD_TOML_FILE))
            .default(WorldConfig::default())
            .build()
            .expect("Failed to load {WORLD_TOML_FILE}"),
    );
}

/// Try to load the particle types file
fn load_world_particle_types_file(
    mut commands: Commands,
    active_world_path: Res<ActiveWorldPath>,
    world_config: Res<Persistent<WorldConfig>>,
    mut msgw_load_particles_scene: MessageWriter<LoadParticleTypesSignal>,
) {
    let particle_types_file = active_world_path
        .0
        .join(world_config.get().particle_types_file.clone());

    if !particle_types_file.exists() {
        // JAB TODO: A temporary solution until we have the editor up and running. It's currently helpful
        // to have some default particles to fall back to.
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

    commands.insert_resource(ParticleTypesFile(
        active_world_path.0.join(particle_types_file.clone()),
    ));

    msgw_load_particles_scene.write(LoadParticleTypesSignal(particle_types_file.clone()));
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(MainCamera);
}

fn load_camera_world_state(
    mut commands: Commands,
    camera: Single<(Entity, &MainCamera)>,
    world_config: Res<Persistent<WorldConfig>>,
) {
    commands.insert_resource(world_config.camera.clone());
    commands.entity(camera.0).insert((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: world_config.get().camera.scale,
            ..OrthographicProjection::default_2d()
        }),
        ChunkLoader,
        ZoomTarget {
            target_scale: world_config.get().camera.scale,
            current_scale: world_config.get().camera.scale,
        },
        world_config.get().camera.zoom_speed.clone(),
    ));
}

fn load_camera_settings(
    mut commands: Commands,
    camera: Single<(Entity, &MainCamera)>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let mut input_map = InputMap::default();
    input_map.insert(
        CameraAction::PanUp,
        settings_config.get().camera.pan_camera_up,
    );
    input_map.insert(
        CameraAction::PanLeft,
        settings_config.get().camera.pan_camera_left,
    );
    input_map.insert(
        CameraAction::PanDown,
        settings_config.get().camera.pan_camera_down,
    );
    input_map.insert(
        CameraAction::PanRight,
        settings_config.get().camera.pan_camera_right,
    );

    commands.entity(camera.0).insert(input_map);
}
