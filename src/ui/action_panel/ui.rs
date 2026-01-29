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
                if ui
                    .add(button_builder(icons.particle_editor, IMAGE_SIZE))
                    .clicked()
                {}

                ui.add_space(IMAGE_MARGIN);

                if ui.add(button_builder(icons.settings, IMAGE_SIZE)).clicked() {}

                ui.add_space(LOWER_MARGIN);
            });
        });
    Ok(())
}

fn button_builder(texture_id: egui::TextureId, image_size: f32) -> egui::Button<'static> {
    let image = egui::Image::new((texture_id, egui::vec2(image_size, image_size)));
    egui::Button::image(image).fill(egui::Color32::TRANSPARENT)
}
