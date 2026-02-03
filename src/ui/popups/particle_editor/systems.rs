use bevy::{ecs::query::QueryData, prelude::*};
use bevy_egui::EguiPrimaryContextPass;
use bevy_falling_sand::prelude::*;

use crate::ui::{EditorState, ParticleData, ParticleMaterialLabels, UiSystems};

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
struct PhysicsQuery {
    static_rigid_body: Option<&'static StaticRigidBodyParticle>,
}

#[derive(QueryData)]
struct ColorQuery {
    changes_color: Option<&'static ChangesColor>,
}

#[derive(QueryData)]
struct ReactionsQuery {
    burns: Option<&'static Burns>,
}

#[derive(QueryData)]
struct ParticleDataQuery {
    core: CoreQuery,
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
        let queried = query.get(*entity).ok();

        let particle_data = ParticleData {
            timed_lifetime: queried
                .as_ref()
                .and_then(|q| q.core.timed_lifetime.cloned())
                .or_else(|| cached.map(|c| c.timed_lifetime.clone()))
                .unwrap_or_else(|| defaults.timed_lifetime.clone()),
            chance_lifetime: queried
                .as_ref()
                .and_then(|q| q.core.chance_lifetime.cloned())
                .or_else(|| cached.map(|c| c.chance_lifetime.clone()))
                .unwrap_or_else(|| defaults.chance_lifetime.clone()),
            static_rigid_body: queried
                .as_ref()
                .and_then(|q| q.physics.static_rigid_body.cloned())
                .or_else(|| cached.map(|c| c.static_rigid_body.clone()))
                .unwrap_or_else(|| defaults.static_rigid_body.clone()),
            changes_color: queried
                .as_ref()
                .and_then(|q| q.color.changes_color.cloned())
                .or_else(|| cached.map(|c| c.changes_color.clone()))
                .unwrap_or_else(|| defaults.changes_color.clone()),
            burns: queried
                .as_ref()
                .and_then(|q| q.reactions.burns.cloned())
                .or_else(|| cached.map(|c| c.burns.clone()))
                .unwrap_or_else(|| defaults.burns.clone()),
        };

        new_state.map.insert(*entity, particle_data);
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
