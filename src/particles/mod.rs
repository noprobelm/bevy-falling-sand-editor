use std::path::PathBuf;

use bevy::prelude::*;
use bevy_falling_sand::prelude::*;
use serde::{Deserialize, Serialize};

use crate::config::{
    ParticleTypesInitFileReadyState, ParticleTypesLoadedState, ParticleTypesPath,
    ParticleTypesPathReadyState,
};

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
            OnEnter(ParticleTypesPathReadyState::Complete),
            move |mut commands: Commands,
                  particle_types_path: Res<ParticleTypesPath>,
                  mut state: ResMut<NextState<ParticleTypesInitFileReadyState>>| {
                let particle_types_init_file =
                    particle_types_path.0.join(particle_types_init_file.clone());

                if !particle_types_init_file.exists() {
                    let default_path = particle_types_path
                        .0
                        .join(PathBuf::from(DEFAULT_PARTICLES_ASSET));
                    if default_path.exists() {
                        if let Err(e) = std::fs::copy(&default_path, &particle_types_init_file) {
                            let warning = format!(
                                "Failed to copy default particles file to {:?}: {}",
                                particle_types_init_file, e
                            );
                            warn!(warning);
                            state.set(ParticleTypesInitFileReadyState::Failed(warning));
                            return;
                        }
                        info!(
                            "Copied default particles file to {:?}",
                            particle_types_init_file
                        );
                    } else {
                        warn!("Default particles file not found at {:?}", default_path);
                    }
                }
                commands.insert_resource(ParticleTypesInitFile(particle_types_init_file.clone()));
                state.set(ParticleTypesInitFileReadyState::Complete)
            },
        );
        app.add_systems(
            OnEnter(ParticleTypesInitFileReadyState::Complete),
            load_particle_types,
        )
        .add_systems(
            Update,
            msgr_particle_types_loaded.run_if(in_state(ParticleTypesLoadedState::Incomplete)),
        );
    }
}

fn load_particle_types(
    mut msgw_load_particles_scene: MessageWriter<LoadParticleTypesSignal>,
    particle_types_path: Res<ParticleTypesInitFile>,
) {
    let path = particle_types_path.0.clone();
    msgw_load_particles_scene.write(LoadParticleTypesSignal(path));
}

fn msgr_particle_types_loaded(
    mut msgr_particle_types_loaded: MessageReader<ParticleTypesLoadedSignal>,
    mut next_state: ResMut<NextState<ParticleTypesLoadedState>>,
) {
    msgr_particle_types_loaded.read().for_each(|msg| {
        info!(
            "Loaded particle types from file: {}",
            msg.0.to_string_lossy()
        );
        next_state.set(ParticleTypesLoadedState::Complete);
    });
}
