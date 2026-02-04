use std::time::Duration;

use bevy::{platform::collections::HashMap, prelude::*};
use bevy_falling_sand::prelude::*;

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

#[derive(Resource, Default, Clone, Debug, Reflect)]
pub struct EditorState {
    pub map: HashMap<Entity, ParticleData>,
}

#[derive(Clone, Debug, Reflect)]
pub struct ParticleData {
    pub timed_lifetime: TimedLifetime,
    pub chance_lifetime: ChanceLifetime,
    pub static_rigid_body: StaticRigidBodyParticle,
    pub palette: Palette,
    pub gradient: ColorGradient,
    pub changes_color: ChangesColor,
    pub burns: Burns,
}

impl Default for ParticleData {
    fn default() -> Self {
        let timed_lifetime = TimedLifetime::new(Duration::from_millis(10000));
        let chance_lifetime = ChanceLifetime::new(0.01);
        let static_rigid_body = StaticRigidBodyParticle::default();
        let changes_color = ChangesColor::Chance(0.1);
        let burns = Burns::new(
            Duration::from_millis(1000),
            Duration::from_millis(100),
            Some(0.5),
            None,
            Some(ColorProfile::palette(vec![
                Color::Srgba(Srgba::new(1., 0.34901962, 0., 1.)),
                Color::Srgba(Srgba::new(1., 0.5686275, 0., 1.)),
                Color::Srgba(Srgba::new(1., 0.8117647, 0., 1.)),
                Color::Srgba(Srgba::new(0.78039217, 0.2901961, 0.019607844, 1.)),
            ])),
            Some(Fire {
                burn_radius: 1.5,
                chance_to_spread: 0.01,
                destroys_on_spread: false,
            }),
            false,
        );
        let palette = Palette::default();
        let gradient = ColorGradient::default();
        Self {
            timed_lifetime,
            chance_lifetime,
            static_rigid_body,
            palette,
            gradient,
            changes_color,
            burns,
        }
    }
}
