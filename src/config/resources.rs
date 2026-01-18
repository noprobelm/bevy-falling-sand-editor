use std::{fs, path::PathBuf};

use crate::app_state::{ConfigPathReadyState, ParticleTypesPathReadyState, WorldPathReadyState};
use bevy::prelude::*;
use bevy_falling_sand::persistence::ParticlePersistenceConfig;
use bevy_persistent::{Persistent, StorageFormat};
use serde::{Deserialize, Serialize};

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

pub struct ResourcesPlugin {
    pub config_path: PathBuf,
    pub settings_path: PathBuf,
    pub world_path: PathBuf,
    pub particle_types_path: PathBuf,
}

impl Plugin for ResourcesPlugin {
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
                setup_config_path!(world_path, WorldPath, WorldPathReadyState),
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

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct ConfigPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct ParticleTypesPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct SettingsPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct WorldPath(pub PathBuf);

#[derive(Resource, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
#[reflect(Resource)]
pub struct InitConfig {
    settings_init_file: PathBuf,
    particle_types_init_file: PathBuf,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            settings_init_file: PathBuf::from("settings.toml"),
            particle_types_init_file: PathBuf::from("particles.scn.ron"),
        }
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
