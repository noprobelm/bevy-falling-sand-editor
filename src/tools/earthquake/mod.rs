mod debug;
mod signals;

pub use signals::*;

use avian2d::prelude::*;
use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
use bevy_falling_sand::{prelude::ParticleCollider, utils::mesh_from_grid_cells};
use bevy_turborand::TurboRand;
use voronoice::{BoundingBox, Point, VoronoiBuilder};

use crate::tools::earthquake::debug::DebugEarthquake;

pub(super) struct EarthquakePlugin;

impl Plugin for EarthquakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((debug::DebugPlugin, signals::SignalsPlugin));
    }
}

struct BuiltFractureBody {
    source_centroid: Vec2,
    collider: Collider,
    particle_collider: ParticleCollider,
    sprites: Vec<(Vec2, Color)>,
}

#[derive(Component, Clone)]
struct FractureBody {
    cells: HashMap<IVec2, Color>,
    source_centroid: Vec2,
}

#[derive(Clone, Debug)]
pub enum EarthquakeRegion {
    Circle {
        center: Vec2,
        radius: f32,
    },
    Rect {
        center: Vec2,
        half_extents: Vec2,
        rotation: f32,
    },
    Polygon {
        vertices: Vec<Vec2>,
    },
}

impl EarthquakeRegion {
    pub fn circle(center: Vec2, radius: f32) -> Self {
        Self::Circle { center, radius }
    }

    pub fn rect(center: Vec2, half_extents: Vec2, rotation: f32) -> Self {
        Self::Rect {
            center,
            half_extents,
            rotation,
        }
    }

    pub fn polygon(vertices: Vec<Vec2>) -> Self {
        Self::Polygon { vertices }
    }

    fn area_hint(&self) -> f32 {
        match self {
            Self::Circle { radius, .. } => std::f32::consts::PI * radius * radius,
            Self::Rect { half_extents, .. } => half_extents.x * half_extents.y * 4.0,
            Self::Polygon { vertices } => polygon_signed_area(vertices).abs() * 0.5,
        }
    }

    fn bounds(&self) -> IRect {
        match self {
            Self::Circle { center, radius } => {
                let half = Vec2::splat(*radius);
                IRect::from_corners(
                    (*center - half).floor().as_ivec2(),
                    (*center + half).ceil().as_ivec2(),
                )
            }
            Self::Rect {
                center,
                half_extents,
                rotation,
            } => {
                let rot = Rot2::radians(*rotation);
                let corners = [
                    Vec2::new(-half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, -half_extents.y),
                    Vec2::new(half_extents.x, half_extents.y),
                    Vec2::new(-half_extents.x, half_extents.y),
                ]
                .map(|corner| *center + rot * corner);
                points_bounds(corners)
                    .unwrap_or_else(|| IRect::from_corners(IVec2::ZERO, IVec2::ZERO))
            }
            Self::Polygon { vertices } => points_bounds(vertices.iter().copied())
                .unwrap_or_else(|| IRect::from_corners(IVec2::ZERO, IVec2::ZERO)),
        }
        .inflate(1)
    }

    fn contains_point(&self, point: Vec2) -> bool {
        match self {
            Self::Circle { center, radius } => point.distance_squared(*center) <= radius * radius,
            Self::Rect {
                center,
                half_extents,
                rotation,
            } => {
                let local = Rot2::radians(-*rotation) * (point - *center);
                local.x.abs() <= half_extents.x && local.y.abs() <= half_extents.y
            }
            Self::Polygon { vertices } => polygon_contains_point(vertices, point),
        }
    }
}

fn points_bounds<I>(points: I) -> Option<IRect>
where
    I: IntoIterator<Item = Vec2>,
{
    let mut points = points.into_iter();
    let first = points.next()?;
    let mut min = first;
    let mut max = first;
    for point in points {
        min = min.min(point);
        max = max.max(point);
    }
    Some(IRect::from_corners(
        min.floor().as_ivec2(),
        max.ceil().as_ivec2(),
    ))
}

fn polygon_signed_area(vertices: &[Vec2]) -> f32 {
    if vertices.len() < 3 {
        return 0.0;
    }

    vertices
        .iter()
        .zip(vertices.iter().cycle().skip(1))
        .map(|(a, b)| a.x * b.y - b.x * a.y)
        .sum()
}

fn polygon_contains_point(vertices: &[Vec2], point: Vec2) -> bool {
    if vertices.len() < 3 {
        return false;
    }

    let mut inside = false;
    for (a, b) in vertices.iter().zip(vertices.iter().cycle().skip(1)) {
        let crosses_y = (a.y > point.y) != (b.y > point.y);
        if crosses_y {
            let x_intersection = (b.x - a.x) * (point.y - a.y) / (b.y - a.y) + a.x;
            if point.x < x_intersection {
                inside = !inside;
            }
        }
    }
    inside
}
