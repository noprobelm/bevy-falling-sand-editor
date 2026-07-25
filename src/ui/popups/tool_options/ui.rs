use bevy::{ecs::system::ParamSet, prelude::*, reflect::enums::Enum};
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self},
};

use crate::{
    tools::{
        PreviousSelectedTool, SelectedTool, earthquake::EarthquakeOptions, painter::PainterOptions,
        select::SelectOptions,
    },
    ui::{
        PopupState, ShowUi, ToolOptionsWindowState, UiSystems, show_earthquake_options,
        show_painter_options, show_select_options,
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

fn show(
    mut contexts: EguiContexts,
    commands: Commands,
    selected_tool: Res<PreviousSelectedTool>,
    mut brush_options: ParamSet<(PainterOptions, EarthquakeOptions)>,
    select_options: SelectOptions,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let title = format!("Tool Options - {}", selected_tool.0.variant_name());
    egui::Window::new(title)
        .constrain_to(ctx.content_rect())
        .show(ctx, |ui| {
            match selected_tool.0 {
                SelectedTool::Painter => show_painter_options(ui, brush_options.p0()),
                SelectedTool::Select => show_select_options(ui, select_options),
                SelectedTool::Earthquake => {
                    show_earthquake_options(ui, commands, brush_options.p1())
                }
            };
        });

    Ok(())
}
