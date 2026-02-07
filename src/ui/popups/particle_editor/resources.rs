use std::time::Duration;

use bevy::{ecs::query::QueryData, platform::collections::HashMap, prelude::*};
use bevy_egui::EguiPrimaryContextPass;
use bevy_falling_sand::prelude::*;

use crate::ui::UiSystems;

pub struct ResourcesPlugin;

impl Plugin for ResourcesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            synchronize_editor_registry
                .run_if(resource_changed::<ParticleTypeRegistry>)
                .before(UiSystems::ParticleEditor),
        )
        .add_systems(
            Update,
            refresh_particle_labels.run_if(condition_particle_movement_changed),
        );
    }
}

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
        MaterialState::Wall(Wall),
        MaterialState::Solid(Solid),
        MaterialState::MovableSolid(MovableSolid::default()),
        MaterialState::Liquid(Liquid::default()),
        MaterialState::Gas(Gas::default()),
        MaterialState::Insect(Insect),
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

#[derive(Bundle, Clone, Debug, Reflect)]
pub struct SolidState {
    pub solid: Solid,
    pub density: Density,
    pub speed: Speed,
}

#[derive(Bundle, Clone, Debug, Reflect)]
pub struct MovableSolidState {
    pub movable_solid: MovableSolid,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Bundle, Clone, Debug, Reflect)]
pub struct LiquidState {
    pub liquid: Liquid,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Bundle, Clone, Debug, Reflect)]
pub struct GasState {
    pub gas: Gas,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Bundle, Clone, Debug, Reflect)]
pub struct InsectState {
    pub insect: Insect,
    pub density: Density,
    pub speed: Speed,
    pub momentum: Momentum,
}

#[derive(Bundle, Clone, Debug, Reflect)]
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

impl MovementStates {
    /// Saves the current component values to the appropriate cached state before switching materials.
    pub fn save_current(
        &mut self,
        material: &MaterialState,
        density: Option<Density>,
        speed: Option<Speed>,
        momentum: Option<Momentum>,
    ) {
        match material {
            MaterialState::Wall(_) => {}
            MaterialState::Solid(_) => {
                self.solid.density = density.unwrap_or(self.solid.density);
                self.solid.speed = speed.unwrap_or(self.solid.speed);
            }
            MaterialState::MovableSolid(ms) => {
                self.movable_solid.movable_solid = ms.clone();
                self.movable_solid.density = density.unwrap_or(self.movable_solid.density);
                self.movable_solid.speed = speed.unwrap_or(self.movable_solid.speed);
                self.movable_solid.momentum = momentum.unwrap_or(self.movable_solid.momentum);
            }
            MaterialState::Liquid(l) => {
                self.liquid.liquid = l.clone();
                self.liquid.density = density.unwrap_or(self.liquid.density);
                self.liquid.speed = speed.unwrap_or(self.liquid.speed);
                self.liquid.momentum = momentum.unwrap_or(self.liquid.momentum);
            }
            MaterialState::Gas(g) => {
                self.gas.gas = g.clone();
                self.gas.density = density.unwrap_or(self.gas.density);
                self.gas.speed = speed.unwrap_or(self.gas.speed);
                self.gas.momentum = momentum.unwrap_or(self.gas.momentum);
            }
            MaterialState::Insect(i) => {
                self.insect.insect = i.clone();
                self.insect.density = density.unwrap_or(self.insect.density);
                self.insect.speed = speed.unwrap_or(self.insect.speed);
                self.insect.momentum = momentum.unwrap_or(self.insect.momentum);
            }
            MaterialState::Other(m) => {
                self.other.movement = m.clone();
                self.other.density = density.unwrap_or(self.other.density);
                self.other.speed = speed.unwrap_or(self.other.speed);
                self.other.momentum = momentum.unwrap_or(self.other.momentum);
            }
        }
    }
}

impl Default for MovementStates {
    fn default() -> Self {
        Self {
            solid: SolidState {
                solid: Solid,
                density: Density::default(),
                speed: Speed::default(),
            },
            movable_solid: MovableSolidState {
                movable_solid: MovableSolid::new()
                    .with_liquid_resistance(0.75)
                    .with_air_resistance(0.9),
                density: Density(1250),
                speed: Speed::new(5, 10),
                momentum: Momentum::default(),
            },
            liquid: LiquidState {
                liquid: Liquid::new(5).with_liquid_resistance(0.1),
                density: Density(750),
                speed: Speed::new(0, 3),
                momentum: Momentum::default(),
            },
            gas: GasState {
                gas: Gas::new(1),
                density: Density(200),
                speed: Speed::new(0, 1),
                momentum: Momentum::default(),
            },
            insect: InsectState {
                insect: Insect,
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

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct ParticleDataQuery {
    pub core: CoreQuery,
    pub movement: MovementQuery,
    pub physics: PhysicsQuery,
    pub color: ColorQuery,
    pub reactions: ReactionsQuery,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct CoreQuery {
    pub particle_type: &'static mut ParticleType,
    pub timed_lifetime: Option<&'static mut TimedLifetime>,
    pub chance_lifetime: Option<&'static mut ChanceLifetime>,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct MovementQuery {
    pub material: &'static MaterialState,
    pub density: Option<&'static mut Density>,
    pub speed: Option<&'static mut Speed>,
    pub momentum: Option<&'static Momentum>,
}

#[derive(QueryData)]
pub(crate) struct PhysicsQuery {
    pub static_rigid_body: Option<&'static StaticRigidBodyParticle>,
}

#[derive(QueryData)]
#[query_data(mutable)]
pub(crate) struct ColorQuery {
    pub profile: &'static mut ColorProfile,
    pub changes_color: Option<&'static mut ChangesColor>,
}

#[derive(QueryData)]
pub(crate) struct ReactionsQuery {
    pub burns: Option<&'static Burns>,
}

fn build_movement_states(
    material: &MaterialState,
    density: Option<&Density>,
    speed: Option<&Speed>,
    momentum: Option<&Momentum>,
    cached: &MovementStates,
) -> MovementStates {
    let mut states = cached.clone();

    match material {
        MaterialState::Wall(_) => {}
        MaterialState::Solid(_) => {
            states.solid = SolidState {
                solid: Solid,
                density: density.cloned().unwrap_or(cached.solid.density),
                speed: speed.cloned().unwrap_or(cached.solid.speed),
            };
        }
        MaterialState::MovableSolid(ms) => {
            states.movable_solid = MovableSolidState {
                movable_solid: ms.clone(),
                density: density.cloned().unwrap_or(cached.movable_solid.density),
                speed: speed.cloned().unwrap_or(cached.movable_solid.speed),
                momentum: momentum.cloned().unwrap_or(cached.movable_solid.momentum),
            };
        }
        MaterialState::Liquid(l) => {
            states.liquid = LiquidState {
                liquid: l.clone(),
                density: density.cloned().unwrap_or(cached.liquid.density),
                speed: speed.cloned().unwrap_or(cached.liquid.speed),
                momentum: momentum.cloned().unwrap_or(cached.liquid.momentum),
            };
        }
        MaterialState::Gas(g) => {
            states.gas = GasState {
                gas: g.clone(),
                density: density.cloned().unwrap_or(cached.gas.density),
                speed: speed.cloned().unwrap_or(cached.gas.speed),
                momentum: momentum.cloned().unwrap_or(cached.gas.momentum),
            };
        }
        MaterialState::Insect(i) => {
            states.insect = InsectState {
                insect: i.clone(),
                density: density.cloned().unwrap_or(cached.insect.density),
                speed: speed.cloned().unwrap_or(cached.insect.speed),
                momentum: momentum.cloned().unwrap_or(cached.insect.momentum),
            };
        }
        MaterialState::Other(m) => {
            states.other = OtherState {
                movement: m.clone(),
                density: density.cloned().unwrap_or(cached.other.density),
                speed: speed.cloned().unwrap_or(cached.other.speed),
                momentum: momentum.cloned().unwrap_or(cached.other.momentum),
            };
        }
    }

    states
}

fn synchronize_editor_registry(
    mut commands: Commands,
    query: Query<ParticleDataQuery>,
    particle_registry: Res<ParticleTypeRegistry>,
    editor_state: Res<EditorState>,
) {
    let defaults = ParticleData::default();
    let mut new_state = EditorState::default();

    for entity in particle_registry.entities() {
        let cached = editor_state.map.get(entity);

        if let Ok(data) = query.get(*entity) {
            let (palette, gradient) = match &data.color.profile.source {
                ColorSource::Palette(p) => (
                    p.clone(),
                    cached.map(|c| c.gradient.clone()).unwrap_or_default(),
                ),
                ColorSource::Gradient(g) => (
                    cached.map(|c| c.palette.clone()).unwrap_or_default(),
                    g.clone(),
                ),
            };

            let cached_movement = cached
                .map(|c| c.movement_states.clone())
                .unwrap_or_else(|| defaults.movement_states.clone());

            let movement_states = build_movement_states(
                data.movement.material,
                data.movement.density,
                data.movement.speed,
                data.movement.momentum,
                &cached_movement,
            );

            let particle_data = ParticleData {
                movement_states,
                timed_lifetime: data
                    .core
                    .timed_lifetime
                    .cloned()
                    .unwrap_or_else(|| defaults.timed_lifetime.clone()),
                chance_lifetime: data
                    .core
                    .chance_lifetime
                    .cloned()
                    .unwrap_or_else(|| defaults.chance_lifetime.clone()),
                static_rigid_body: data
                    .physics
                    .static_rigid_body
                    .cloned()
                    .unwrap_or(defaults.static_rigid_body),
                palette,
                gradient,
                changes_color: data
                    .color
                    .changes_color
                    .cloned()
                    .unwrap_or(defaults.changes_color),
                burns: data
                    .reactions
                    .burns
                    .cloned()
                    .unwrap_or_else(|| defaults.burns.clone()),
            };

            new_state.map.insert(*entity, particle_data);
        }
    }

    commands.insert_resource(new_state);
}

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
