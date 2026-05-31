use bevy::prelude::*;
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self},
};

use crate::ui::{
    BrushSettingsParam, PopupState, ShowUi, ToolOptionsWindowState, UiSystems, show_brush_settings,
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

fn show(mut contexts: EguiContexts, brush_settings: BrushSettingsParam) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Tool Options")
        .constrain_to(ctx.available_rect())
        .show(ctx, |ui| show_brush_settings(ui, brush_settings));

    Ok(())
}
