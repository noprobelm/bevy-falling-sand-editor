use bevy::prelude::*;
use bevy_falling_sand::prelude::{LoadParticleTypesSignal, PersistParticleTypesSignal};
use bevy_persistent::Persistent;

use super::default::spawn_default_particles;
use crate::{
    config::{ActiveWorldPath, ParticleTypesFile, WorldConfig},
    setup::SetupSystems,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
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
    mut msgw_persist: MessageWriter<PersistParticleTypesSignal>,
) {
    let particle_types_file = active_world_path
        .0
        .join(world_config.get().particle_types_file.clone());

    commands.insert_resource(ParticleTypesFile(
        active_world_path.0.join(particle_types_file.clone()),
    ));

    if particle_types_file.exists() {
        msgw_load_particles_scene.write(LoadParticleTypesSignal(particle_types_file));
    } else {
        spawn_default_particles(&mut commands);
        msgw_persist.write(PersistParticleTypesSignal(particle_types_file));
        info!("Spawned default particles and queued persistence");
    }
}
