use bevy::{platform::collections::HashMap, prelude::*};
use bevy_falling_sand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Default)]
pub struct ParticleMaterialLabels {
    pub walls: Vec<String>,
    pub solids: Vec<String>,
    pub movable_solids: Vec<String>,
    pub liquids: Vec<String>,
    pub gases: Vec<String>,
    pub insects: Vec<String>,
    pub other: Vec<String>,
}

impl ParticleMaterialLabels {
    pub fn push(&mut self, material: &MaterialState, name: String) {
        match material {
            MaterialState::Wall => self.walls.push(name),
            MaterialState::Solid => self.solids.push(name),
            MaterialState::MovableSolid => self.movable_solids.push(name),
            MaterialState::Liquid => self.liquids.push(name),
            MaterialState::Gas => self.gases.push(name),
            MaterialState::Insect => self.insects.push(name),
            MaterialState::Other => self.other.push(name),
        }
    }

    pub fn categories(&self) -> impl Iterator<Item = (&str, &Vec<String>)> {
        [
            ("Walls", &self.walls),
            ("Solids", &self.solids),
            ("Movable Solids", &self.movable_solids),
            ("Liquids", &self.liquids),
            ("Gases", &self.gases),
            ("Insects", &self.insects),
            ("Other", &self.other),
        ]
        .into_iter()
    }
}

pub const ALL_MATERIAL_STATES: [MaterialState; 7] = [
    MaterialState::Wall,
    MaterialState::Solid,
    MaterialState::MovableSolid,
    MaterialState::Liquid,
    MaterialState::Gas,
    MaterialState::Insect,
    MaterialState::Other,
];

#[derive(Resource, Copy, Clone, PartialEq, Debug, Reflect)]
pub struct SelectedParticle(pub Entity);

#[derive(Resource, Clone, Debug, Reflect)]
pub struct EditorRegistry {
    pub map: HashMap<Entity, ParticleData>,
}

#[derive(Clone, Debug, Reflect)]
pub struct ParticleData {
    pub particle_type: ParticleType,
    pub state: MaterialState,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
    pub timed_lifetime: TimedLifetime,
    pub chance_lifetim: ChanceLifetime,
    pub static_rigid_body: StaticRigidBodyParticle,
    pub color: ParticleColor,
    pub changes_color: ChangesColor,
    pub fire: Fire,
    pub burns: Burns,
    pub burning: Burning,
}
