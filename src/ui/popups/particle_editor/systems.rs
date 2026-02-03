use bevy::prelude::*;
use bevy_falling_sand::prelude::*;

use crate::ui::{EditorRegistry, ParticleMaterialLabels};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            refresh_particle_labels.run_if(condition_particle_movement_changed),
        );
    }
}

fn on_remove_particle_type(
    removed: On<Remove, ParticleType>,
    mut registry: ResMut<EditorRegistry>,
) {
    registry.map.remove(&removed.entity);
}

fn on_add_particle_type(added: On<Add, ParticleType>, mut registry: ResMut<EditorRegistry>) {}

// This doesn't strictly indicate a particle has actually changed its material type, but this query
// is a little more palatable than doing antoher query like `ParticleTypeMaterials` (except with the
// `Changed` `QueryFilter`). Movement updates for particle types should be happening infrequently
// enough where unecessarily running this system would be costly.
fn refresh_particle_labels(
    mut commands: Commands,
    materials: Query<(&ParticleType, &MaterialState), With<ParticleType>>,
) {
    let mut labels = ParticleMaterialLabels::default();
    for (ptype, material) in &materials {
        labels.push(material, ptype.name.to_string());
    }
    commands.insert_resource(labels);
}

fn condition_particle_movement_changed(movement: Query<Entity, Changed<MaterialState>>) -> bool {
    !movement.is_empty()
}
