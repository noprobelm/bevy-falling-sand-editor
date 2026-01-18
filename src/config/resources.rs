use std::path::PathBuf;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

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
