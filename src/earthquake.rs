use avian2d::prelude::*;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
use bevy_falling_sand::prelude::ParticleCollider;
use bevy_falling_sand::utils::connected_components;
use bevy_falling_sand::{
    ParticleMap,
    prelude::{DespawnParticleSignal, ParticleColor, StaticRigidBodyParticle},
};
use bevy_turborand::{GlobalRng, TurboRand};
use voronoice::{BoundingBox, Point, VoronoiBuilder};

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

const EARTHQUAKE_RIGID_BODY_RENDER_Z: f32 = 1.0;

#[derive(Event)]
pub struct Earthquake {
    pub center: Vec2,
    pub radius: f32,
}

#[derive(Component)]
pub struct DebugEarthquake {
    center: Vec2,
    radius: f32,
    fracture_edges: Vec<(Vec2, Vec2)>,
    timer: Timer,
}

fn on_earthquake(
    trigger: On<Earthquake>,
    mut commands: Commands,
    mut rng: ResMut<GlobalRng>,
    mut despawn_writer: MessageWriter<DespawnParticleSignal>,
    map: Res<ParticleMap>,
    static_particles: Query<&StaticRigidBodyParticle>,
    particle_colors: Query<&ParticleColor>,
) {
    let mut by_position: HashMap<IVec2, Entity> = HashMap::default();
    map.within_radius(trigger.center.round().as_ivec2(), trigger.radius)
        .for_each(|(pos, entity)| {
            if static_particles.contains(entity) {
                by_position.insert(pos, entity);
            }
        });

    let shape_count = connected_components(by_position.keys().copied()).len();

    let particle_positions: Vec<IVec2> = by_position.keys().copied().collect();
    let cells = generate_voronoi_cells(
        &mut rng,
        trigger.center,
        trigger.radius,
        &particle_positions,
    );

    let fracture_edges: Vec<(Vec2, Vec2)> = cells.iter().flat_map(cell_boundary_edges).collect();

    for cell in &cells {
        spawn_fracture_body(&mut commands, &particle_colors, &by_position, cell);
    }

    for cell in &cells {
        for pos in cell {
            if let Some(&entity) = by_position.get(pos) {
                despawn_writer.write(DespawnParticleSignal::from_entity(entity));
            }
        }
    }

    // JAB TODO: Despawn this entity after duration using `DelayedCommands` once Bevy releases it.

    debug!(
        "earthquake at {:?} r={}: {} shapes, {} cells, {} fracture edges",
        trigger.center,
        trigger.radius,
        shape_count,
        cells.len(),
        fracture_edges.len(),
    );

    commands.spawn(DebugEarthquake {
        center: trigger.center,
        radius: trigger.radius,
        fracture_edges,
        timer: Timer::from_seconds(5., TimerMode::Once),
    });
}

fn generate_voronoi_cells(
    rng: &mut GlobalRng,
    center: Vec2,
    radius: f32,
    particles: &[IVec2],
) -> Vec<HashSet<IVec2>> {
    if particles.is_empty() {
        return Vec::new();
    }

    let target_count = ((radius * radius * 0.01) as usize).clamp(8, 256);
    let site_count = target_count.min(particles.len());

    let sites: Vec<Point> = rng
        .as_mut()
        .sample_multiple(particles, site_count)
        .into_iter()
        .map(|p| Point {
            x: p.x as f64,
            y: p.y as f64,
        })
        .collect();

    let bbox = BoundingBox::new(
        Point {
            x: center.x as f64,
            y: center.y as f64,
        },
        (radius * 2.0 + 2.0) as f64,
        (radius * 2.0 + 2.0) as f64,
    );

    let Some(voronoi) = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(bbox)
        .build()
    else {
        return Vec::new();
    };

    let final_sites = voronoi.sites();
    let mut cells: HashMap<usize, HashSet<IVec2>> = HashMap::default();
    for &p in particles {
        let mut best_idx: usize = 0;
        let mut best_dist = f32::INFINITY;
        for (i, site) in final_sites.iter().enumerate() {
            let dx = p.x as f32 - site.x as f32;
            let dy = p.y as f32 - site.y as f32;
            let d = dx * dx + dy * dy;
            if d < best_dist {
                best_dist = d;
                best_idx = i;
            }
        }
        cells.entry(best_idx).or_default().insert(p);
    }

    cells.into_values().collect()
}

fn cell_boundary_edges(cell: &HashSet<IVec2>) -> Vec<(Vec2, Vec2)> {
    let mut edges = Vec::new();
    for &p in cell {
        let fx = p.x as f32;
        let fy = p.y as f32;
        if !cell.contains(&IVec2::new(p.x, p.y + 1)) {
            edges.push((Vec2::new(fx, fy + 1.0), Vec2::new(fx + 1.0, fy + 1.0)));
        }
        if !cell.contains(&IVec2::new(p.x, p.y - 1)) {
            edges.push((Vec2::new(fx, fy), Vec2::new(fx + 1.0, fy)));
        }
        if !cell.contains(&IVec2::new(p.x + 1, p.y)) {
            edges.push((Vec2::new(fx + 1.0, fy), Vec2::new(fx + 1.0, fy + 1.0)));
        }
        if !cell.contains(&IVec2::new(p.x - 1, p.y)) {
            edges.push((Vec2::new(fx, fy), Vec2::new(fx, fy + 1.0)));
        }
    }
    edges
}

fn spawn_fracture_body(
    commands: &mut Commands,
    particle_colors: &Query<&ParticleColor>,
    by_position: &HashMap<IVec2, Entity>,
    cell: &HashSet<IVec2>,
) {
    if cell.is_empty() {
        return;
    }

    let particle_world: Vec<(IVec2, Vec2)> = cell
        .iter()
        .map(|&p| (p, Vec2::new(p.x as f32 + 0.5, p.y as f32 + 0.5)))
        .collect();
    let centroid =
        particle_world.iter().map(|(_, v)| *v).sum::<Vec2>() / particle_world.len() as f32;

    let shapes: Vec<(Vec2, f32, Collider)> = particle_world
        .iter()
        .map(|(_, world)| (*world - centroid, 0.0, Collider::rectangle(1.0, 1.0)))
        .collect();
    let collider = Collider::compound(shapes);

    commands
        .spawn((
            Transform::from_xyz(centroid.x, centroid.y, EARTHQUAKE_RIGID_BODY_RENDER_Z),
            Visibility::default(),
            RigidBody::Dynamic,
            collider,
            ParticleCollider,
        ))
        .with_children(|p| {
            for (grid_pos, world) in &particle_world {
                let color = by_position
                    .get(grid_pos)
                    .and_then(|e| particle_colors.get(*e).ok())
                    .map_or(Color::WHITE, |pc| pc.0);
                let local = *world - centroid;
                p.spawn((
                    Sprite::from_color(color, Vec2::ONE),
                    Transform::from_xyz(local.x, local.y, 0.0),
                ));
            }
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
            let alpha = 1. - debug_earthquake.timer.fraction();
            earthquake_gizmos.circle_2d(
                Isometry2d::from_translation(debug_earthquake.center),
                debug_earthquake.radius,
                Color::srgba(1., 1., 1., alpha),
            );
            let fracture_color = Color::srgba(1., 0.4, 0.2, alpha);
            for &(start, end) in &debug_earthquake.fracture_edges {
                earthquake_gizmos.line_2d(start, end, fracture_color);
            }
            if debug_earthquake.timer.is_finished() {
                commands.entity(entity).despawn();
            }
        });
}
