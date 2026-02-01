use bevy::{ecs::system::SystemParam, prelude::*, reflect::Enum};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_falling_sand::prelude::*;

use crate::ui::{
    ALL_MATERIAL_STATES, ParticleEditorApplicationState, ParticleMaterialLabels, PopupState,
    SelectedEditorParticle, ShowUi, UiSystems,
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

/// System param to fetch particle types by material type.
#[derive(SystemParam)]
pub struct ParticleEditorParams<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub material_labels: Res<'w, ParticleMaterialLabels>,
    pub registry: Res<'w, ParticleTypeRegistry>,
    pub particle_types: Query<
        'w,
        's,
        (
            &'static mut ParticleType,
            &'static MaterialState,
            Option<&'static mut Density>,
            Option<&'static mut Speed>,
        ),
    >,
}

fn show(
    mut contexts: EguiContexts,
    mut is_on: Local<bool>,
    mut selected_particle: Option<ResMut<SelectedEditorParticle>>,
    editor_params: ParticleEditorParams,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Particle Editor").show(ctx, |ui| {
        show_top_options(ui, &mut is_on);

        ui.separator();

        show_editor(ui, &mut selected_particle, editor_params);
    });

    Ok(())
}

fn show_top_options(ui: &mut egui::Ui, is_on: &mut bool) {
    ui.with_layout(egui::Layout::left_to_right(egui::Align::LEFT), |ui| {
        ui.label("Link to brush");
        ui.add(crate::ui::widgets::toggle_switch::toggle(is_on));
    });
}

fn show_editor(
    ui: &mut egui::Ui,
    selected_particle: &mut Option<ResMut<SelectedEditorParticle>>,
    editor_params: ParticleEditorParams,
) {
    ui.columns(2, |columns| {
        show_material_labels(&mut columns[0], &editor_params.material_labels);
        show_editing_area(&mut columns[1], selected_particle, editor_params);
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

fn show_editing_area(
    ui: &mut egui::Ui,
    selected_particle: &mut Option<ResMut<SelectedEditorParticle>>,
    mut editor_params: ParticleEditorParams,
) {
    egui::ScrollArea::vertical()
        .id_salt("editing_area")
        .show(ui, |ui| {
            let Some(selected_particle) = selected_particle else {
                ui.label("No particle selected for editing.");
                return;
            };

            let (particle_type, material, density, speed) = editor_params
                .particle_types
                .get_mut(selected_particle.0)
                .expect("No matching query found for selected particle");

            egui::Grid::new("editing_grid")
                .num_columns(2)
                .show(ui, |ui| {
                    show_particle_type_text_edit(ui, particle_type);
                    show_material_combo_box(ui, material);
                    show_density(ui, density);
                    show_speed_threshold(ui, speed);
                });
        });
}

fn show_particle_type_text_edit(ui: &mut egui::Ui, mut particle_type: Mut<'_, ParticleType>) {
    let mut name = particle_type.name.to_string();
    ui.label("Name:");
    ui.add(egui::TextEdit::singleline(&mut name));
    ui.end_row();
    particle_type.set_if_neq(name.into());
}

fn show_material_combo_box(ui: &mut egui::Ui, material: &MaterialState) -> MaterialState {
    let mut selection = *material;
    egui::ComboBox::from_id_salt("material_state_combo")
        .selected_text(selection.variant_name())
        .show_ui(ui, |ui| {
            for variant in ALL_MATERIAL_STATES {
                ui.selectable_value(&mut selection, variant, variant.variant_name());
            }
        });
    ui.end_row();
    selection
}

fn show_density(ui: &mut egui::Ui, density: Option<Mut<'_, Density>>) {
    ui.label("Density");
    if let Some(mut density) = density {
        let mut value = density.0.to_string();
        ui.add(egui::TextEdit::singleline(&mut value));
        if let Ok(new) = value.parse::<u32>() {
            density.set_if_neq(Density(new));
        } else if value.len() == 0 {
            density.set_if_neq(Density(0));
        }
    }
    ui.end_row();
}

fn show_speed_threshold(ui: &mut egui::Ui, speed: Option<Mut<'_, Speed>>) {
    ui.label("Speed Increase Threshold").on_hover_ui(|ui| {
        ui.label("Number of turns a particle must move unobstructed before increasing speed.");
    });
    if let Some(mut speed) = speed {
        let mut value = speed.threshold.to_string();
        ui.add(egui::TextEdit::singleline(&mut value));
        if let Ok(new) = value.parse::<u8>() {
            if speed.threshold != new {
                speed.threshold = new;
            }
        } else if value.len() == 0 && speed.threshold != 0 {
            speed.threshold = 0;
        }
    }
    ui.end_row();
}
