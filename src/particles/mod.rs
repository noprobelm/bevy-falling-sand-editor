mod setup;

use bevy::prelude::*;
use bevy_falling_sand::core::{Particle, SpawnParticleSignal};
pub use setup::*;

pub struct ParticlesPlugin;

impl Plugin for ParticlesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SetupPlugin)
            .add_systems(Update, spawn_particles);
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
