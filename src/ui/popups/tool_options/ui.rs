use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self},
};

use crate::{
    tools::{PreviousSelectedTool, SelectedTool, brush::BrushOptions, select::SelectOptions},
    ui::{
        PopupState, ShowUi, ToolOptionsWindowState, UiSystems, show_brush_settings,
        show_select_settings,
    },
};

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .run_if(in_state(PopupState::<ToolOptionsWindowState>::Open))
                .in_set(UiSystems::ToolOptions),
        );
    }
}

#[derive(SystemParam)]
struct ToolOptions<'w, 's> {
    pub brush: BrushOptions<'w, 's>,
    pub select: SelectOptions<'w>,
}

fn show(
    mut contexts: EguiContexts,
    selected_tool: Res<PreviousSelectedTool>,
    tool_options: ToolOptions,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Tool Options")
        .constrain_to(ctx.available_rect())
        .show(ctx, |ui| {
            match selected_tool.0 {
                SelectedTool::Brush => show_brush_settings(ui, tool_options.brush),
                SelectedTool::Select => show_select_settings(ui, tool_options.select),
            };
        });

    Ok(())
}
