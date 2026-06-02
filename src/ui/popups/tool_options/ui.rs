use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self},
};

use crate::{
    tools::{
        PreviousSelectedTool, SelectedTool, brush::BrushOptions, earthquake::EarthquakeOptions,
        select::SelectOptions,
    },
    ui::{
        PopupState, ShowUi, ToolOptionsWindowState, UiSystems, show_brush_options,
        show_earthquake_options, show_select_options,
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
    pub earthquake: EarthquakeOptions<'w>,
}

fn show(
    mut contexts: EguiContexts,
    commands: Commands,
    selected_tool: Res<PreviousSelectedTool>,
    tool_options: ToolOptions,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Tool Options")
        .constrain_to(ctx.available_rect())
        .show(ctx, |ui| {
            match selected_tool.0 {
                SelectedTool::Brush => show_brush_options(ui, tool_options.brush),
                SelectedTool::Select => show_select_options(ui, tool_options.select),
                SelectedTool::Earthquake => {
                    show_earthquake_options(ui, commands, tool_options.earthquake)
                }
            };
        });

    Ok(())
}
