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
            MaterialState::Wall(_) => self.walls.push(name),
            MaterialState::Solid(_) => self.solids.push(name),
            MaterialState::MovableSolid(_) => self.movable_solids.push(name),
            MaterialState::Liquid(_) => self.liquids.push(name),
            MaterialState::Gas(_) => self.gases.push(name),
            MaterialState::Insect(_) => self.insects.push(name),
            MaterialState::Other(_) => self.other.push(name),
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

pub fn all_material_states() -> [MaterialState; 7] {
    [
        MaterialState::Wall(Wall::default()),
        MaterialState::Solid(Solid::default()),
        MaterialState::MovableSolid(MovableSolid::default()),
        MaterialState::Liquid(Liquid::default()),
        MaterialState::Gas(Gas::default()),
        MaterialState::Insect(Insect::default()),
        MaterialState::Other(Movement::default()),
    ]
}

#[derive(Resource, Copy, Clone, PartialEq, Debug, Reflect)]
pub struct SelectedParticle(pub Entity);

#[derive(Resource, Default, Clone, Debug, Reflect)]
pub struct EditorState {
    pub map: HashMap<Entity, ParticleData>,
}

#[derive(Clone, Debug, Reflect)]
pub struct MovementStates {
    pub solid: SolidState,
    pub movable_solid: MovableSolidState,
    pub liquid: LiquidState,
    pub gas: GasState,
    pub insect: InsectState,
    pub other: OtherState,
}

#[derive(Clone, Debug, Reflect)]
pub struct SolidState {
    pub density: Density,
    pub speed: Speed,
}

#[derive(Clone, Debug, Reflect)]
pub struct MovableSolidState {
    pub movable_solid: MovableSolid,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Clone, Debug, Reflect)]
pub struct LiquidState {
    pub liquid: Liquid,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Clone, Debug, Reflect)]
pub struct GasState {
    pub gas: Gas,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Clone, Debug, Reflect)]
pub struct InsectState {
    pub insect: Insect,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Clone, Debug, Reflect)]
pub struct OtherState {
    pub movement: Movement,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Clone, Debug, Reflect)]
pub struct ParticleData {
    pub movement_states: MovementStates,
    pub timed_lifetime: TimedLifetime,
    pub chance_lifetime: ChanceLifetime,
    pub static_rigid_body: StaticRigidBodyParticle,
    pub palette: Palette,
    pub gradient: ColorGradient,
    pub changes_color: ChangesColor,
    pub burns: Burns,
}

impl Default for MovementStates {
    fn default() -> Self {
        Self {
            solid: SolidState {
                density: Density::default(),
                speed: Speed::default(),
            },
            movable_solid: MovableSolidState {
                movable_solid: MovableSolid::default(),
                density: Density::default(),
                speed: Speed::default(),
                momentum: Momentum::default(),
            },
            liquid: LiquidState {
                liquid: Liquid::default(),
                density: Density::default(),
                speed: Speed::default(),
                momentum: Momentum::default(),
            },
            gas: GasState {
                gas: Gas::default(),
                density: Density::default(),
                speed: Speed::default(),
                momentum: Momentum::default(),
            },
            insect: InsectState {
                insect: Insect::default(),
                density: Density::default(),
                speed: Speed::default(),
                momentum: Momentum::default(),
            },
            other: OtherState {
                movement: Movement::default(),
                density: Density::default(),
                speed: Speed::default(),
                momentum: Momentum::default(),
            },
        }
    }
}

impl Default for ParticleData {
    fn default() -> Self {
        let movement_states = MovementStates::default();
        let timed_lifetime = TimedLifetime::new(Duration::from_millis(10000));
        let chance_lifetime = ChanceLifetime::new(0.01);
        let static_rigid_body = StaticRigidBodyParticle;
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
            movement_states,
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
