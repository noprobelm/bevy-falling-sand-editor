use bevy::prelude::*;
use bevy_falling_sand::prelude::*;

use crate::ui::{ParticleMaterialLabels, SelectedEditorParticle};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleMaterialLabels>().add_systems(
            Update,
            spawn_selected_particle
                .run_if(not(resource_exists::<SelectedEditorParticle>))
                .run_if(condition_particle_types_loaded),
        );
    }
}

fn spawn_selected_particle(
    mut commands: Commands,
    registry: Res<ParticleTypeRegistry>,
    particle_types: Query<&ParticleType>,
) {
    const DEFAULT_PARTICLE_NAME: &str = "Dirt Wall";
    let particle = if let Some(entity) = registry.get(DEFAULT_PARTICLE_NAME) {
        Particle::from(
            particle_types
                .get(*entity)
                .expect("Failed to find particle type {DEFAULT_PARTICLE} in query")
                .clone(),
        )
    } else {
        Particle::from(
            particle_types
                .get(
                    *registry
                        .entities()
                        .next()
                        .expect("No particle types found in the world"),
                )
                .expect("Failed to find particle type in query")
                .clone(),
        )
    };

    commands.insert_resource(SelectedEditorParticle(particle));
}

fn condition_particle_types_loaded(particle_types: Query<(), Added<ParticleType>>) -> bool {
    !particle_types.is_empty()
}
