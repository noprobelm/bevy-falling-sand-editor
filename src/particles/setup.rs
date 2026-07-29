use bevy::prelude::*;
use bevy_falling_sand::prelude::{LoadParticleTypesSignal, PersistParticleTypesSignal};
use bevy_persistent::Persistent;

use super::default::{DefaultParticleIds, spawn_default_particles};
use crate::{
    config::{ActiveWorldPath, ParticleTypesFile, WorldConfig},
    setup::SetupSystems,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<RestoreDefaultParticleTypesSignal>()
            .add_systems(
                Startup,
                (
                    // Load the particle types from `world.toml`
                    load_world_particle_types_file,
                )
                    .in_set(SetupSystems::Particles),
            )
            .add_systems(
                Update,
                (restore_default_particle_types, persist_restored_defaults).chain(),
            );
    }
}

/// Requests that the active world's `default.scn.ron` be recreated from `default.rs`.
#[derive(Message)]
pub struct RestoreDefaultParticleTypesSignal;

/// Try to load the particle types file
fn load_world_particle_types_file(
    mut commands: Commands,
    active_world_path: Res<ActiveWorldPath>,
    world_config: Res<Persistent<WorldConfig>>,
    mut msgw_load_particles_scene: MessageWriter<LoadParticleTypesSignal>,
    mut msgw_persist: MessageWriter<PersistParticleTypesSignal>,
) {
    commands.insert_resource(DefaultParticleIds::default());

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

fn restore_default_particle_types(
    mut commands: Commands,
    mut restore_requests: MessageReader<RestoreDefaultParticleTypesSignal>,
    active_world_path: Res<ActiveWorldPath>,
    mut particle_types_file: ResMut<ParticleTypesFile>,
    particle_types: Query<Entity, With<bevy_falling_sand::prelude::ParticleType>>,
) {
    if restore_requests.read().next().is_none() {
        return;
    }

    for entity in &particle_types {
        commands.entity(entity).despawn();
    }
    spawn_default_particles(&mut commands);

    particle_types_file.0 = active_world_path.0.join("default.scn.ron");
    info!("Restored default particle definitions from default.rs");
}

fn persist_restored_defaults(
    mut restore_requests: MessageReader<RestoreDefaultParticleTypesSignal>,
    particle_types_file: Res<ParticleTypesFile>,
    mut persist: MessageWriter<PersistParticleTypesSignal>,
) {
    if restore_requests.read().next().is_some() {
        persist.write(PersistParticleTypesSignal(particle_types_file.0.clone()));
    }
}
