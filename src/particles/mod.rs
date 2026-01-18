use std::path::PathBuf;

use bevy::prelude::*;
use bevy_falling_sand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{ActiveWorldPath, ParticleTypesConfigState, WorldConfigState};

const DEFAULT_PARTICLES_ASSET: &str = "assets/particles/particles.scn.ron";

#[derive(Resource, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize, Reflect)]
pub struct ParticleTypesInitFile(pub PathBuf);

pub struct ParticlesPlugin {
    pub particle_types_init_file: PathBuf,
}

impl Default for ParticlesPlugin {
    fn default() -> Self {
        Self {
            particle_types_init_file: PathBuf::from("particles.scn.ron"),
        }
    }
}

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        let particle_types_init_file = self.particle_types_init_file.clone();
        app.add_systems(
            OnEnter(WorldConfigState::Initialized),
            move |mut commands: Commands,
                  active_world_path: Res<ActiveWorldPath>,
                  mut msgw_load_particles_scene: MessageWriter<LoadParticleTypesSignal>| {
                let particle_types_file =
                    active_world_path.0.join(particle_types_init_file.clone());

                // Copy default particles file if needed
                if !particle_types_file.exists() {
                    let default_path = PathBuf::from(DEFAULT_PARTICLES_ASSET);
                    if default_path.exists() {
                        if let Err(e) = std::fs::copy(&default_path, &particle_types_file) {
                            warn!(
                                "Failed to copy default particles file to {:?}: {}",
                                particle_types_file, e
                            );
                            return;
                        }
                        info!(
                            "Copied default particles file to {:?}",
                            particle_types_file
                        );
                    } else {
                        warn!("Default particles file not found at {:?}", default_path);
                    }
                }

                commands.insert_resource(ParticleTypesInitFile(particle_types_file.clone()));

                // Load particle types
                msgw_load_particles_scene.write(LoadParticleTypesSignal(particle_types_file));
            },
        );

        app.add_systems(
            Update,
            msgr_particle_types_loaded.run_if(in_state(ParticleTypesConfigState::Initializing)),
        );
    }
}

fn msgr_particle_types_loaded(
    mut msgr_particle_types_loaded: MessageReader<ParticleTypesLoadedSignal>,
    mut next_state: ResMut<NextState<ParticleTypesConfigState>>,
) {
    msgr_particle_types_loaded.read().for_each(|msg| {
        info!(
            "Loaded particle types from file: {}",
            msg.0.to_string_lossy()
        );
        next_state.set(ParticleTypesConfigState::Initialized);
    });
}
