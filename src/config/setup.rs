use std::{fs, path::PathBuf};

use crate::{
    app_state::{ConfigPathReadyState, ParticleTypesPathReadyState, WorldPathReadyState},
    config::{ConfigPath, InitConfig, ParticleTypesPath, SettingsPath, WorldPath},
};
use bevy::prelude::*;
use bevy_falling_sand::persistence::ParticlePersistenceConfig;
use bevy_persistent::{Persistent, StorageFormat};

macro_rules! setup_config_path {
    ($path:ident, $resource:ident, $state:ty) => {{
        let path = $path;
        move |mut commands: Commands, mut state: ResMut<NextState<$state>>| {
            if let Err(e) = fs::create_dir_all(&path) {
                let warning = format!("Failed to create directory {:?}: {}", path, e);
                warn!(warning);
                state.set(<$state>::Failed(warning));
                return;
            }
            commands.insert_resource($resource(path.clone()));
            state.set(<$state>::Complete);
        }
    }};
}

pub struct SetupPlugin {
    pub config_path: PathBuf,
    pub settings_path: PathBuf,
    pub world_path: PathBuf,
    pub particle_types_path: PathBuf,
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        let settings_path = self.settings_path.clone();
        let world_path = self.world_path.clone();
        let particle_types_path = self.particle_types_path.clone();

        app.add_systems(
            Startup,
            move |mut commands: Commands, mut state: ResMut<NextState<ConfigPathReadyState>>| {
                if let Err(e) = fs::create_dir_all(&config_path) {
                    let warning =
                        format!("Failed to create config directory {:?}: {}", config_path, e);
                    warn!(warning);
                    state.set(ConfigPathReadyState::Failed(warning));
                    return;
                }
                commands.insert_resource(ConfigPath(config_path.clone()));
                state.set(ConfigPathReadyState::Complete)
            },
        )
        .add_systems(
            OnEnter(ConfigPathReadyState::Complete),
            (
                setup_config_path!(settings_path, SettingsPath, ConfigPathReadyState),
                {
                    let path = world_path;
                    move |mut commands: Commands,
                          mut state: ResMut<NextState<WorldPathReadyState>>| {
                        if let Err(e) = fs::create_dir_all(&path) {
                            let warning = format!("Failed to create directory {:?}: {}", path, e);
                            warn!(warning);
                            state.set(<WorldPathReadyState>::Failed(warning));
                            return;
                        }
                        commands.insert_resource(WorldPath(path.clone()));
                        state.set(<WorldPathReadyState>::Complete);
                    }
                },
                setup_config_path!(
                    particle_types_path,
                    ParticleTypesPath,
                    ParticleTypesPathReadyState
                ),
            ),
        )
        .add_systems(OnEnter(ConfigPathReadyState::Complete), load_init_config)
        .add_systems(
            OnEnter(WorldPathReadyState::Complete),
            update_bevy_falling_sand_persistence_path,
        );
    }
}

fn update_bevy_falling_sand_persistence_path(
    world_path: Res<WorldPath>,
    mut persistence_config: ResMut<ParticlePersistenceConfig>,
) {
    persistence_config.save_path = world_path.0.clone();
}

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
