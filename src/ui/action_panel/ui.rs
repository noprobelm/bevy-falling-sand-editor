use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use super::setup::SidePanelIconTextureIds;
use crate::ui::{ShowUi, UiSystems};

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .run_if(resource_exists::<SidePanelIconTextureIds>)
                .in_set(UiSystems::SidePanel),
        );
    }
}

fn show(mut contexts: EguiContexts, icons: Res<SidePanelIconTextureIds>) -> Result {
    const IMAGE_SIZE: f32 = 32.;
    const WIDGET_WIDTH: f32 = 40.;
    const IMAGE_MARGIN: f32 = 5.;
    const LOWER_MARGIN: f32 = 2.;

    let ctx = contexts.ctx_mut()?;

    egui::Window::new("action window")
        .title_bar(false)
        .resizable(false)
        .min_width(WIDGET_WIDTH)
        .max_width(WIDGET_WIDTH)
        .show(ctx, |ui| {
            ui.style_mut()
                .visuals
                .widgets
                .noninteractive
                .bg_stroke
                .width = 4.0;
            ui.separator();
            ui.add_space(5.);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                let particle_editor_image =
                    egui::Image::new((icons.particle_editor, egui::vec2(IMAGE_SIZE, IMAGE_SIZE)))
                        .tint(egui::Color32::WHITE);
                if ui
                    .add(
                        egui::Button::image(particle_editor_image).fill(egui::Color32::TRANSPARENT),
                    )
                    .clicked()
                {}

                ui.add_space(IMAGE_MARGIN);

                let document_image = egui::Image::new((icons.settings, egui::vec2(32.0, 32.0)));
                if ui
                    .add(egui::Button::image(document_image).fill(egui::Color32::TRANSPARENT))
                    .clicked()
                {}

                ui.add_space(LOWER_MARGIN);
            });
        });
    Ok(())
}
