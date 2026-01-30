use bevy::{ecs::system::SystemParam, prelude::*};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_falling_sand::prelude::*;

use crate::ui::{ParticleEditorApplicationState, PopupState, ShowUi, UiSystems};

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .run_if(in_state(PopupState::<ParticleEditorApplicationState>::Open))
                .in_set(UiSystems::ParticleEditor),
        );
    }
}

fn show(mut contexts: EguiContexts, mut is_on: Local<bool>) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Particle Editor").show(ctx, |ui| {
        ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
            ui.label("Link to brush");
            ui.add(crate::ui::widgets::toggle_switch::toggle(&mut is_on));
        });
    });

    Ok(())
}
