use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::tools::{SelectedTool, earthquake::EarthquakeBrush, painter::PainterBrush};

pub(super) struct BrushPlugin;

impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_selected_tool_brush_size);
    }
}

#[derive(
    Component, Copy, Clone, Default, PartialEq, PartialOrd, Debug, Reflect, Serialize, Deserialize,
)]
pub struct ToolBrushSize(pub f32);

#[derive(Component, Clone, Default, PartialEq, Debug, Reflect, Serialize, Deserialize)]
pub struct ToolBrushColor(pub Color);

#[derive(Clone, Debug, Reflect, Serialize, Deserialize)]
#[serde(default)]
pub struct ToolBrushConfiguration {
    pub max_size: f32,
    pub resize_step: f32,
    pub color: Color,
}

impl Default for ToolBrushConfiguration {
    fn default() -> Self {
        Self {
            max_size: 256.0,
            resize_step: 1.0,
            color: Color::srgba(1.0, 1.0, 1.0, 0.3),
        }
    }
}

impl ToolBrushConfiguration {
    pub fn with_max_size(self, max_size: f32) -> Self {
        Self { max_size, ..self }
    }

    pub fn with_resize_step(self, resize_step: f32) -> Self {
        Self {
            resize_step,
            ..self
        }
    }

    pub fn with_color(self, color: Color) -> Self {
        Self { color, ..self }
    }

    pub fn clamped_size(&self, size: f32, min_size: f32) -> f32 {
        let min = min_size.min(self.max_size).max(0.0);
        let max = min_size.max(self.max_size).max(min);
        size.clamp(min, max)
    }

    pub fn resize(&self, size: &mut ToolBrushSize, min_size: f32, delta: f32) {
        if delta > 0.0 {
            size.0 = self.clamped_size(size.0 + self.resize_step.max(0.0), min_size);
        } else if delta < 0.0 {
            size.0 = self.clamped_size(size.0 - self.resize_step.max(0.0), min_size);
        }
    }
}

#[derive(Event, Message, Clone, Copy, PartialEq, PartialOrd, Debug)]
pub struct SetSelectedToolBrushSize(pub f32);

fn on_set_selected_tool_brush_size(
    trigger: On<SetSelectedToolBrushSize>,
    selected_tool: Res<State<SelectedTool>>,
    mut painter_brush: Query<&mut ToolBrushSize, (With<PainterBrush>, Without<EarthquakeBrush>)>,
    mut earthquake_brush: Query<&mut ToolBrushSize, (With<EarthquakeBrush>, Without<PainterBrush>)>,
) -> Result {
    let size = trigger.event().0;

    match selected_tool.get() {
        SelectedTool::Painter => {
            painter_brush.single_mut()?.0 = size;
        }
        SelectedTool::Earthquake => {
            earthquake_brush.single_mut()?.0 = size;
        }
        SelectedTool::Select => {
            error!("Selected tool does not have a brush");
        }
    }

    Ok(())
}
