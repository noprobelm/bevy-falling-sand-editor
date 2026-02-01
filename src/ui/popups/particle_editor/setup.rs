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

fn spawn_selected_particle(mut commands: Commands, registry: Res<ParticleTypeRegistry>) {
    const DEFAULT_PARTICLE_NAME: &str = "Flammable Gas";
    let entity = registry
        .get(DEFAULT_PARTICLE_NAME)
        .or_else(|| registry.entities().next())
        .copied()
        .expect("No particle types found in the world");

    commands.insert_resource(SelectedEditorParticle(entity));
}

fn condition_particle_types_loaded(particle_types: Query<(), Added<ParticleType>>) -> bool {
    !particle_types.is_empty()
}
