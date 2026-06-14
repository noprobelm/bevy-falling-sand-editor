use avian2d::prelude::*;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
use bevy_falling_sand::{prelude::*, utils::mesh_from_grid_cells};

use crate::tools::earthquake::{
    EarthquakeConfiguration,
    region::{points_bounds, polygon_contains_point, polygon_edges},
    states::EarthquakeFractureShape,
    voronoi::GeneratedVoronoiCell,
};

pub(super) struct BuiltFractureBody {
    pub(super) source_centroid: Vec2,
    pub(super) collider: Collider,
    pub(super) particle_collider: ParticleCollider,
    sprites: Vec<(Vec2, Color)>,
}

#[derive(Component, Clone)]
pub(super) struct FractureBody {
    pub(super) cells: HashMap<IVec2, Color>,
    pub(super) source_centroid: Vec2,
}

pub(super) fn spawn_fracture_body_for_cell(
    commands: &mut Commands,
    particle_colors: &Query<&ParticleColor>,
    by_position: &HashMap<IVec2, Entity>,
    config: &EarthquakeConfiguration,
    cell: &GeneratedVoronoiCell,
    fracture_shape: EarthquakeFractureShape,
    excluded_particles: &HashSet<IVec2>,
) -> Option<Entity> {
    if cell.particles.is_empty() {
        return None;
    }

    let cell_colors =
        cell_colors_for_voronoi_cell(cell, fracture_shape, particle_colors, by_position)
            .into_iter()
            .filter(|(position, _)| !excluded_particles.contains(position))
            .collect();

    spawn_fracture_body_from_cells(
        commands,
        cell_colors,
        config,
        fracture_shape == EarthquakeFractureShape::Concave,
    )
}

pub(super) fn cell_colors_for_voronoi_cell(
    cell: &GeneratedVoronoiCell,
    fracture_shape: EarthquakeFractureShape,
    particle_colors: &Query<&ParticleColor>,
    by_position: &HashMap<IVec2, Entity>,
) -> HashMap<IVec2, Color> {
    match fracture_shape {
        EarthquakeFractureShape::Convex => convex_hull_vertices_for_cells(&cell.particles)
            .and_then(|vertices| cell_colors_for_polygon(&vertices, particle_colors, by_position))
            .unwrap_or_else(|| {
                cell_colors_for_positions(&cell.particles, particle_colors, by_position)
            }),
        EarthquakeFractureShape::Concave if !cell.is_on_hull => {
            cell_colors_for_polygon(&cell.vertices, particle_colors, by_position).unwrap_or_else(
                || cell_colors_for_positions(&cell.particles, particle_colors, by_position),
            )
        }
        EarthquakeFractureShape::Concave => {
            cell_colors_for_positions(&cell.particles, particle_colors, by_position)
        }
    }
}

pub(super) fn fracture_debug_edges(
    cell: &GeneratedVoronoiCell,
    fracture_shape: EarthquakeFractureShape,
) -> Vec<(Vec2, Vec2)> {
    match fracture_shape {
        EarthquakeFractureShape::Convex => cell_boundary_edges(&cell.particles),
        EarthquakeFractureShape::Concave if !cell.is_on_hull => polygon_edges(&cell.vertices),
        EarthquakeFractureShape::Concave => cell_boundary_edges(&cell.particles),
    }
}

pub(super) fn concave_fracture_line_particles(
    cells: &[GeneratedVoronoiCell],
    particles: impl IntoIterator<Item = IVec2>,
) -> HashSet<IVec2> {
    let particles: HashSet<IVec2> = particles.into_iter().collect();
    let mut edge_counts: HashMap<VoronoiEdgeKey, (Vec2, Vec2, usize)> = HashMap::default();

    for cell in cells {
        for (start, end) in polygon_edges(&cell.vertices) {
            let key = VoronoiEdgeKey::new(start, end);
            edge_counts
                .entry(key)
                .and_modify(|(_, _, count)| *count += 1)
                .or_insert((start, end, 1));
        }
    }

    edge_counts
        .into_values()
        .filter(|(_, _, count)| *count > 1)
        .flat_map(|(start, end, _)| rasterized_line_cells(start, end))
        .filter(|position| particles.contains(position))
        .collect()
}

fn cell_colors_for_positions(
    positions: &HashSet<IVec2>,
    particle_colors: &Query<&ParticleColor>,
    by_position: &HashMap<IVec2, Entity>,
) -> HashMap<IVec2, Color> {
    positions
        .iter()
        .filter_map(|grid_pos| {
            particle_color_at(*grid_pos, particle_colors, by_position)
                .map(|color| (*grid_pos, color))
        })
        .collect()
}

fn cell_colors_for_polygon(
    vertices: &[Vec2],
    particle_colors: &Query<&ParticleColor>,
    by_position: &HashMap<IVec2, Entity>,
) -> Option<HashMap<IVec2, Color>> {
    let bounds = points_bounds(vertices.iter().copied())?;
    let mut cell_colors = HashMap::default();

    for y in bounds.min.y..bounds.max.y {
        for x in bounds.min.x..bounds.max.x {
            let grid_pos = IVec2::new(x, y);
            let cell_center = grid_pos.as_vec2() + Vec2::splat(0.5);
            if polygon_contains_point(vertices, cell_center)
                && let Some(color) = particle_color_at(grid_pos, particle_colors, by_position)
            {
                cell_colors.insert(grid_pos, color);
            }
        }
    }

    (!cell_colors.is_empty()).then_some(cell_colors)
}

fn particle_color_at(
    grid_pos: IVec2,
    particle_colors: &Query<&ParticleColor>,
    by_position: &HashMap<IVec2, Entity>,
) -> Option<Color> {
    by_position
        .get(&grid_pos)
        .map(|entity| particle_colors.get(*entity).map_or(Color::WHITE, |pc| pc.0))
}

fn convex_hull_vertices_for_cells(cells: &HashSet<IVec2>) -> Option<Vec<Vec2>> {
    let mesh = mesh_from_grid_cells(cells.iter().copied(), 0.0);
    convex_hull_vertices(mesh.vertices)
}

fn convex_hull_vertices(mut points: Vec<Vec2>) -> Option<Vec<Vec2>> {
    if points.len() < 3 {
        return None;
    }

    points.sort_by(|a, b| a.x.total_cmp(&b.x).then_with(|| a.y.total_cmp(&b.y)));
    points.dedup();

    if points.len() < 3 {
        return None;
    }

    let mut lower = Vec::new();
    for point in &points {
        while lower.len() >= 2
            && cross(
                lower[lower.len() - 1] - lower[lower.len() - 2],
                *point - lower[lower.len() - 1],
            ) <= 0.0
        {
            lower.pop();
        }
        lower.push(*point);
    }

    let mut upper = Vec::new();
    for point in points.iter().rev() {
        while upper.len() >= 2
            && cross(
                upper[upper.len() - 1] - upper[upper.len() - 2],
                *point - upper[upper.len() - 1],
            ) <= 0.0
        {
            upper.pop();
        }
        upper.push(*point);
    }

    lower.pop();
    upper.pop();
    lower.extend(upper);
    (lower.len() >= 3).then_some(lower)
}

fn cross(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
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

#[derive(Clone, Copy, Eq, Hash, PartialEq)]
struct VoronoiEdgeKey {
    a: IVec2,
    b: IVec2,
}

impl VoronoiEdgeKey {
    fn new(start: Vec2, end: Vec2) -> Self {
        let start = quantized_point(start);
        let end = quantized_point(end);
        if (start.x, start.y) <= (end.x, end.y) {
            Self { a: start, b: end }
        } else {
            Self { a: end, b: start }
        }
    }
}

fn quantized_point(point: Vec2) -> IVec2 {
    (point * 1000.0).round().as_ivec2()
}

fn rasterized_line_cells(start: Vec2, end: Vec2) -> Vec<IVec2> {
    let delta = end - start;
    let steps = ((delta.x.abs().max(delta.y.abs()) * 2.0).ceil() as usize).max(1);
    (0..=steps)
        .map(|i| {
            let t = i as f32 / steps as f32;
            (start + delta * t).floor().as_ivec2()
        })
        .collect()
}

fn particle_world_positions<'a, I>(cell: I) -> Vec<(IVec2, Vec2)>
where
    I: IntoIterator<Item = &'a IVec2>,
{
    cell.into_iter()
        .map(|&p| (p, Vec2::new(p.x as f32 + 0.5, p.y as f32 + 0.5)))
        .collect()
}

fn particle_centroid(particle_world: &[(IVec2, Vec2)]) -> Vec2 {
    particle_world.iter().map(|(_, v)| *v).sum::<Vec2>() / particle_world.len() as f32
}

fn compound_particle_collider(particle_world: &[(IVec2, Vec2)], centroid: Vec2) -> Collider {
    let shapes: Vec<(Vec2, f32, Collider)> = particle_world
        .iter()
        .map(|(_, world)| (*world - centroid, 0.0, Collider::rectangle(1.0, 1.0)))
        .collect();
    Collider::compound(shapes)
}

fn convex_particle_collider(
    cell: &HashSet<IVec2>,
    particle_world: &[(IVec2, Vec2)],
    centroid: Vec2,
) -> Collider {
    let mesh = mesh_from_grid_cells(cell.iter().copied(), 0.0);
    if mesh.vertices.is_empty() {
        return compound_particle_collider(particle_world, centroid);
    }

    let vertices: Vec<Vec2> = mesh
        .vertices
        .into_iter()
        .map(|vertex| vertex - centroid)
        .collect();

    Collider::convex_hull(vertices)
        .unwrap_or_else(|| compound_particle_collider(particle_world, centroid))
}

fn mesh_particle_collider(
    cell: &HashSet<IVec2>,
    particle_world: &[(IVec2, Vec2)],
    centroid: Vec2,
) -> Collider {
    let mesh = mesh_from_grid_cells(cell.iter().copied(), 0.0);
    if mesh.vertices.is_empty() || mesh.indices.is_empty() {
        return compound_particle_collider(particle_world, centroid);
    }

    let vertices: Vec<Vec2> = mesh
        .vertices
        .into_iter()
        .map(|vertex| vertex - centroid)
        .collect();

    Collider::try_trimesh(vertices, mesh.indices)
        .unwrap_or_else(|_| compound_particle_collider(particle_world, centroid))
}

pub(super) fn spawn_fracture_body_from_cells(
    commands: &mut Commands,
    cell_colors: HashMap<IVec2, Color>,
    config: &EarthquakeConfiguration,
    use_mesh_collider: bool,
) -> Option<Entity> {
    let built = build_fracture_body(&cell_colors, config, use_mesh_collider)?;
    let transform = Transform::from_xyz(
        built.source_centroid.x,
        built.source_centroid.y,
        config.rigid_body_render_z,
    );
    Some(spawn_built_fracture_body(
        commands,
        cell_colors,
        built,
        transform,
        RigidBody::Dynamic,
        config,
        config.particle_collider_options,
    ))
}

pub(super) fn build_fracture_body(
    cell_colors: &HashMap<IVec2, Color>,
    config: &EarthquakeConfiguration,
    use_mesh_collider: bool,
) -> Option<BuiltFractureBody> {
    if cell_colors.len() < config.min_fracture_body_cells() {
        return None;
    }

    let cells: HashSet<IVec2> = cell_colors.keys().copied().collect();
    let particle_world = particle_world_positions(cells.iter());
    let centroid = particle_centroid(&particle_world);
    let collider = if use_mesh_collider {
        mesh_particle_collider(&cells, &particle_world, centroid)
    } else {
        convex_particle_collider(&cells, &particle_world, centroid)
    };
    let sprites: Vec<(Vec2, Color)> = particle_world
        .iter()
        .map(|(grid_pos, world)| {
            let color = cell_colors.get(grid_pos).copied().unwrap_or(Color::WHITE);
            (*world, color)
        })
        .collect();
    Some(BuiltFractureBody {
        source_centroid: centroid,
        collider,
        particle_collider: ParticleCollider::from_grid_cells(cells, centroid),
        sprites,
    })
}

pub(super) fn spawn_built_fracture_body(
    commands: &mut Commands,
    cell_colors: HashMap<IVec2, Color>,
    built: BuiltFractureBody,
    transform: Transform,
    rigid_body: RigidBody,
    config: &EarthquakeConfiguration,
    particle_collider_options: ParticleColliderOptions,
) -> Entity {
    let BuiltFractureBody {
        source_centroid,
        collider,
        particle_collider,
        sprites,
    } = built;
    let particle_collider = particle_collider.with_options(particle_collider_options);

    commands
        .spawn((
            transform,
            Visibility::default(),
            rigid_body,
            collider,
            CollisionMargin(config.collision_margin),
            particle_collider,
            FractureBody {
                cells: cell_colors,
                source_centroid,
            },
        ))
        .with_children(|p| {
            spawn_fracture_body_sprites(p, source_centroid, &sprites);
        })
        .id()
}

#[allow(clippy::too_many_arguments)]
pub(super) fn apply_built_fracture_body(
    commands: &mut Commands,
    entity: Entity,
    children: &Query<&Children>,
    fracture_body: &mut FractureBody,
    collider: &mut Collider,
    particle_collider: &mut ParticleCollider,
    transform: &mut Transform,
    cell_colors: HashMap<IVec2, Color>,
    built: BuiltFractureBody,
) -> Vec2 {
    let BuiltFractureBody {
        source_centroid,
        collider: new_collider,
        particle_collider: new_particle_collider,
        sprites,
    } = built;

    let old_centroid = fracture_body.source_centroid;
    fracture_body.cells = cell_colors;
    fracture_body.source_centroid = source_centroid;
    *collider = new_collider;
    *particle_collider = new_particle_collider
        .with_options(ParticleColliderOptions::from(particle_collider.resting));
    *transform = shifted_fracture_transform(*transform, old_centroid, source_centroid);

    if let Ok(children) = children.get(entity) {
        for child in children {
            commands.entity(*child).despawn();
        }
    }

    commands.entity(entity).with_children(|p| {
        spawn_fracture_body_sprites(p, source_centroid, &sprites);
    });

    source_centroid
}

fn spawn_fracture_body_sprites(
    parent: &mut ChildSpawnerCommands,
    centroid: Vec2,
    sprites: &[(Vec2, Color)],
) {
    for (world, color) in sprites {
        let local = *world - centroid;
        parent.spawn((
            Sprite::from_color(*color, Vec2::ONE),
            Transform::from_xyz(local.x, local.y, 0.0),
        ));
    }
}

pub(super) fn shifted_fracture_transform(
    mut transform: Transform,
    old_source_centroid: Vec2,
    new_source_centroid: Vec2,
) -> Transform {
    let local_delta = new_source_centroid - old_source_centroid;
    transform.translation += transform.rotation * local_delta.extend(0.0);
    transform
}

pub(super) fn cell_colors_for_component(
    colors: &HashMap<IVec2, Color>,
    component: &[IVec2],
) -> HashMap<IVec2, Color> {
    component
        .iter()
        .filter_map(|cell| colors.get(cell).copied().map(|color| (*cell, color)))
        .collect()
}
