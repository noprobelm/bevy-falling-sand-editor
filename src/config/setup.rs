use std::{fs, path::PathBuf};

use crate::setup::SetupSystems;
use bevy::prelude::*;
use bevy_falling_sand::prelude::ParticlePersistenceConfig;
use bevy_persistent::{Persistent, StorageFormat};

use super::{
    ActiveSettingsPath, ActiveWorldPath, ConfigPath, InitConfig, SettingsConfig, WorldConfig,
};

const WORLD_PATH: &str = "world";
const DATA_PATH: &str = "data";

const INIT_TOML_FILE: &str = "init.toml";
const WORLD_TOML_FILE: &str = "world.toml";

pub struct ConfigSetupPlugin {
    pub config_path: PathBuf,
}

impl Plugin for ConfigSetupPlugin {
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
                // Setup world subpath
                load_world_base_path,
                // Load `init.toml` for the initialization info
                load_init_config_file,
                // Load `settings.toml` for the settings info
                load_settings_config_file,
                // From init.toml, load the necessary config subpaths
                load_active_world_path,
                // Load world.toml for the active world configuration
                load_world_config_file,
                // Configure bfs persistence to update from fallback path to active world path
                configure_bfs_persistence,
            )
                .in_set(SetupSystems::Configuration)
                .chain(),
        );
    }
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

/// Load init.toml to an `InitConfig` resource.
///
/// # Panics
///
/// Panics if `init.toml` fails to load or be created
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

/// Try to load the `settings.toml` file
///
/// # Panics
///
/// Panics if the settings file fails to load or be created
fn load_settings_config_file(
    mut commands: Commands,
    config_path: Res<ConfigPath>,
    init_config: Res<Persistent<InitConfig>>,
) {
    let settings_file_path = config_path.0.join(init_config.get().settings_init_file());

    commands.insert_resource(
        Persistent::<SettingsConfig>::builder()
            .name("settings")
            .format(StorageFormat::Toml)
            .path(&settings_file_path)
            .default(SettingsConfig::default())
            .build()
            .expect("Failed to load {settings_file_path}"),
    );
    commands.insert_resource(ActiveSettingsPath(settings_file_path));
}

/// Try to load the config data for this world.
///
/// # Panics
///
/// Panics if the active world path or its data subpath fail to be loaded or created.
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

/// Try to load the `world.toml` file
///
/// # Panics
///
/// Panics if the `world.toml` file fails to load or be created.
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

/// Set `bevy_falling_sand` persistence to use the active world's data path
fn configure_bfs_persistence(
    active_world_path: Res<ActiveWorldPath>,
    mut persistence_config: ResMut<ParticlePersistenceConfig>,
) {
    persistence_config.save_path = active_world_path.0.join(DATA_PATH);
}
