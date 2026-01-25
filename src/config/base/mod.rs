use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ConfigPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ActiveSettingsPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct ActiveWorldPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ParticleTypesFile(pub PathBuf);

#[derive(Resource, Clone, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
pub struct InitConfig {
    pub settings_init_file: PathBuf,
    pub active_world_path: PathBuf,
}

impl Default for InitConfig {
    fn default() -> Self {
        Self {
            settings_init_file: PathBuf::from("settings.toml"),
            active_world_path: PathBuf::from("default"),
        }
    }
}

impl InitConfig {
    pub fn active_world_path(&self) -> &PathBuf {
        &self.active_world_path
    }

    pub fn settings_init_file(&self) -> &PathBuf {
        &self.settings_init_file
    }
}
