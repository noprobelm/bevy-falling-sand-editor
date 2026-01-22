pub use base::*;
use bevy::app::Plugin;
pub use settings::*;
pub use world::*;

pub(super) mod base {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};
    use std::path::PathBuf;

    #[derive(
        Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect,
    )]
    pub struct ConfigPath(pub PathBuf);

    #[derive(
        Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect,
    )]
    pub struct ActiveSettingsPath(pub PathBuf);

    #[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
    pub struct ActiveWorldPath(pub PathBuf);

    #[derive(
        Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect,
    )]
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
}

pub(super) mod world {
    use std::path::PathBuf;

    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    use crate::camera::ZoomSpeed;

    #[derive(Resource, Clone, Debug, Serialize, Deserialize)]
    pub struct WorldConfig {
        pub camera: CameraConfig,
        pub particle_types_file: PathBuf,
    }

    impl Default for WorldConfig {
        fn default() -> Self {
            Self {
                camera: CameraConfig::default(),
                particle_types_file: PathBuf::from("particles.scn.ron"),
            }
        }
    }

    #[derive(Resource, Clone, Debug, Serialize, Deserialize)]
    pub struct CameraConfig {
        pub scale: f32,
        pub zoom_speed: ZoomSpeed,
        pub position: Vec2,
    }

    impl Default for CameraConfig {
        fn default() -> Self {
            let scale = 0.25;
            Self {
                scale,
                zoom_speed: ZoomSpeed(8.0),
                position: Vec2::ZERO,
            }
        }
    }

    impl CameraConfig {
        pub fn from_query(query: &Single<(&Transform, &ZoomSpeed, &Projection)>) -> Self {
            let scale = match query.2 {
                Projection::Orthographic(ortho) => ortho.scale,
                _ => unreachable!(),
            };
            let position = Vec2::new(query.0.translation.x, query.0.translation.y);
            Self {
                scale,
                zoom_speed: query.1.clone(),
                position,
            }
        }
    }
}

pub(super) mod settings {
    use bevy::prelude::*;
    use serde::{Deserialize, Serialize};

    #[derive(Resource, Clone, Default, Debug, Serialize, Deserialize)]
    pub struct SettingsConfig {
        pub camera: CameraKeyBindings,
        pub console: ConsoleKeyBindings,
    }

    #[derive(Resource, Clone, Debug, Serialize, Deserialize)]
    pub struct ConsoleKeyBindings {
        pub toggle_information_area: KeyCode,
    }

    impl Default for ConsoleKeyBindings {
        fn default() -> Self {
            Self {
                toggle_information_area: KeyCode::Backquote,
            }
        }
    }

    #[derive(Resource, Clone, Debug, Serialize, Deserialize)]
    pub struct CameraKeyBindings {
        pub pan_camera_up: KeyCode,
        pub pan_camera_left: KeyCode,
        pub pan_camera_down: KeyCode,
        pub pan_camera_right: KeyCode,
    }

    impl Default for CameraKeyBindings {
        fn default() -> Self {
            CameraKeyBindings {
                pan_camera_up: KeyCode::KeyW,
                pan_camera_left: KeyCode::KeyA,
                pan_camera_down: KeyCode::KeyS,
                pan_camera_right: KeyCode::KeyD,
            }
        }
    }
}
