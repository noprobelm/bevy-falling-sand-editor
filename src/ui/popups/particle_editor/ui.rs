use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_falling_sand::prelude::*;

use crate::ui::{
    ParticleEditorApplicationState, ParticleMaterialLabels, PopupState, ShowUi, UiSystems,
};

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

fn show(
    mut contexts: EguiContexts,
    mut is_on: Local<bool>,
    material_labels: Res<ParticleMaterialLabels>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Particle Editor").show(ctx, |ui| {
        show_top_options(ui, &mut is_on);

        ui.separator();

        show_editor(ui, &material_labels);
    });

    Ok(())
}

fn show_top_options(ui: &mut egui::Ui, is_on: &mut bool) {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
        ui.label("Link to brush");
        ui.add(crate::ui::widgets::toggle_switch::toggle(is_on));
    });
}

fn show_editor(ui: &mut egui::Ui, material_labels: &ParticleMaterialLabels) {
    ui.columns(3, |columns| {
        show_material_labels(&mut columns[0], material_labels);

        columns[1].add(egui::Separator::default().vertical());

        show_editing_area(&mut columns[2]);
    })
}

fn show_material_labels(ui: &mut egui::Ui, material_labels: &ParticleMaterialLabels) {
    egui::ScrollArea::vertical()
        .id_salt("material_labels")
        .show(ui, |ui| {
            material_labels.categories().for_each(|(heading, items)| {
                egui::CollapsingHeader::new(heading)
                    .default_open(false)
                    .show(ui, |ui| {
                        items
                            .iter()
                            .for_each(|label| if ui.button(label).clicked() {});
                    });
            });
        });
}

fn show_editing_area(ui: &mut egui::Ui) {
    egui::ScrollArea::vertical()
        .id_salt("editing_area")
        .show(ui, |ui| {
            ui.label("Right col");
        });
}
