use bevy::prelude::*;

use crate::tools::earthquake::states::EarthquakeRegionState;

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

    pub fn from_brush_state(state: EarthquakeRegionState, center: Vec2, size: f32) -> Self {
        match state {
            EarthquakeRegionState::Circle => Self::circle(center, size),
            EarthquakeRegionState::Rect => Self::rect(center, Vec2::splat(size), 0.0),
            EarthquakeRegionState::Polygon => {
                let vertices = diamond_vertices(center, size).into_iter().collect();
                Self::polygon(vertices)
            }
        }
    }

    pub(super) fn area_hint(&self) -> f32 {
        match self {
            Self::Circle { radius, .. } => std::f32::consts::PI * radius * radius,
            Self::Rect { half_extents, .. } => half_extents.x * half_extents.y * 4.0,
            Self::Polygon { vertices } => polygon_signed_area(vertices).abs() * 0.5,
        }
    }

    pub(super) fn bounds(&self) -> IRect {
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

    pub(super) fn contains_point(&self, point: Vec2) -> bool {
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

    pub(super) fn draw_gizmo<G: GizmoConfigGroup>(&self, gizmos: &mut Gizmos<G>, color: Color) {
        match self {
            Self::Circle { center, radius } => {
                gizmos.circle_2d(Isometry2d::from_translation(*center), *radius, color);
            }
            Self::Rect {
                center,
                half_extents,
                rotation,
            } => {
                gizmos.rect_2d(
                    Isometry2d::new(*center, Rot2::radians(*rotation)),
                    *half_extents * 2.0,
                    color,
                );
            }
            Self::Polygon { vertices } => {
                for (start, end) in polygon_edges(vertices) {
                    gizmos.line_2d(start, end, color);
                }
            }
        }
    }
}

fn diamond_vertices(center: Vec2, size: f32) -> [Vec2; 4] {
    [
        Vec2::new(0.0, size),
        Vec2::new(size, 0.0),
        Vec2::new(0.0, -size),
        Vec2::new(-size, 0.0),
    ]
    .map(|vertex| center + vertex)
}

pub(super) fn points_bounds<I>(points: I) -> Option<IRect>
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

pub(super) fn polygon_contains_point(vertices: &[Vec2], point: Vec2) -> bool {
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

pub(super) fn polygon_edges(vertices: &[Vec2]) -> Vec<(Vec2, Vec2)> {
    if vertices.len() < 2 {
        return Vec::new();
    }

    vertices
        .iter()
        .copied()
        .zip(vertices.iter().copied().cycle().skip(1))
        .take(vertices.len())
        .collect()
}
