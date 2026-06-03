use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::tools::brush::ToolBrushConfiguration;

pub(super) const PAINTER_BRUSH_DEFAULT_SIZE: f32 = 2.0;
pub(super) const PAINTER_BRUSH_MIN_SIZE: f32 = 1.0;

#[derive(Resource, Clone, Debug, Reflect, Serialize, Deserialize)]
#[serde(default)]
pub struct PainterConfiguration {
    pub brush: ToolBrushConfiguration,
}

impl Default for PainterConfiguration {
    fn default() -> Self {
        Self {
            brush: ToolBrushConfiguration::default()
                .with_max_size(50.0)
                .with_resize_step(1.0)
                .with_color(Color::srgba(1.0, 1.0, 1.0, 0.3)),
        }
    }
}
