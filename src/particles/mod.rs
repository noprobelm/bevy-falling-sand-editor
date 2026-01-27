mod setup;

use bevy::prelude::*;
use bevy_falling_sand::core::{Particle, ParticleMap, SpawnParticleSignal};
pub use setup::*;

use crate::CursorPosition;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SetupPlugin)
            .init_resource::<HoveredParticle>()
            .add_systems(Update, (spawn_particles, update_hovered_particle));
    }
}

fn spawn_particles(mut msgw_spawn_particle: MessageWriter<SpawnParticleSignal>) {
    for y in -3..3 {
        for x in -3..3 {
            msgw_spawn_particle.write(SpawnParticleSignal::new(
                Particle::new("Water"),
                IVec2::new(x, y),
            ));
        }
    }
}

#[derive(Default, Resource, Clone, Debug)]
pub struct HoveredParticle {
    pub particle: Option<Particle>,
}

fn update_hovered_particle(
    cursor_position: Res<CursorPosition>,
    map: Res<ParticleMap>,
    particle_query: Query<&Particle>,
    mut hovered_particle: ResMut<HoveredParticle>,
) -> Result {
    let position = IVec2::new(
        cursor_position.current.x.floor() as i32,
        cursor_position.current.y.floor() as i32,
    );
    if let Ok(Some(entity)) = map.get_entity(position) {
        let particle = particle_query.get(entity)?;
        hovered_particle.particle = Some(particle.clone());
    } else {
        hovered_particle.particle = None
    }
    Ok(())
}
