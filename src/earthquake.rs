use bevy::platform::collections::HashMap;
use bevy::prelude::*;
use bevy_falling_sand::{
    ParticleMap,
    prelude::{StaticRigidBodyParticle, connected_components, perimeter_positions},
};

pub(super) struct EarthquakePlugin;

impl Plugin for EarthquakePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_earthquake)
            .add_systems(Update, debug_earthquake)
            .init_gizmo_group::<EarthquakeGizmos>();
    }
}

#[derive(GizmoConfigGroup, Copy, Clone, Default, Debug, Reflect)]
pub(super) struct EarthquakeGizmos;

#[derive(Event)]
pub struct Earthquake {
    pub center: Vec2,
    pub radius: f32,
}

pub struct EarthquakeShape {
    pub cells: Vec<(IVec2, Entity)>,
    pub edges: Vec<(IVec2, Entity)>,
}

#[derive(Component)]
pub struct DebugEarthquake {
    center: Vec2,
    radius: f32,
    timer: Timer,
}

fn on_earthquake(
    trigger: On<Earthquake>,
    mut commands: Commands,
    map: Res<ParticleMap>,
    static_particles: Query<&StaticRigidBodyParticle>,
) {
    let mut by_position: HashMap<IVec2, Entity> = HashMap::default();
    map.within_radius(trigger.center.round().as_ivec2(), trigger.radius)
        .for_each(|(pos, entity)| {
            if static_particles.contains(entity) {
                by_position.insert(pos, entity);
            }
        });

    let shapes: Vec<EarthquakeShape> = connected_components(by_position.keys().copied())
        .into_iter()
        .map(|component| {
            let edges = perimeter_positions(&component)
                .into_iter()
                .map(|pos| (pos, by_position[&pos]))
                .collect();
            let cells = component
                .into_iter()
                .map(|pos| (pos, by_position[&pos]))
                .collect();
            EarthquakeShape { cells, edges }
        })
        .collect();

    // JAB TODO: Despawn this entity after duration using `DelayedCommands` once Bevy releases it.

    info!(
        "earthquake at {:?} r={}: {} shapes",
        trigger.center,
        trigger.radius,
        shapes.len()
    );

    commands.spawn(DebugEarthquake {
        center: trigger.center,
        radius: trigger.radius,
        timer: Timer::from_seconds(5., TimerMode::Once),
    });
}

fn debug_earthquake(
    mut commands: Commands,
    mut debug_earthquake: Query<(Entity, &mut DebugEarthquake)>,
    time: Res<Time>,
    mut earthquake_gizmos: Gizmos<EarthquakeGizmos>,
) {
    debug_earthquake
        .iter_mut()
        .for_each(|(entity, mut debug_earthquake)| {
            debug_earthquake.timer.tick(time.delta());
            earthquake_gizmos.circle_2d(
                Isometry2d::from_translation(debug_earthquake.center),
                debug_earthquake.radius,
                Color::srgba(1., 1., 1., 1. - debug_earthquake.timer.fraction()),
            );
            if debug_earthquake.timer.is_finished() {
                commands.entity(entity).despawn();
            }
        });
}
