use std::{fs, path::PathBuf};

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::app_state::ConfigPathLoadedState;

pub struct ConfigPlugin {
    pub config_path: PathBuf,
}

impl Default for ConfigPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self { config_path }
    }
}

impl Plugin for ConfigPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        app.add_systems(
            Startup,
            move |mut commands: Commands, mut state: ResMut<NextState<ConfigPathLoadedState>>| {
                if let Err(e) = fs::create_dir_all(&config_path) {
                    warn!("Failed to create config directory {:?}: {}", config_path, e);
                    return;
                }
                commands.insert_resource(ConfigPath(config_path.clone()));
                state.set(ConfigPathLoadedState::Complete)
            },
        )
        .add_systems(
            OnEnter(ConfigPathLoadedState::Complete),
            (setup_settings_path, setup_world_path),
        );
    }
}

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ConfigPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct SettingsPath(pub PathBuf);

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct WorldPath(pub PathBuf);

fn setup_settings_path(mut commands: Commands, config_path: Res<ConfigPath>) {
    commands.insert_resource(SettingsPath(config_path.0.join("settings.ron")));
}

fn setup_world_path(mut commands: Commands, config_path: Res<ConfigPath>) {
    commands.insert_resource(SettingsPath(config_path.0.join("world")));
}
