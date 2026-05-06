mod algs;
mod default;
mod setup;

use bevy::prelude::*;
use bevy_falling_sand::core::particle::{
    AttachedToParticleType, Particle, ParticleMap, ParticleSyncExt, ParticleType,
};
use serde::{Deserialize, Serialize};

pub use algs::*;
pub use setup::*;

use crate::Cursor;

/// User-defined category for grouping particles in the editor.
/// Defaults to movement-oriented categories like "Wall", "Solid", "Liquid", etc.
#[derive(Component, Clone, Default, PartialEq, Debug, Reflect, Serialize, Deserialize)]
#[reflect(Component)]
#[type_path = "bfs_editor::particle"]
pub struct ParticleCategory(pub String);

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, PatternsPlugin))
            .register_type::<ParticleCategory>()
            .register_particle_sync_component::<ParticleCategory>()
            .init_resource::<HoveredParticle>()
            .add_systems(Update, update_hovered_particle);
    }
}

#[derive(Default, Resource, Clone, Debug)]
pub struct HoveredParticle {
    pub particle: Option<ParticleType>,
}

fn update_hovered_particle(
    cursor_position: Res<Cursor>,
    map: Res<ParticleMap>,
    particle_query: Query<&AttachedToParticleType, With<Particle>>,
    type_query: Query<&ParticleType>,
    mut hovered_particle: ResMut<HoveredParticle>,
) -> Result {
    let position = IVec2::new(
        cursor_position.current.x.floor() as i32,
        cursor_position.current.y.floor() as i32,
    );
    if let Ok(Some(entity)) = map.get_copied(position)
        && let Ok(attached) = particle_query.get(entity)
        && let Ok(particle_type) = type_query.get(attached.0)
    {
        hovered_particle.particle = Some(particle_type.clone());
    } else {
        hovered_particle.particle = None;
    }
    Ok(())
}
