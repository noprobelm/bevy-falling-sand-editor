use bevy::prelude::*;
use bevy_falling_sand::prelude::*;

use crate::{
    particles::DefaultParticleIds,
    ui::{
        EditorState, ParticleCategoryLabels, ParticleTypesSavedMessageConfiguration,
        SelectedParticle,
    },
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleTypesSavedMessageConfiguration>()
            .init_resource::<ParticleCategoryLabels>()
            .init_resource::<EditorState>()
            .add_systems(
                Update,
                set_initial_selected_particle
                    .run_if(not(resource_exists::<SelectedParticle>))
                    .run_if(condition_particle_types_loaded),
            );
    }
}

fn set_initial_selected_particle(
    mut commands: Commands,
    registry: Res<ParticleTypeRegistry>,
    default_ids: Res<DefaultParticleIds>,
) {
    let entity = registry
        .get(default_ids.flammable_gas)
        .or_else(|| registry.entities().next())
        .copied()
        .expect("No particle types found in the world");

    commands.insert_resource(SelectedParticle(entity));
}

fn condition_particle_types_loaded(particle_types: Query<(), Added<ParticleType>>) -> bool {
    !particle_types.is_empty()
}
