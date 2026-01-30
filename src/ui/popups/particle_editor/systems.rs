use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_falling_sand::prelude::*;

use crate::ui::ParticleMaterialLabels;

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            refresh_particle_labels.run_if(condition_particle_movement_changed),
        );
    }
}

/// System param to fetch particle types by material type.
#[derive(SystemParam, Copy, Clone, Debug)]
pub struct ParticleTypeMaterials<'w, 's> {
    pub walls: Query<'w, 's, &'static ParticleType, With<Wall>>,
    pub solids: Query<'w, 's, &'static ParticleType, With<Solid>>,
    pub movable_solids: Query<'w, 's, &'static ParticleType, With<MovableSolid>>,
    pub liquids: Query<'w, 's, &'static ParticleType, With<Liquid>>,
    pub gases: Query<'w, 's, &'static ParticleType, With<Gas>>,
    pub insects: Query<'w, 's, &'static ParticleType, With<Insect>>,
    pub other: Query<
        'w,
        's,
        &'static ParticleType,
        (
            Without<Wall>,
            Without<Solid>,
            Without<MovableSolid>,
            Without<Liquid>,
            Without<Gas>,
            Without<Insect>,
        ),
    >,
}

impl ParticleTypeMaterials<'_, '_> {
    fn generate_labels(&self) -> ParticleMaterialLabels {
        let mut labels = ParticleMaterialLabels::default();

        macro_rules! collect_labels {
            ($($field:ident),*) => {
                $(
                    labels.$field.extend(self.$field.iter().map(|ptype| ptype.name.to_string()));
                )*
            };
        }

        collect_labels!(
            walls,
            solids,
            movable_solids,
            liquids,
            gases,
            insects,
            other
        );

        labels
    }
}

// This doesn't strictly indicate a particle has actually changed its material type, but this query
// is a little more palatable than doing antoher query like `ParticleTypeMaterials` (except with the
// `Changed` `QueryFilter`). Movement updates for particle types should be happening infrequently
// enough where unecessarily running this system would be costly.
fn refresh_particle_labels(mut commands: Commands, materials: ParticleTypeMaterials) {
    commands.insert_resource(materials.generate_labels());
}

fn condition_particle_movement_changed(movement: Query<Entity, Changed<Movement>>) -> bool {
    !movement.is_empty()
}
