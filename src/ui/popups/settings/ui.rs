use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use crate::ui::{PopupState, SettingsApplicationState, ShowUi, UiSystems};

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .run_if(in_state(PopupState::<SettingsApplicationState>::Open))
                .in_set(UiSystems::Settings),
        );
    }
}

fn show(mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Settings").show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |_ui| {});
    });

    Ok(())
}
