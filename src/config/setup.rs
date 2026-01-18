use std::{fs, path::PathBuf};

use crate::config::{
    ActiveWorldPath, ConfigPath, ConfigPathState, InitConfig, InitConfigState, WorldConfig,
    WorldConfigState,
};
use bevy::prelude::*;
use bevy_falling_sand::persistence::ParticlePersistenceConfig;
use bevy_persistent::{Persistent, StorageFormat};

pub struct SetupPlugin {
    pub config_path: PathBuf,
    pub settings_path: PathBuf,
    pub world_path: PathBuf,
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        let settings_path = self.settings_path.clone();
        let world_path = self.world_path.clone();

        // Step 1: Create config path on startup
        app.add_systems(
            Startup,
            move |mut commands: Commands, mut state: ResMut<NextState<ConfigPathState>>| {
                fs::create_dir_all(&config_path).expect(&format!(
                    "Failed to create config directory {:?}",
                    config_path
                ));
                commands.insert_resource(ConfigPath(config_path.clone()));
                state.set(ConfigPathState::Initialized)
            },
        );

        // Step 2: Create subpaths and load init.toml
        app.add_systems(OnEnter(ConfigPathState::Initialized), {
            let settings_path = settings_path.clone();
            let world_path = world_path.clone();
            move |mut commands: Commands,
                  config_path: Res<ConfigPath>,
                  mut state: ResMut<NextState<InitConfigState>>| {
                fs::create_dir_all(&settings_path).expect(&format!(
                    "Failed to create settings directory {:?}",
                    settings_path
                ));

                fs::create_dir_all(&world_path).expect(&format!(
                    "Failed to create world directory {:?}",
                    world_path
                ));

                commands.insert_resource(
                    Persistent::<InitConfig>::builder()
                        .name("init")
                        .format(StorageFormat::Toml)
                        .path(config_path.0.clone().join("init.toml"))
                        .default(InitConfig::default())
                        .build()
                        .expect("Failed to load init.toml"),
                );

                state.set(InitConfigState::Initialized);
            }
        });

        // Step 3: Create active world path and load world.toml
        app.add_systems(
            OnEnter(InitConfigState::Initialized),
            (load_world_config, set_bevy_falling_sand_persistence_path).chain(),
        );
    }
}

fn load_world_config(
    mut commands: Commands,
    config_path: Res<ConfigPath>,
    init_config: Res<Persistent<InitConfig>>,
    mut state: ResMut<NextState<WorldConfigState>>,
) {
    let active_world_path = config_path
        .0
        .join("world")
        .join(init_config.get().active_world_path());

    fs::create_dir_all(&active_world_path).expect(&format!(
        "Failed to create active world directory {:?}",
        active_world_path
    ));

    let data_path = active_world_path.join("data");
    fs::create_dir_all(&data_path)
        .expect(&format!("Failed to create data directory {:?}", data_path));

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

    state.set(WorldConfigState::Initialized);
}

fn set_bevy_falling_sand_persistence_path(
    active_world_path: Res<ActiveWorldPath>,
    mut persistence_config: ResMut<ParticlePersistenceConfig>,
) {
    persistence_config.save_path = active_world_path.0.join("data");
}
