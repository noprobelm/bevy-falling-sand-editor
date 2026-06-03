use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::tools::brush::{ToolBrushConfiguration, ToolBrushSize};

pub(super) const EARTHQUAKE_BRUSH_DEFAULT_SIZE: f32 = 24.0;
pub(super) const EARTHQUAKE_BRUSH_MIN_SIZE: f32 = 1.0;

#[derive(Resource, Clone, Debug, Reflect, Serialize, Deserialize)]
#[serde(default)]
pub struct EarthquakeConfiguration {
    pub brush: ToolBrushConfiguration,
    pub voronoi_cells_per_area: f32,
    pub voronoi_min_sites: usize,
    pub voronoi_max_sites: usize,
    pub min_fracture_body_cells: usize,
    pub rigid_body_render_z: f32,
    pub collision_margin: f32,
    pub debug_gizmo_duration_secs: f32,
    pub debug_region_color: Color,
    pub debug_fracture_color: Color,
}

impl Default for EarthquakeConfiguration {
    fn default() -> Self {
        Self {
            brush: ToolBrushConfiguration::default()
                .with_max_size(256.0)
                .with_resize_step(1.0)
                .with_color(Color::srgba(1.0, 1.0, 1.0, 0.3)),
            voronoi_cells_per_area: 0.01,
            voronoi_min_sites: 8,
            voronoi_max_sites: 256,
            min_fracture_body_cells: 2,
            rigid_body_render_z: 1.0,
            collision_margin: 0.1,
            debug_gizmo_duration_secs: 5.0,
            debug_region_color: Color::WHITE,
            debug_fracture_color: Color::srgba(1.0, 0.4, 0.2, 1.0),
        }
    }
}

impl EarthquakeConfiguration {
    pub(super) fn voronoi_site_count(&self, area_hint: f32, particle_count: usize) -> usize {
        let min_sites = self.voronoi_min_sites.min(self.voronoi_max_sites);
        let max_sites = self.voronoi_min_sites.max(self.voronoi_max_sites);
        let target_count = (area_hint.max(0.0) * self.voronoi_cells_per_area.max(0.0)) as usize;
        target_count.clamp(min_sites, max_sites).min(particle_count)
    }

    pub(super) fn min_fracture_body_cells(&self) -> usize {
        self.min_fracture_body_cells.max(1)
    }

    pub(super) fn debug_gizmo_duration_secs(&self) -> f32 {
        self.debug_gizmo_duration_secs.max(0.0)
    }

    pub(super) fn debug_region_color_with_alpha(&self, alpha: f32) -> Color {
        color_with_alpha(self.debug_region_color, alpha)
    }

    pub(super) fn debug_fracture_color_with_alpha(&self, alpha: f32) -> Color {
        color_with_alpha(self.debug_fracture_color, alpha)
    }

    pub(super) fn resized_brush_size(&self, size: &mut ToolBrushSize, delta: f32) {
        self.brush.resize(size, EARTHQUAKE_BRUSH_MIN_SIZE, delta);
    }
}

fn color_with_alpha(color: Color, alpha: f32) -> Color {
    let srgba = color.to_srgba();
    Color::srgba(srgba.red, srgba.green, srgba.blue, srgba.alpha * alpha)
}
