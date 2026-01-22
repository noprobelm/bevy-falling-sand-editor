//! ## Overview
//!
//! All application setup processes stem from this module via `SetupPlugin`.
//!
//! Application setup is a multi-phased process. The `SetupSystems` `SystemSet` is used to control the
//! order of our `Startup` systems.
//!
//! - `SetupSystems::Configuration` deals with accessing the configuration path and subpaths, reading
//!   the `initl.toml` file for startup information, loading the active world path, and loading the
//!   `settings.toml` file.
//! - `SetupSystems::Console` deals with adding directives to the `DirectiveRegistry` reource
//!   from the `directives` module
//! - `SetupSystems::Camera` spawns the `MainCamera` entity and loads its settings and world state
//!   data from the previous session.
//! - `SetupSystems::Particles` loads the particle types file defined in `init.toml`
//!
//! This module makes use of [`bevy-persistent`] for maintaining application state and
//! configuration files between sessions.
//!
//! ## Persistent init config
//!
//! The `Persistent<InitConfig>` resource holds information about the location of the initialization
//! files for the `Persistent<WorldConfig>` and `Persistent<SettingsConfig>` resources. It is
//! based on an existing `init.toml`, or creates a default version of this file if there isn't one
//! found in the base config path.
//!
//! ## Persistent world config
//!
//! After the active world's config path is ensured, the `Persistent<WorldConfig>` resource is
//! loaded from the `world.toml` file. This resource resource holds state information specific
//! to the current world, such as the particle types and the camera's last position
//!
//! ## Persistent settings config
//! The `Persistent<SettingsConfig>` resource manages the application-wide settings state between
//! each session. These settings apply to all worlds.
use std::path::PathBuf;

use bevy::prelude::*;

use crate::{
    camera::CameraSetupPlugin, config::ConfigSetupPlugin, particles::ParticlesSetupPlugin,
    ui::ConsoleSetupPlugin,
};

pub struct SetupPlugin {
    pub config_path: PathBuf,
}

impl Default for SetupPlugin {
    fn default() -> Self {
        let config_path = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
            .join(".config")
            .join("bevy_falling_sand");
        Self { config_path }
    }
}

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        let config_path = self.config_path.clone();
        app.add_plugins((
            ConfigSetupPlugin { config_path },
            ConsoleSetupPlugin,
            CameraSetupPlugin,
            ParticlesSetupPlugin,
        ))
        .configure_sets(
            Startup,
            (
                SetupSystems::Configuration,
                SetupSystems::Console,
                SetupSystems::Camera,
                SetupSystems::Particles,
            )
                .chain(),
        );
    }
}

/// System set for application initialization systems.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetupSystems {
    Configuration,
    Console,
    Camera,
    Particles,
}
