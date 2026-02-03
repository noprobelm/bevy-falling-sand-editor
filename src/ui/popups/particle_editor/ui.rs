use std::{ops::RangeInclusive, time::Duration};

use bevy::{ecs::system::SystemParam, prelude::*, reflect::Enum};
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self, emath::Numeric},
};
use bevy_falling_sand::prelude::*;

use crate::ui::{
    ALL_MATERIAL_STATES, EditorState, ParticleEditorApplicationState, ParticleMaterialLabels,
    PopupState, SelectedParticle, ShowUi, UiSystems,
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
    pub brush: Single<'w, 's, &'static mut crate::brush::SelectedParticle>,
    pub material_labels: Res<'w, ParticleMaterialLabels>,
    pub particle_registry: Res<'w, ParticleTypeRegistry>,
    pub editor_state: ResMut<'w, EditorState>,
    pub particle_types: Query<
        'w,
        's,
        (
            &'static mut ParticleType,
            &'static MaterialState,
            Option<&'static mut Density>,
            Option<&'static mut Speed>,
            Option<&'static Momentum>,
            Option<&'static mut TimedLifetime>,
            Option<&'static mut ChanceLifetime>,
        ),
    >,
}

fn show(
    mut contexts: EguiContexts,
    mut synchronize_brush_selection: Local<Option<bool>>,
    mut selected_particle: Option<ResMut<SelectedParticle>>,
    editor_params: ParticleEditorParams,
) -> Result {
    let synchronize_brush_selection = synchronize_brush_selection.get_or_insert(true);
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Particle Editor").show(ctx, |ui| {
        show_top_options(ui, synchronize_brush_selection);

        ui.separator();

        show_editor(
            ui,
            &mut selected_particle,
            editor_params,
            *synchronize_brush_selection,
        );
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
    selected_particle: &mut Option<ResMut<SelectedParticle>>,
    mut editor_params: ParticleEditorParams,
    synchronize_brush_selection: bool,
) {
    ui.columns(2, |columns| {
        show_material_labels(
            &mut columns[0],
            &mut editor_params,
            synchronize_brush_selection,
        );
        show_editing_area(&mut columns[1], selected_particle, editor_params);
    })
}

fn show_material_labels(
    ui: &mut egui::Ui,
    editor_params: &mut ParticleEditorParams,
    synchronize_brush_selection: bool,
) {
    let categories: Vec<_> = editor_params
        .material_labels
        .categories()
        .map(|(h, items)| (h.to_string(), items.clone()))
        .collect();

    let mut selected_label: Option<String> = None;

    egui::ScrollArea::vertical()
        .id_salt("material_labels")
        .show(ui, |ui| {
            for (heading, items) in &categories {
                egui::CollapsingHeader::new(heading)
                    .default_open(false)
                    .show(ui, |ui| {
                        for label in items {
                            if ui.button(label).clicked() {
                                selected_label = Some(label.clone());
                            }
                        }
                    });
            }
        });

    if let Some(label) = selected_label
        && let Some(entity) = editor_params.particle_registry.get(&label)
    {
        editor_params
            .commands
            .insert_resource(SelectedParticle(*entity));
        if synchronize_brush_selection {
            editor_params.brush.0 = Particle::from(label);
        }
    }
}

fn show_editing_area(
    ui: &mut egui::Ui,
    selected_particle: &mut Option<ResMut<SelectedParticle>>,
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
                    let cached = editor_params
                        .editor_state
                        .map
                        .get_mut(&selected_particle.0)
                        .expect("Failed to find particle type entity in editor registry");

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
                        &mut cached.timed_lifetime,
                    );
                    show_chance_lifetime(
                        &mut editor_params.commands,
                        selected_particle.0,
                        ui,
                        chance_lifetime,
                        &mut cached.chance_lifetime,
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
        let new_value = add_label_with_drag_value(ui, "Density", density.0, 0..=u32::MAX, 1.0);
        density.set_if_neq(Density(new_value));
    }
}

fn show_speed(ui: &mut egui::Ui, speed: Option<Mut<'_, Speed>>) {
    if let Some(mut speed) = speed {
        let new_max = add_label_with_slider_and_drag_value(ui, "Max Speed", speed.max, 0..=100);
        if speed.max != new_max {
            speed.max = new_max;
        }

        let new_threshold = add_label_with_slider_and_drag_value(
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
    timed_lifetime: Option<Mut<'_, TimedLifetime>>,
    cached_timed_lifetime: &mut TimedLifetime,
) {
    let enabled = timed_lifetime.is_some();
    let new_value = add_label_with_toggle_switch(ui, "Lifetime (Timed)", enabled);
    if new_value != enabled {
        if new_value {
            commands
                .entity(entity)
                .insert(cached_timed_lifetime.clone());
        } else {
            commands.entity(entity).remove::<TimedLifetime>();
        }
    }
    if let Some(mut lifetime) = timed_lifetime {
        let duration_ms = lifetime.duration().as_millis() as u64;
        let new_value =
            add_label_with_drag_value(ui, "    Timer (ms):", duration_ms, 0..=u64::MAX, 1.0);
        if new_value != duration_ms {
            lifetime.0.set_duration(Duration::from_millis(new_value));
            cached_timed_lifetime
                .0
                .set_duration(Duration::from_millis(new_value));
        }
    }
}

fn show_chance_lifetime(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    chance_lifetime: Option<Mut<'_, ChanceLifetime>>,
    cached_chance_lifetime: &mut ChanceLifetime,
) {
    let enabled = chance_lifetime.is_some();
    let new_value = add_label_with_toggle_switch(ui, "Lifetime (Chance)", enabled);
    if new_value != enabled {
        if new_value {
            commands
                .entity(entity)
                .insert(cached_chance_lifetime.clone());
        } else {
            commands.entity(entity).remove::<ChanceLifetime>();
        }
    }
    if let Some(mut lifetime) = chance_lifetime {
        let new_value = add_label_with_drag_value(
            ui,
            "    Chance (pct):",
            lifetime.chance * 100.,
            0.0..=100.,
            0.1,
        );
        let new_chance = new_value / 100.;
        if (lifetime.chance - new_chance).abs() > f64::EPSILON {
            lifetime.chance = new_chance;
            cached_chance_lifetime.chance = new_chance;
        }
        let duration_ms = lifetime.tick_timer.duration().as_millis() as u64;
        let new_value =
            add_label_with_drag_value(ui, "    Tick Timer (ms):", duration_ms, 0..=u64::MAX, 1.0);
        if new_value != duration_ms {
            lifetime
                .tick_timer
                .set_duration(Duration::from_millis(new_value));
            cached_chance_lifetime
                .tick_timer
                .set_duration(Duration::from_millis(new_value));
        }
    }
}

fn add_label_with_drag_value<Num>(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    value: Num,
    range: RangeInclusive<Num>,
    speed: f64,
) -> Num
where
    Num: Numeric,
{
    ui.label(label);
    add_empty_space(ui);
    let mut drag_value = value;
    ui.add(
        egui::DragValue::new(&mut drag_value)
            .range(range)
            .speed(speed),
    );
    ui.end_row();
    drag_value
}

fn add_label_with_slider_and_drag_value<Num>(
    ui: &mut egui::Ui,
    label: impl Into<egui::WidgetText>,
    value: Num,
    range: RangeInclusive<Num>,
) -> Num
where
    Num: Numeric,
{
    ui.label(label);
    let mut drag_value = value;
    ui.add(egui::Slider::new(&mut drag_value, range.clone()).show_value(false));
    ui.add(egui::DragValue::new(&mut drag_value).range(range));
    ui.end_row();
    drag_value
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

fn add_empty_space(ui: &mut egui::Ui) {
    ui.label("");
}
