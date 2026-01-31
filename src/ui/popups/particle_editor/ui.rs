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
    pub particle_types: Query<'w, 's, (&'static mut ParticleType, &'static MaterialState)>,
}

fn show(
    mut contexts: EguiContexts,
    mut is_on: Local<bool>,
    selected_particle: Option<Res<SelectedEditorParticle>>,
    editor_params: ParticleEditorParams,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Particle Editor").show(ctx, |ui| {
        show_top_options(ui, &mut is_on);

        ui.separator();

        show_editor(ui, selected_particle, editor_params);
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
    selected_particle: Option<Res<SelectedEditorParticle>>,
    editor_params: ParticleEditorParams,
) {
    ui.columns(2, |columns| {
        show_material_labels(&mut columns[0], &editor_params.material_labels);

        columns[1].horizontal(|ui| {
            ui.add(egui::Separator::default().vertical());
            show_editing_area(ui, selected_particle, editor_params);
        });
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
    selected_particle: Option<Res<SelectedEditorParticle>>,
    mut editor_params: ParticleEditorParams,
) {
    egui::ScrollArea::vertical()
        .id_salt("editing_area")
        .show(ui, |ui| {
            if selected_particle.is_none() {
                ui.label("No particle selected for editing.");
                return;
            }

            let selected_particle = selected_particle.unwrap();
            let entity = editor_params
                .registry
                .get(&selected_particle.0.name)
                .expect("Invalid particle for SelectedEditorParticle: {selected_particle.0}");
            let (mut particle_type, material) = editor_params
                .particle_types
                .get_mut(*entity)
                .expect("No matching query found for selected particle");

            ui.columns(2, |columns| {
                show_particle_type_text_edit(&mut columns[0], &selected_particle, particle_type);
                let selected_material = show_material_combo_box(&mut columns[1], material);
            });
        });
}

fn show_particle_type_text_edit(
    ui: &mut egui::Ui,
    selected_particle: &SelectedEditorParticle,
    mut particle_type: Mut<'_, ParticleType>,
) {
    let mut name = selected_particle.0.name.to_string();
    ui.add(egui::Label::new("Name: "));
    ui.add(egui::TextEdit::singleline(&mut name));
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
    selection
}
