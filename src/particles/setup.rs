use std::path::PathBuf;

use bevy::prelude::*;
use bevy_falling_sand::prelude::LoadParticleTypesSignal;
use bevy_persistent::Persistent;

use crate::{
    config::{ActiveWorldPath, ParticleTypesFile, WorldConfig},
    setup::SetupSystems,
};
//
// JAB TODO: A temporary solution until we have the editor up and running. It's currently helpful
// to have some default particles to fall back to.
const DEFAULT_PARTICLES_ASSET: &str = "assets/particles/particles.scn.ron";

pub struct ParticlesSetupPlugin;

impl Plugin for ParticlesSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                // Load the particle types from `world.toml`
                load_world_particle_types_file,
            )
                .in_set(SetupSystems::Particles),
        );
    }
}

/// Try to load the particle types file
fn load_world_particle_types_file(
    mut commands: Commands,
    active_world_path: Res<ActiveWorldPath>,
    world_config: Res<Persistent<WorldConfig>>,
    mut msgw_load_particles_scene: MessageWriter<LoadParticleTypesSignal>,
) {
    let particle_types_file = active_world_path
        .0
        .join(world_config.get().particle_types_file.clone());

    if !particle_types_file.exists() {
        // JAB TODO: A temporary solution until we have the editor up and running. It's currently helpful
        // to have some default particles to fall back to.
        let default_path = PathBuf::from(DEFAULT_PARTICLES_ASSET);
        if default_path.exists() {
            if let Err(e) = std::fs::copy(&default_path, &particle_types_file) {
                warn!(
                    "Failed to copy default particles file to {:?}: {}",
                    particle_types_file, e
                );
                return;
            }
            info!("Copied default particles file to {:?}", particle_types_file);
        } else {
            warn!("Default particles file not found at {:?}", default_path);
        }
    }

    commands.insert_resource(ParticleTypesFile(
        active_world_path.0.join(particle_types_file.clone()),
    ));

    msgw_load_particles_scene.write(LoadParticleTypesSignal(particle_types_file.clone()));
}
