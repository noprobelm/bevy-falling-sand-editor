use bevy::{ecs::query::QueryData, prelude::*};
use bevy_egui::EguiPrimaryContextPass;
use bevy_falling_sand::prelude::*;

use crate::ui::{
    EditorState, GasState, InsectState, LiquidState, MovableSolidState, MovementStates, OtherState,
    ParticleData, ParticleMaterialLabels, SolidState, UiSystems,
};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
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

#[derive(QueryData)]
struct CoreQuery {
    particle_type: &'static ParticleType,
    timed_lifetime: Option<&'static TimedLifetime>,
    chance_lifetime: Option<&'static ChanceLifetime>,
}

#[derive(QueryData)]
struct MovementQuery {
    material: &'static MaterialState,
    density: Option<&'static Density>,
    speed: Option<&'static Speed>,
    momentum: Option<&'static Momentum>,
}

#[derive(QueryData)]
struct PhysicsQuery {
    static_rigid_body: Option<&'static StaticRigidBodyParticle>,
}

#[derive(QueryData)]
struct ColorQuery {
    profile: &'static ColorProfile,
    changes_color: Option<&'static ChangesColor>,
}

#[derive(QueryData)]
struct ReactionsQuery {
    burns: Option<&'static Burns>,
}

#[derive(QueryData)]
struct ParticleDataQuery {
    core: CoreQuery,
    movement: MovementQuery,
    physics: PhysicsQuery,
    color: ColorQuery,
    reactions: ReactionsQuery,
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
