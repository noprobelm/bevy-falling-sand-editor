use std::collections::HashMap;

use bevy::prelude::*;
use bevy_falling_sand::core::{ParticleType, SpawnParticleSignal};
use bevy_turborand::{GlobalRng, TurboRand};

#[derive(Event, Clone, PartialEq, Debug)]
pub struct SpawnBarnsleyEvent {
    pub center: IVec2,
    pub size: IVec2,
    pub num_iterations: u32,
    pub f1: f32,
    pub f2: f32,
    pub f3: f32,
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash, Debug)]
pub enum FernElement {
    Stem,
    LeftPinna,
    RightPinna,
}

/// A classified point in the rasterized fern.
pub struct FernPoint {
    pub position: IVec2,
    pub element: FernElement,
}

/// Generates a Barnsley fern rasterized onto an integer grid.
///
/// The fern is centered at `center` and scaled to fit within `size` while
/// preserving aspect ratio. Each point is classified as stem or pinna based
/// on its proximity to the rachis (main axis).
pub fn generate_fern(
    rng: &mut GlobalRng,
    center: IVec2,
    size: IVec2,
    f1: f32,
    f2: f32,
    f3: f32,
    min_iterations: u32,
) -> Vec<FernPoint> {
    let pixel_count = (size.x * size.y) as u32;
    let iterations = min_iterations.max(pixel_count * 5);
    let raw = sample_ifs(rng, f1, f2, f3, iterations);
    rasterize(&raw, center, size)
}

/// Runs the Barnsley fern iterated function system, producing raw f32 points.
fn sample_ifs(rng: &mut GlobalRng, f1: f32, f2: f32, f3: f32, iterations: u32) -> Vec<Vec2> {
    let mut points = Vec::with_capacity(iterations as usize);
    let (mut x, mut y) = (0.0_f32, 0.0_f32);

    for _ in 0..iterations {
        let r = rng.as_mut().f32_normalized();

        if r < f1 {
            x = 0.0;
            y *= 0.16;
        } else if r < f2 {
            let (nx, ny) = (0.85 * x + 0.04 * y, -0.04 * x + 0.85 * y + 1.6);
            x = nx;
            y = ny;
        } else if r < f3 {
            let (nx, ny) = (0.2 * x - 0.26 * y, 0.23 * x + 0.22 * y + 1.6);
            x = nx;
            y = ny;
        } else {
            let (nx, ny) = (-0.15 * x + 0.28 * y, 0.26 * x + 0.24 * y + 0.44);
            x = nx;
            y = ny;
        }

        points.push(Vec2::new(x, y));
    }
    points
}

/// Applies F2 (the dominant affine transform that traces the rachis).
fn apply_f2(p: Vec2) -> Vec2 {
    Vec2::new(0.85 * p.x + 0.04 * p.y, -0.04 * p.x + 0.85 * p.y + 1.6)
}

/// Computes the stem (rachis) as a densely-sampled polyline by iterating F2
/// from the origin and subdividing large gaps.
fn stem_trajectory() -> Vec<Vec2> {
    let mut coarse = Vec::with_capacity(64);
    let mut p = Vec2::ZERO;
    coarse.push(p);
    for _ in 0..63 {
        p = apply_f2(p);
        coarse.push(p);
    }

    // Subdivide so consecutive samples are at most 0.05 apart.
    let max_gap = 0.05;
    let mut fine = Vec::with_capacity(1024);
    for w in coarse.windows(2) {
        let (a, b) = (w[0], w[1]);
        let steps = ((b - a).length() / max_gap).ceil() as u32;
        for i in 0..steps {
            fine.push(a.lerp(b, i as f32 / steps as f32));
        }
    }
    if let Some(&last) = coarse.last() {
        fine.push(last);
    }
    fine
}

/// Interpolates the stem's x-coordinate at a given y.
fn stem_x_at_y(stem: &[Vec2], y: f32) -> Option<f32> {
    let i = stem.windows(2).position(|w| w[0].y <= y && y <= w[1].y)?;
    let (a, b) = (stem[i], stem[i + 1]);
    let t = if (b.y - a.y).abs() < f32::EPSILON {
        0.0
    } else {
        (y - a.y) / (b.y - a.y)
    };
    Some(a.x.lerp(b.x, t))
}

/// Maps raw f32 IFS points onto an integer grid, classifies each as stem or
/// pinna, and stamps a small disc around each to fill gaps between frond lines.
fn rasterize(points: &[Vec2], center: IVec2, size: IVec2) -> Vec<FernPoint> {
    if points.is_empty() {
        return vec![];
    }

    let (mut min, mut max) = (Vec2::splat(f32::MAX), Vec2::splat(f32::MIN));
    for &p in points {
        min = min.min(p);
        max = max.max(p);
    }

    let extent = max - min;
    let scale = match (extent.x == 0.0, extent.y == 0.0) {
        (true, true) => 1.0,
        (true, false) => (size.y - 1) as f32 / extent.y,
        (false, true) => (size.x - 1) as f32 / extent.x,
        _ => ((size.x - 1) as f32 / extent.x).min((size.y - 1) as f32 / extent.y),
    };

    let scaled_extent = extent * scale;
    let offset = Vec2::new(
        center.x as f32 - scaled_extent.x / 2.0,
        center.y as f32 - scaled_extent.y / 2.0,
    );

    let stem = stem_trajectory();
    let pixel_raw = if scale > 0.0 { 1.0 / scale } else { 1.0 };
    let stem_threshold_sq = (pixel_raw * 2.0).powi(2);

    let classify = |p: Vec2| -> FernElement {
        let dist_sq = stem
            .iter()
            .map(|&s| (p - s).length_squared())
            .fold(f32::MAX, f32::min);

        if dist_sq < stem_threshold_sq {
            FernElement::Stem
        } else if p.x < stem_x_at_y(&stem, p.y).unwrap_or(0.0) {
            FernElement::LeftPinna
        } else {
            FernElement::RightPinna
        }
    };

    let mut cells = HashMap::<IVec2, FernElement>::with_capacity(points.len());
    let radius = 2_i32;
    let r2 = radius * radius;

    for &p in points {
        let element = classify(p);
        let n = (p - min) * scale + offset;
        let (cx, cy) = (n.x.round() as i32, n.y.round() as i32);

        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx * dx + dy * dy > r2 {
                    continue;
                }
                cells
                    .entry(IVec2::new(cx + dx, cy + dy))
                    .and_modify(|e| {
                        if element == FernElement::Stem {
                            *e = FernElement::Stem;
                        }
                    })
                    .or_insert(element);
            }
        }
    }

    // Fill stem gaps with Bresenham lines.
    let stem_pixels: Vec<IVec2> = stem
        .iter()
        .map(|&s| {
            let n = (s - min) * scale + offset;
            IVec2::new(n.x.round() as i32, n.y.round() as i32)
        })
        .collect();

    for w in stem_pixels.windows(2) {
        for p in bresenham_line(w[0], w[1]) {
            cells.entry(p).or_insert(FernElement::Stem);
        }
    }

    cells
        .into_iter()
        .map(|(position, element)| FernPoint { position, element })
        .collect()
}

fn bresenham_line(a: IVec2, b: IVec2) -> Vec<IVec2> {
    let mut points = Vec::new();
    let dx = (b.x - a.x).abs();
    let dy = -(b.y - a.y).abs();
    let sx = if a.x < b.x { 1 } else { -1 };
    let sy = if a.y < b.y { 1 } else { -1 };
    let mut err = dx + dy;
    let (mut x, mut y) = (a.x, a.y);

    loop {
        points.push(IVec2::new(x, y));
        if x == b.x && y == b.y {
            break;
        }
        let e2 = 2 * err;
        if e2 >= dy {
            err += dy;
            x += sx;
        }
        if e2 <= dx {
            err += dx;
            y += sy;
        }
    }
    points
}

pub fn spawn_barnsley(
    trigger: On<SpawnBarnsleyEvent>,
    mut particles: MessageWriter<SpawnParticleSignal>,
    mut rng: ResMut<GlobalRng>,
) {
    let ev = trigger.event();
    let fern = generate_fern(
        &mut rng,
        ev.center,
        ev.size,
        ev.f1,
        ev.f2,
        ev.f3,
        ev.num_iterations,
    );

    let wood = ParticleType::new("Wood Wall");
    let grass = ParticleType::new("Grass Wall");

    for point in &fern {
        let particle = match point.element {
            FernElement::Stem => wood.clone(),
            _ => grass.clone(),
        };
        particles.write(SpawnParticleSignal::new(particle, point.position));
    }
}
