use bevy::platform::collections::{HashMap, HashSet};
use bevy::prelude::*;
use bevy_turborand::{GlobalRng, TurboRand};
use voronoice::{BoundingBox, Point, VoronoiBuilder};

use crate::tools::earthquake::{EarthquakeConfiguration, EarthquakeRegion};

pub(super) struct GeneratedVoronoiCell {
    pub(super) particles: HashSet<IVec2>,
    pub(super) vertices: Vec<Vec2>,
}

pub(super) fn generate_voronoi_cells(
    rng: &mut GlobalRng,
    config: &EarthquakeConfiguration,
    region: &EarthquakeRegion,
    bounds: IRect,
    particles: &[IVec2],
) -> Vec<GeneratedVoronoiCell> {
    if particles.is_empty() {
        return Vec::new();
    }

    let site_count = config.voronoi_site_count(region.area_hint(), particles.len());

    let sites: Vec<Point> = rng
        .as_mut()
        .sample_multiple(particles, site_count)
        .into_iter()
        .map(|p| {
            let center = p.as_vec2() + Vec2::splat(0.5);
            Point {
                x: center.x as f64,
                y: center.y as f64,
            }
        })
        .collect();

    let bbox = voronoi_bounding_box(bounds);

    let Some(voronoi) = VoronoiBuilder::default()
        .set_sites(sites)
        .set_bounding_box(bbox)
        .build()
    else {
        return Vec::new();
    };

    let final_sites = voronoi.sites();
    let vertices_by_site: Vec<Vec<Vec2>> = voronoi
        .iter_cells()
        .map(|cell| {
            cell.iter_vertices()
                .map(|point| Vec2::new(point.x as f32, point.y as f32))
                .collect()
        })
        .collect();
    let mut cells: HashMap<usize, HashSet<IVec2>> = HashMap::default();
    for &p in particles {
        let center = p.as_vec2() + Vec2::splat(0.5);
        let mut best_idx: usize = 0;
        let mut best_dist = f32::INFINITY;
        for (i, site) in final_sites.iter().enumerate() {
            let dx = center.x - site.x as f32;
            let dy = center.y - site.y as f32;
            let d = dx * dx + dy * dy;
            if d < best_dist {
                best_dist = d;
                best_idx = i;
            }
        }
        cells.entry(best_idx).or_default().insert(p);
    }

    cells
        .into_iter()
        .map(|(site, particles)| GeneratedVoronoiCell {
            particles,
            vertices: vertices_by_site.get(site).cloned().unwrap_or_default(),
        })
        .collect()
}

fn voronoi_bounding_box(bounds: IRect) -> BoundingBox {
    let min = bounds.min.as_vec2();
    let max = bounds.max.as_vec2();
    let center = (min + max) * 0.5;
    let size = (max - min).max(Vec2::splat(1.0));
    BoundingBox::new(
        Point {
            x: center.x as f64,
            y: center.y as f64,
        },
        size.x as f64,
        size.y as f64,
    )
}
