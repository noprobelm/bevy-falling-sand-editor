use std::{fs, path::PathBuf};

use bevy::prelude::*;
use bevy_falling_sand::persistence::ParticlePersistenceConfig;
use serde::{Deserialize, Serialize};

use crate::app_state::{ConfigPathReadyState, ParticleTypesPathReadyState, WorldPathReadyState};

macro_rules! setup_path_system {
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

pub struct ConfigStartupPlugin {
    pub config_path: PathBuf,
    pub settings_path: PathBuf,
    pub world_path: PathBuf,
    pub particle_types_path: PathBuf,
}

impl Default for ConfigStartupPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self {
            settings_path: config_path.join("settings"),
            world_path: config_path.join("world"),
            particle_types_path: config_path.join("particles"),
            config_path,
        }
    }
}

impl Plugin for ConfigStartupPlugin {
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
                setup_path_system!(settings_path, SettingsPath, ConfigPathReadyState),
                setup_path_system!(world_path, WorldPath, WorldPathReadyState),
                setup_path_system!(
                    particle_types_path,
                    ParticleTypesPath,
                    ParticleTypesPathReadyState
                ),
            ),
        )
        .add_systems(
            OnEnter(WorldPathReadyState::Complete),
            |world_path: Res<WorldPath>, mut config: ResMut<ParticlePersistenceConfig>| {
                config.save_path = world_path.0.clone();
            },
        );
    }
}

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ConfigPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ParticleTypesPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct SettingsPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct WorldPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ParticleTypesInitFile(pub PathBuf);
