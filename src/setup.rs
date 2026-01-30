use bevy::prelude::*;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Startup,
            (
                SetupSystems::Configuration,
                SetupSystems::Ui,
                SetupSystems::Camera,
                SetupSystems::Particles,
                SetupSystems::Brush,
            )
                .chain(),
        );
    }
}

/// System set for application initialization systems.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum SetupSystems {
    Configuration,
    Ui,
    Camera,
    Particles,
    Brush,
}
