use std::ops::RangeInclusive;

use bevy::{ecs::system::SystemParam, prelude::*, reflect::Enum};
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self, emath::Numeric},
};
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
            Option<&'static Momentum>,
            Option<&'static TimedLifetime>,
            Option<&'static ChanceLifetime>,
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

            let (
                particle_type,
                material,
                density,
                speed,
                momentum,
                timed_lifetime,
                chance_lifetime,
            ) = editor_params
                .particle_types
                .get_mut(selected_particle.0)
                .expect("No matching query found for selected particle");

            egui::Grid::new("editing_grid")
                .num_columns(3)
                .show(ui, |ui| {
                    show_particle_type_text_edit(ui, particle_type);
                    show_material_combo_box(ui, material);
                    show_density(ui, density);
                    show_speed(ui, speed);
                    show_momentum(
                        &mut editor_params.commands,
                        selected_particle.0,
                        ui,
                        material,
                        momentum,
                    );
                    show_timed_lifetime(
                        &mut editor_params.commands,
                        selected_particle.0,
                        ui,
                        timed_lifetime,
                    );
                    show_chance_lifetime(
                        &mut editor_params.commands,
                        selected_particle.0,
                        ui,
                        chance_lifetime,
                    );
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
    if let Some(mut density) = density {
        let new_value = add_label_with_numeric_text_edit(ui, "Density", density.0, 0..=u32::MAX);
        density.set_if_neq(Density(new_value));
    }
}

fn show_speed(ui: &mut egui::Ui, speed: Option<Mut<'_, Speed>>) {
    if let Some(mut speed) = speed {
        let new_max = add_label_with_slider_and_text_edit(ui, "Max Speed", speed.max, 0..=100);
        if speed.max != new_max {
            speed.max = new_max;
        }

        let new_threshold = add_label_with_slider_and_text_edit(
            ui,
            "Speed Increase Threshold",
            speed.threshold,
            0..=100,
        );
        if speed.threshold != new_threshold {
            speed.threshold = new_threshold;
        }
    }
}

fn show_momentum(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    material: &MaterialState,
    momentum: Option<&Momentum>,
) {
    if material != &MaterialState::Wall {
        let enabled = momentum.is_some();
        let new_value = add_label_with_toggle_switch(ui, "Momentum", enabled);
        if new_value != enabled {
            if new_value {
                commands.entity(entity).insert(Momentum::default());
            } else {
                commands.entity(entity).remove::<Momentum>();
            }
        }
    }
}

fn show_timed_lifetime(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    timed_lifetime: Option<&TimedLifetime>,
) {
    let enabled = timed_lifetime.is_some();
    let new_value = add_label_with_toggle_switch(ui, "Lifetime (Timer)", enabled);
}

fn show_chance_lifetime(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    chance_lifetime: Option<&ChanceLifetime>,
) {
    let enabled = chance_lifetime.is_some();
    let new_value = add_label_with_toggle_switch(ui, "Lifetime (Chance)", enabled);
}

fn add_label_with_numeric_text_edit<Num>(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    value: Num,
    range: RangeInclusive<Num>,
) -> Num
where
    Num: std::fmt::Display + std::str::FromStr + Default + PartialOrd + Copy,
{
    ui.label(label);
    add_empty_space(ui);
    let mut text = value.to_string();
    ui.add(egui::TextEdit::singleline(&mut text));
    let result = if let Ok(new) = text.parse::<Num>() {
        clamp(new, range)
    } else if text.is_empty() {
        Num::default()
    } else {
        value
    };
    ui.end_row();
    result
}

fn add_label_with_slider_and_text_edit<Num>(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    value: Num,
    range: RangeInclusive<Num>,
) -> Num
where
    Num: Numeric + std::fmt::Display + std::str::FromStr + Default + Copy,
{
    ui.label(label);
    let mut slider_value = value;
    ui.add(egui::Slider::new(&mut slider_value, range.clone()).show_value(false));
    let mut text = slider_value.to_string();
    ui.add(egui::TextEdit::singleline(&mut text));
    let result = if let Ok(new) = text.parse::<Num>() {
        clamp_numeric(new, range)
    } else if text.is_empty() {
        Num::default()
    } else {
        slider_value
    };
    ui.end_row();
    result
}

fn add_label_with_toggle_switch(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    mut is_on: bool,
) -> bool {
    ui.label(label);
    add_empty_space(ui);
    ui.add(crate::ui::widgets::toggle_switch::toggle(&mut is_on));
    ui.end_row();
    is_on
}

fn clamp<Num: PartialOrd + Copy>(value: Num, range: RangeInclusive<Num>) -> Num {
    if value < *range.start() {
        return *range.start();
    }
    if value > *range.end() {
        return *range.end();
    }
    value
}

fn clamp_numeric<Num: Numeric + Copy>(value: Num, range: RangeInclusive<Num>) -> Num {
    if value.to_f64() < range.start().to_f64() {
        return *range.start();
    }
    if value.to_f64() > range.end().to_f64() {
        return *range.end();
    }
    value
}

fn add_empty_space(ui: &mut egui::Ui) {
    ui.label("");
}
