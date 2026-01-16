use std::{fs, path::PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app_state::{ConfigPathReadyState, ParticleTypesPathReadyState, WorldPathReadyState};

pub struct ConfigStartupPlugin {
    pub config_path: PathBuf,
}

impl Default for ConfigStartupPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self { config_path }
    }
}

impl Plugin for ConfigStartupPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
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
                setup_settings_path,
                setup_world_path,
                setup_particle_types_path,
            ),
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

macro_rules! setup_path_system {
    ($fn_name:ident, $resource:ident, $state:ty, $($path_segment:expr),+) => {
        fn $fn_name(
            mut commands: Commands,
            config_path: Res<ConfigPath>,
            mut state: ResMut<NextState<$state>>,
        ) {
            let path = config_path.0 $(.join($path_segment))+;
            if let Err(e) = fs::create_dir_all(&path) {
                let warning = format!("Failed to create config directory {:?}: {}", path, e);
                warn!(warning);
                state.set(<$state>::Failed(warning));
                return;
            }
            commands.insert_resource($resource(path));
            state.set(<$state>::Complete);
        }
    };
}

setup_path_system!(
    setup_settings_path,
    SettingsPath,
    ConfigPathReadyState,
    "settings.ron"
);
setup_path_system!(setup_world_path, WorldPath, WorldPathReadyState, "world");
setup_path_system!(
    setup_particle_types_path,
    ParticleTypesPath,
    ParticleTypesPathReadyState,
    "particles",
    "particles.scn.ron"
);
