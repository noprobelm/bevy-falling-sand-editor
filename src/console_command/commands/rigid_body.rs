use avian2d::prelude::RigidBody;
use bevy::prelude::*;
use bevy_falling_sand::prelude::{
    ChunkDirtyState, ChunkIndex, DynamicRigidBodyProxy, ParticleMap,
    StaticRigidBodyParticleCollider,
};

use crate::console_command::ConsoleCommand;

pub(super) struct RigidBodyConsoleCommandPlugin;

impl Plugin for RigidBodyConsoleCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_despawn_rigid_bodies);
    }
}

#[derive(Default)]
pub struct RigidBodyConsoleCommand;

impl ConsoleCommand for RigidBodyConsoleCommand {
    fn name(&self) -> &'static str {
        "rigid_body"
    }

    fn description(&self) -> &'static str {
        "Rigid body operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(RigidBodyDespawnConsoleCommand)]
    }
}

#[derive(Default)]
pub struct RigidBodyDespawnConsoleCommand;

impl ConsoleCommand for RigidBodyDespawnConsoleCommand {
    fn name(&self) -> &'static str {
        "despawn"
    }

    fn description(&self) -> &'static str {
        "Despawn rigid bodies from the world"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(RigidBodyDespawnAllConsoleCommand),
            Box::new(RigidBodyDespawnDynamicConsoleCommand),
            Box::new(RigidBodyDespawnStaticConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct RigidBodyDespawnAllConsoleCommand;

impl ConsoleCommand for RigidBodyDespawnAllConsoleCommand {
    fn name(&self) -> &'static str {
        "all"
    }

    fn description(&self) -> &'static str {
        "Despawn all rigid bodies from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all rigid bodies from the world");
        commands.trigger(DespawnRigidBodiesEvent::all());
    }
}

#[derive(Default)]
pub struct RigidBodyDespawnDynamicConsoleCommand;

impl ConsoleCommand for RigidBodyDespawnDynamicConsoleCommand {
    fn name(&self) -> &'static str {
        "dynamic"
    }

    fn description(&self) -> &'static str {
        "Despawn dynamic rigid bodies from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all dynamic rigid bodies from the world");
        commands.trigger(DespawnRigidBodiesEvent::dynamic());
    }
}

#[derive(Default)]
pub struct RigidBodyDespawnStaticConsoleCommand;

impl ConsoleCommand for RigidBodyDespawnStaticConsoleCommand {
    fn name(&self) -> &'static str {
        "static"
    }

    fn description(&self) -> &'static str {
        "Despawn static rigid bodies from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all static rigid bodies from the world");
        commands.trigger(DespawnRigidBodiesEvent::static_bodies());
    }
}

#[derive(Event, Copy, Clone, Debug)]
struct DespawnRigidBodiesEvent {
    kind: RigidBodyDespawnKind,
}

impl DespawnRigidBodiesEvent {
    const fn all() -> Self {
        Self {
            kind: RigidBodyDespawnKind::All,
        }
    }

    const fn dynamic() -> Self {
        Self {
            kind: RigidBodyDespawnKind::Dynamic,
        }
    }

    const fn static_bodies() -> Self {
        Self {
            kind: RigidBodyDespawnKind::Static,
        }
    }
}

#[derive(Copy, Clone, Debug)]
enum RigidBodyDespawnKind {
    All,
    Dynamic,
    Static,
}

#[allow(clippy::needless_pass_by_value)]
fn on_despawn_rigid_bodies(
    trigger: On<DespawnRigidBodiesEvent>,
    mut commands: Commands,
    mut map: ResMut<ParticleMap>,
    chunk_index: Res<ChunkIndex>,
    mut chunk_query: Query<&mut ChunkDirtyState>,
    rigid_bodies: Query<
        (Entity, &RigidBody, Option<&DynamicRigidBodyProxy>),
        Without<StaticRigidBodyParticleCollider>,
    >,
) {
    let mut despawned = 0;

    for (entity, rigid_body, proxy) in &rigid_bodies {
        if !matches_kind(trigger.kind, rigid_body) {
            continue;
        }

        if let Some(proxy) = proxy {
            despawn_dynamic_proxy_particle(
                &mut commands,
                &mut map,
                &chunk_index,
                &mut chunk_query,
                proxy,
            );
        }

        commands.entity(entity).try_despawn();
        despawned += 1;
    }

    info!("Despawned {despawned} rigid bodies");
}

fn matches_kind(kind: RigidBodyDespawnKind, rigid_body: &RigidBody) -> bool {
    match kind {
        RigidBodyDespawnKind::All => true,
        RigidBodyDespawnKind::Dynamic => rigid_body.is_dynamic(),
        RigidBodyDespawnKind::Static => rigid_body.is_static(),
    }
}

fn despawn_dynamic_proxy_particle(
    commands: &mut Commands,
    map: &mut ParticleMap,
    chunk_index: &ChunkIndex,
    chunk_query: &mut Query<&mut ChunkDirtyState>,
    proxy: &DynamicRigidBodyProxy,
) {
    if let Some(position) = proxy.last_map_position
        && map.get_copied(position) == Ok(Some(proxy.particle_entity))
    {
        let _ = map.remove(position);
        mark_dirty(position, chunk_index, chunk_query);
    }

    commands.entity(proxy.particle_entity).try_despawn();
}

fn mark_dirty(
    position: IVec2,
    chunk_index: &ChunkIndex,
    chunk_query: &mut Query<&mut ChunkDirtyState>,
) {
    let chunk_coord = chunk_index.world_to_chunk_coord(position);
    if let Some(chunk_entity) = chunk_index.get(chunk_coord)
        && let Ok(mut dirty_state) = chunk_query.get_mut(chunk_entity)
    {
        dirty_state.mark_dirty(position);
    }
}
