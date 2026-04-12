use bevy::{ecs::system::SystemParam, prelude::*, reflect::Enum};
use bevy_egui::{
    EguiContexts, EguiPrimaryContextPass,
    egui::{self},
};
use bevy_falling_sand::prelude::*;
use std::time::Duration;

use crate::{
    chunk_effects::{BurnEffect, GasEffect, GlowEffect, LiquidEffect},
    config::ParticleTypesFile,
    particles::ParticleCategory,
    ui::*,
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
    pub category_labels: Res<'w, ParticleCategoryLabels>,
    pub particle_registry: Res<'w, ParticleTypeRegistry>,
    pub editor_state: ResMut<'w, EditorState>,
    pub msgw_reset_particle_type: MessageWriter<'w, SyncParticleTypeChildrenSignal>,
    pub particle_types_file: Res<'w, ParticleTypesFile>,
    pub msgw_save_particle: MessageWriter<'w, PersistParticleTypesSignal>,
    pub particles_saved_msg_config: Res<'w, ParticleTypesSavedMessageConfiguration>,
    pub particle_types_recently_saved: Option<Res<'w, ParticleTypesRecentlySaved>>,
}

fn show(
    mut contexts: EguiContexts,
    synchronize_brush_state: Res<State<SynchronizeBrushState>>,
    mut next_synchronize_brush_state: ResMut<NextState<SynchronizeBrushState>>,
    mut selected_particle: Option<ResMut<SelectedParticle>>,
    mut editor_params: ParticleEditorParams,
    particle_query: Query<ParticleDataQuery>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Particle Editor")
        .constrain_to(ctx.available_rect())
        .show(ctx, |ui| {
            show_top_options(
                ui,
                &synchronize_brush_state,
                &mut next_synchronize_brush_state,
                selected_particle.as_ref().map(|s| s.0),
                &mut editor_params.msgw_reset_particle_type,
            );

            ui.separator();

            show_editor(
                ui,
                &mut selected_particle,
                editor_params,
                synchronize_brush_state,
                particle_query,
            );
        });

    Ok(())
}

fn show_top_options(
    ui: &mut egui::Ui,
    synchronize_brush_state: &Res<State<SynchronizeBrushState>>,
    next_synchronize_brush_state: &mut ResMut<NextState<SynchronizeBrushState>>,
    selected_entity: Option<Entity>,
    msgw_reset: &mut MessageWriter<SyncParticleTypeChildrenSignal>,
) {
    ui.horizontal(|ui| {
        ui.label("Link to brush");
        let mut is_on = match synchronize_brush_state.get() {
            SynchronizeBrushState::Enabled => true,
            SynchronizeBrushState::Disabled => false,
        };
        if ui
            .add(crate::ui::widgets::toggle_switch::toggle(&mut is_on))
            .changed()
        {
            if is_on {
                next_synchronize_brush_state.set(SynchronizeBrushState::Enabled)
            } else {
                next_synchronize_brush_state.set(SynchronizeBrushState::Disabled)
            }
        };
        if let Some(entity) = selected_entity {
            let remaining = ui.available_width();
            ui.add_space(remaining - 110.0_f32.min(remaining));
            if ui.button("Propagate To All").clicked() {
                msgw_reset.write(SyncParticleTypeChildrenSignal::from_parent_handle(entity));
            }
        }
    });
}

fn show_editor(
    ui: &mut egui::Ui,
    selected_particle: &mut Option<ResMut<SelectedParticle>>,
    mut editor_params: ParticleEditorParams,
    synchronize_brush_selection: Res<State<SynchronizeBrushState>>,
    particle_query: Query<ParticleDataQuery>,
) {
    ui.columns(2, |columns| {
        show_category_labels(
            &mut columns[0],
            &mut editor_params,
            synchronize_brush_selection,
        );
        show_editing_area(
            &mut columns[1],
            selected_particle,
            &mut editor_params,
            particle_query,
        );
    });
    ui.separator();
    ui.horizontal(|ui| {
        if ui.button("Save").clicked() {
            editor_params
                .msgw_save_particle
                .write(PersistParticleTypesSignal(
                    editor_params.particle_types_file.0.clone(),
                ));
        };
        if let Some(recently_saved) = editor_params.particle_types_recently_saved {
            let colors = editor_params.particles_saved_msg_config.colors;
            let fade_factor = recently_saved.timer.remaining().as_secs_f32()
                / recently_saved.timer.duration().as_secs_f32();
            ui.label(
                egui::RichText::new(format!(
                    "Particle defs saved to {:?}",
                    recently_saved.path.as_os_str()
                ))
                .color(egui::Color32::from_rgba_unmultiplied(
                    colors[0],
                    colors[1],
                    colors[2],
                    (fade_factor * 255.) as u8,
                )),
            );
        }
    });
}

fn show_category_labels(
    ui: &mut egui::Ui,
    editor_params: &mut ParticleEditorParams,
    synchronize_brush_selection: Res<State<SynchronizeBrushState>>,
) {
    let categories: Vec<_> = editor_params
        .category_labels
        .categories()
        .map(|(h, items)| (h.to_string(), items.clone()))
        .collect();

    let mut selected_label: Option<String> = None;

    egui::ScrollArea::vertical()
        .id_salt("category_labels")
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
        if synchronize_brush_selection.get() == &SynchronizeBrushState::Enabled {
            editor_params.brush.0 = Particle::from(label);
        }
    }
}

fn show_editing_area(
    ui: &mut egui::Ui,
    selected_particle: &mut Option<ResMut<SelectedParticle>>,
    editor_params: &mut ParticleEditorParams,
    mut particle_query: Query<ParticleDataQuery>,
) {
    egui::ScrollArea::vertical()
        .id_salt("editing_area")
        .show(ui, |ui| {
            egui::Frame::NONE
                .inner_margin(egui::Margin {
                    right: 12,
                    ..Default::default()
                })
                .show(ui, |ui| {
                    let Some(selected_particle) = selected_particle else {
                        ui.label("No particle selected for editing.");
                        return;
                    };

                    let data = particle_query
                        .get_mut(selected_particle.0)
                        .expect("No matching query found for selected particle");
                    let (particle_type, timed_lifetime, chance_lifetime) = (
                        data.core.particle_type,
                        data.core.timed_lifetime,
                        data.core.chance_lifetime,
                    );
                    let (
                        mut movement,
                        density,
                        speed,
                        momentum,
                        resistor,
                        category,
                        air_resistance,
                    ) = (
                        data.movement.movement,
                        data.movement.density,
                        data.movement.speed,
                        data.movement.momentum,
                        data.movement.resistor,
                        data.movement.category,
                        data.movement.air_resistance,
                    );
                    let mut color_profile = data.color.profile;
                    let burns = data.reactions.burns;
                    let contact_reaction = data.reactions.contact_reaction;
                    let (liquid_effect, gas_effect, glow_effect, burn_effect) = (
                        data.effects.liquid,
                        data.effects.gas,
                        data.effects.glow,
                        data.effects.burn,
                    );

                    let state = editor_params
                        .editor_state
                        .map
                        .get_mut(&selected_particle.0)
                        .expect("Failed to find particle type entity in editor registry");

                    egui::Grid::new("identity_grid")
                        .num_columns(2)
                        .show(ui, |ui| {
                            show_particle_type_text_edit(ui, particle_type);
                            show_category(
                                &mut editor_params.commands,
                                selected_particle.0,
                                ui,
                                category,
                            );
                        });

                    egui::CollapsingHeader::new("Movement")
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new("movement_grid")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    show_movement_toggle(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        &mut movement,
                                        &mut state.cached_movement.movement,
                                    );
                                    if let Some(ref mut movement) = movement {
                                        show_neighbor_groups(
                                            ui,
                                            movement,
                                            &mut air_resistance
                                                .expect("No air resistance found on particle!"),
                                        );
                                    }
                                    show_density(ui, density);
                                    show_speed(ui, speed);
                                    show_momentum(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        movement.is_some(),
                                        momentum,
                                    );
                                    show_resistor(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        resistor,
                                        &mut state.cached_movement.resistor,
                                    );
                                });
                        });

                    egui::CollapsingHeader::new("Lifetime")
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new("lifetime_grid")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    show_timed_lifetime(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        timed_lifetime,
                                        &mut state.timed_lifetime,
                                    );
                                    show_chance_lifetime(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        chance_lifetime,
                                        &mut state.chance_lifetime,
                                    );
                                });
                        });

                    egui::CollapsingHeader::new("Color")
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new("color_grid").num_columns(2).show(ui, |ui| {
                                show_color_source(
                                    ui,
                                    &mut color_profile.source,
                                    &mut state.palette,
                                    &mut state.gradient,
                                );
                                show_color_assignment(ui, &mut color_profile.assignment);
                                show_palette_colors(ui, &mut color_profile.source);
                            });
                        });

                    egui::CollapsingHeader::new("Visual Effects")
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new("effects_grid")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    show_effect_overlays(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        liquid_effect,
                                        gas_effect,
                                        glow_effect,
                                        burn_effect,
                                    );
                                });
                        });

                    egui::CollapsingHeader::new("Flammability")
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new("flammability_grid")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    show_flammability(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        burns,
                                        &mut state.burns,
                                    );
                                });
                        });

                    egui::CollapsingHeader::new("Contact Reactions")
                        .default_open(false)
                        .show(ui, |ui| {
                            egui::Grid::new("contact_grid")
                                .num_columns(2)
                                .show(ui, |ui| {
                                    show_contact_reactions(
                                        &mut editor_params.commands,
                                        selected_particle.0,
                                        ui,
                                        contact_reaction,
                                        &mut state.contact_reaction,
                                    );
                                });
                        });
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

fn show_category(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    category: Option<Mut<'_, ParticleCategory>>,
) {
    ui.label("Category:");
    let current = category.as_ref().map(|c| c.0.clone()).unwrap_or_default();
    let mut text = current.clone();
    let response = ui.add(egui::TextEdit::singleline(&mut text));
    ui.end_row();

    if response.changed() {
        if text.is_empty() {
            commands.entity(entity).remove::<ParticleCategory>();
        } else {
            commands.entity(entity).insert(ParticleCategory(text));
        }
    }
}

fn show_movement_toggle(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    movement: &mut Option<Mut<'_, Movement>>,
    movement_state: &mut Movement,
) {
    let enabled = movement.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Movement", enabled);
    if new_value != enabled {
        if new_value {
            commands.entity(entity).insert(movement_state.clone());
        } else {
            if let Some(m) = movement.as_ref() {
                *movement_state = (**m).clone();
            }
            commands.entity(entity).remove::<Movement>();
            *movement = None;
        }
    }
}

fn show_density(ui: &mut egui::Ui, density: Option<Mut<'_, Density>>) {
    if let Some(mut density) = density {
        let new_value = add_label_with_drag_value(ui, 0, "Density", density.0, 0..=u32::MAX, 1.0);
        density.set_if_neq(Density(new_value));
    }
}

fn show_speed(ui: &mut egui::Ui, speed: Option<Mut<'_, Speed>>) {
    if let Some(mut speed) = speed {
        let new_max =
            add_label_with_drag_value(ui, 0, "Max Speed", speed.max_speed(), 0..=100, 1.0);
        if speed.max_speed() != new_max {
            speed.set_max_speed(new_max);
        }

        let new_threshold = add_label_with_drag_value(
            ui,
            0,
            "Speed Increase Threshold",
            speed.threshold(),
            0..=100,
            1.0,
        );
        if speed.threshold() != new_threshold {
            speed.set_threshold(new_threshold);
        }
    }
}

fn show_momentum(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    has_movement: bool,
    momentum: Option<&Momentum>,
) {
    if has_movement {
        let enabled = momentum.is_some();
        let new_value = add_label_with_toggle_switch(ui, 0, "Momentum", enabled);
        if new_value != enabled {
            if new_value {
                commands.entity(entity).insert(Momentum::default());
            } else {
                commands.entity(entity).remove::<Momentum>();
            }
        }
    }
}

fn show_resistor(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    resistor: Option<&ParticleResistor>,
    resistor_state: &mut ParticleResistor,
) {
    let enabled = resistor.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "ParticleResistor", enabled);
    if new_value != enabled {
        if new_value {
            commands.entity(entity).insert(*resistor_state);
        } else {
            commands.entity(entity).remove::<ParticleResistor>();
        }
    }
    if let Some(resistor) = resistor {
        let value = resistor.0;
        let new_value = add_label_with_drag_value(ui, 0, "    Resistance", value, 0.0..=1.0, 0.01);
        if (new_value - value).abs() > f64::EPSILON {
            commands.entity(entity).insert(ParticleResistor(new_value));
            resistor_state.0 = new_value;
        }
    }
}

fn show_timed_lifetime(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    timed_lifetime: Option<Mut<'_, TimedLifetime>>,
    lifetime_state: &mut TimedLifetime,
) {
    let enabled = timed_lifetime.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Lifetime (Timed)", enabled);
    if new_value != enabled {
        if new_value {
            commands.entity(entity).insert(lifetime_state.clone());
        } else {
            commands.entity(entity).remove::<TimedLifetime>();
        }
    }
    if let Some(mut lifetime) = timed_lifetime {
        let duration_ms = lifetime.duration().as_millis() as u64;
        let new_value =
            add_label_with_drag_value(ui, 0, "    Timer (ms):", duration_ms, 0..=u64::MAX, 1.0);
        if new_value != duration_ms {
            lifetime.0.set_duration(Duration::from_millis(new_value));
            lifetime_state
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
    lifetime_state: &mut ChanceLifetime,
) {
    let enabled = chance_lifetime.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Lifetime (Chance)", enabled);
    if new_value != enabled {
        if new_value {
            commands.entity(entity).insert(lifetime_state.clone());
        } else {
            commands.entity(entity).remove::<ChanceLifetime>();
        }
    }
    if let Some(mut lifetime) = chance_lifetime {
        let new_value = add_label_with_drag_value(
            ui,
            0,
            "    Chance (pct):",
            lifetime.chance * 100.,
            0.0..=100.,
            0.1,
        );
        let new_chance = new_value / 100.;
        if (lifetime.chance - new_chance).abs() > f64::EPSILON {
            lifetime.chance = new_chance;
            lifetime_state.chance = new_chance;
        }
        let duration_ms = lifetime.tick_timer.duration().as_millis() as u64;
        let new_value = add_label_with_drag_value(
            ui,
            0,
            "    Tick Timer (ms):",
            duration_ms,
            0..=u64::MAX,
            1.0,
        );
        if new_value != duration_ms {
            lifetime
                .tick_timer
                .set_duration(Duration::from_millis(new_value));
            lifetime_state
                .tick_timer
                .set_duration(Duration::from_millis(new_value));
        }
    }
}

fn show_color_source(
    ui: &mut egui::Ui,
    color_source: &mut ColorSource,
    cached_palette: &mut Palette,
    cached_gradient: &mut ColorGradient,
) {
    ui.label("Color Source: ");
    egui::ComboBox::from_id_salt("color_source_combo")
        .selected_text(color_source.variant_name())
        .show_ui(ui, |ui| {
            let changed = ui
                .selectable_label(matches!(color_source, ColorSource::Palette(_)), "Palette")
                .clicked()
                || ui
                    .selectable_label(matches!(color_source, ColorSource::Gradient(_)), "Gradient")
                    .clicked();

            if changed {
                match color_source {
                    ColorSource::Palette(palette) => {
                        *cached_palette = palette.clone();
                        *color_source = ColorSource::Gradient(cached_gradient.clone());
                    }
                    ColorSource::Gradient(gradient) => {
                        *cached_gradient = gradient.clone();
                        *color_source = ColorSource::Palette(cached_palette.clone());
                    }
                    ColorSource::Texture(_) => {
                        *color_source = ColorSource::Palette(cached_palette.clone());
                    }
                }
            }
        });
    ui.end_row();
}

fn show_color_assignment(ui: &mut egui::Ui, color_assignment: &mut ColorAssignment) {
    ui.label("Color Assignment:");
    egui::ComboBox::from_id_salt("color_assignment_combo")
        .selected_text(color_assignment.variant_name())
        .show_ui(ui, |ui| {
            ui.selectable_value(color_assignment, ColorAssignment::Sequential, "Sequential");
            ui.selectable_value(color_assignment, ColorAssignment::Random, "Random");
        });
    ui.end_row();
}

fn show_palette_colors(ui: &mut egui::Ui, color_source: &mut ColorSource) {
    if let ColorSource::Palette(palette) = color_source {
        ui.label("    Palette Colors");
        if ui.button("Add Color").clicked() {
            let new_color = palette
                .colors
                .last()
                .copied()
                .unwrap_or(Color::srgba_u8(255, 255, 255, 255));
            palette.colors.push(new_color);
        }
        ui.end_row();

        let mut to_remove: Option<usize> = None;
        let colors_len = palette.colors.len();
        for (i, color) in palette.colors.iter_mut().enumerate() {
            let srgba = color.to_srgba();
            let original = egui::Color32::from_rgba_unmultiplied(
                (srgba.red * 255.0) as u8,
                (srgba.green * 255.0) as u8,
                (srgba.blue * 255.0) as u8,
                (srgba.alpha * 255.0) as u8,
            );
            let mut color32 = original;
            skip_grid_column(ui);
            ui.push_id(format!("palette_color_{}", i), |ui| {
                ui.horizontal(|ui| {
                    ui.color_edit_button_srgba(&mut color32);
                    if ui.button("X").clicked() && colors_len > 1 {
                        to_remove = Some(i);
                    }
                });
            });
            ui.end_row();

            if color32 != original {
                *color = Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
            }
        }

        if let Some(remove_index) = to_remove {
            palette.colors.remove(remove_index);
        }
    }
}

fn show_effect_overlays(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    liquid_effect: Option<&LiquidEffect>,
    gas_effect: Option<&GasEffect>,
    glow_effect: Option<&GlowEffect>,
    burn_effect: Option<&BurnEffect>,
) {
    let liquid_enabled = liquid_effect.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Liquid", liquid_enabled);
    if new_value != liquid_enabled {
        if new_value {
            commands.entity(entity).insert(LiquidEffect);
        } else {
            commands.entity(entity).remove::<LiquidEffect>();
        }
    }

    let gas_enabled = gas_effect.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Gas", gas_enabled);
    if new_value != gas_enabled {
        if new_value {
            commands.entity(entity).insert(GasEffect);
        } else {
            commands.entity(entity).remove::<GasEffect>();
        }
    }

    let glow_enabled = glow_effect.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Glow", glow_enabled);
    if new_value != glow_enabled {
        if new_value {
            commands.entity(entity).insert(GlowEffect);
        } else {
            commands.entity(entity).remove::<GlowEffect>();
        }
    }

    let burn_enabled = burn_effect.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Burn", burn_enabled);
    if new_value != burn_enabled {
        if new_value {
            commands.entity(entity).insert(BurnEffect);
        } else {
            commands.entity(entity).remove::<BurnEffect>();
        }
    }
}

fn show_flammability(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    burns: Option<Mut<'_, Flammable>>,
    burns_state: &mut Flammable,
) {
    let enabled = burns.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Flammable", enabled);
    if new_value != enabled {
        if new_value {
            commands.entity(entity).insert(burns_state.clone());
        } else {
            commands.entity(entity).remove::<Flammable>();
        }
    }

    if let Some(mut burns) = burns {
        show_burns_timing(ui, &mut burns, burns_state);
        show_burns_ignites_on_spawn(ui, &mut burns, burns_state);
        show_burns_reaction(ui, &mut burns, burns_state);

        add_minor_grid_separator(ui);

        show_burns_spreads(ui, &mut burns, burns_state);
    }
}

fn show_burns_timing(
    ui: &mut egui::Ui,
    burns: &mut Mut<'_, Flammable>,
    burns_state: &mut Flammable,
) {
    let duration_ms = burns.duration.as_millis() as u64;
    let new_value =
        add_label_with_drag_value(ui, 0, "    Duration (ms):", duration_ms, 0..=u64::MAX, 1.0);
    if new_value != duration_ms {
        burns.duration = Duration::from_millis(new_value);
        burns_state.duration = Duration::from_millis(new_value);
    }

    let tick_rate_ms = burns.tick_rate.as_millis() as u64;
    let new_value = add_label_with_drag_value(
        ui,
        0,
        "    Tick Rate (ms):",
        tick_rate_ms,
        0..=u64::MAX,
        1.0,
    );
    if new_value != tick_rate_ms {
        burns.tick_rate = Duration::from_millis(new_value);
        burns_state.tick_rate = Duration::from_millis(new_value);
    }

    let chance_despawn_per_tick = burns.chance_despawn_per_tick;
    let new_value = add_label_with_drag_value(
        ui,
        0,
        "    Despawn chance (per tick)",
        chance_despawn_per_tick,
        0.0..=1.,
        0.01,
    );
    if new_value != chance_despawn_per_tick {
        burns.chance_despawn_per_tick = new_value;
        burns_state.chance_despawn_per_tick = new_value;
    }
}

fn show_burns_ignites_on_spawn(
    ui: &mut egui::Ui,
    burns: &mut Mut<'_, Flammable>,
    burns_state: &mut Flammable,
) {
    let ignites = burns.ignites_on_spawn;
    let new_value = add_label_with_toggle_switch(ui, 0, "    Ignites on spawn", ignites);
    if new_value != ignites {
        burns.ignites_on_spawn = new_value;
        burns_state.ignites_on_spawn = new_value;
    }
}

fn show_burns_reaction(
    ui: &mut egui::Ui,
    burns: &mut Mut<'_, Flammable>,
    burns_state: &mut Flammable,
) {
    let reaction_enabled = burns.reaction.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "    Reaction", reaction_enabled);
    if new_value != reaction_enabled {
        if new_value {
            burns.reaction = Some(burns_state.reaction.clone().unwrap_or_default());
        } else {
            burns.reaction = None;
        }
        burns_state.reaction = burns.reaction.clone();
    }
    if let Some(ref mut reaction) = burns.reaction {
        let chance = reaction.chance_to_produce;
        let new_value =
            add_label_with_drag_value(ui, 0, "        Chance to produce", chance, 0.0..=1.0, 0.01);
        if (new_value - chance).abs() > f64::EPSILON {
            reaction.chance_to_produce = new_value;
            if let Some(ref mut state_reaction) = burns_state.reaction {
                state_reaction.chance_to_produce = new_value;
            }
        }

        let mut produces_name = reaction.produces.name.to_string();
        ui.label("        Produces");
        if ui
            .add(egui::TextEdit::singleline(&mut produces_name))
            .changed()
        {
            reaction.produces = Particle::from(produces_name.clone());
            if let Some(ref mut state_reaction) = burns_state.reaction {
                state_reaction.produces = Particle::from(produces_name);
            }
        }
        ui.end_row();
    }
}

fn show_burns_spreads(
    ui: &mut egui::Ui,
    burns: &mut Mut<'_, Flammable>,
    burns_state: &mut Flammable,
) {
    let spreads_enabled = burns.spreads_fire;
    let new_value = add_label_with_toggle_switch(ui, 0, "    Spreads fire", spreads_enabled);
    if new_value != spreads_enabled {
        burns.spreads_fire = new_value;
        burns_state.spreads_fire = new_value;
    }

    if burns.spreads_fire {
        let radius = burns.spread_radius;
        let new_value =
            add_label_with_drag_value(ui, 0, "        Spread radius", radius, 0.0..=10.0, 0.1);
        if (new_value - radius).abs() > f32::EPSILON {
            burns.spread_radius = new_value;
            burns_state.spread_radius = new_value;
        }
    }
}

fn show_contact_reactions(
    commands: &mut Commands,
    entity: Entity,
    ui: &mut egui::Ui,
    contact_reaction: Option<Mut<'_, ContactReaction>>,
    contact_reaction_state: &mut ContactReaction,
) {
    let enabled = contact_reaction.is_some();
    let new_value = add_label_with_toggle_switch(ui, 0, "Enabled", enabled);
    if new_value != enabled {
        if new_value {
            commands
                .entity(entity)
                .insert(contact_reaction_state.clone());
        } else {
            commands.entity(entity).remove::<ContactReaction>();
        }
    }

    if let Some(mut contact_reaction) = contact_reaction {
        show_contact_rules(ui, &mut contact_reaction, contact_reaction_state);
    }
}

fn show_contact_rules(
    ui: &mut egui::Ui,
    contact_reaction: &mut Mut<'_, ContactReaction>,
    contact_reaction_state: &mut ContactReaction,
) {
    ui.label("    Rules");
    if ui.button("Add Rule").clicked() {
        let rule = ContactRule {
            target: Particle::default(),
            becomes: Particle::default(),
            chance: 0.1,
            radius: 1.0,
            consumes: Consumes::Source,
        };
        contact_reaction.rules.push(rule.clone());
        contact_reaction_state.rules.push(rule);
    }
    ui.end_row();

    let mut to_remove: Option<usize> = None;
    for (i, rule) in contact_reaction.rules.iter_mut().enumerate() {
        if i > 0 {
            add_minor_grid_separator(ui);
        }

        ui.label(format!("    Rule {}", i + 1));
        ui.push_id(format!("contact_rule_remove_{i}"), |ui| {
            if ui.button("X").clicked() {
                to_remove = Some(i);
            }
        });
        ui.end_row();

        let mut target_name = rule.target.name.to_string();
        ui.label("        Target");
        let changed = ui
            .push_id(format!("contact_rule_target_{i}"), |ui| {
                ui.add(egui::TextEdit::singleline(&mut target_name))
                    .changed()
            })
            .inner;
        ui.end_row();
        if changed {
            rule.target = Particle::from(target_name.clone());
            contact_reaction_state.rules[i].target = Particle::from(target_name);
        }

        let mut becomes_name = rule.becomes.name.to_string();
        ui.label("        Becomes");
        let changed = ui
            .push_id(format!("contact_rule_becomes_{i}"), |ui| {
                ui.add(egui::TextEdit::singleline(&mut becomes_name))
                    .changed()
            })
            .inner;
        ui.end_row();
        if changed {
            rule.becomes = Particle::from(becomes_name.clone());
            contact_reaction_state.rules[i].becomes = Particle::from(becomes_name);
        }

        let chance = rule.chance;
        ui.label("        Chance");
        let new_chance = ui
            .push_id(format!("contact_rule_chance_{i}"), |ui| {
                let mut value = chance;
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::DragValue::new(&mut value)
                            .range(0.0..=1.0)
                            .speed(0.01),
                    );
                });
                value
            })
            .inner;
        ui.end_row();
        if (new_chance - chance).abs() > f64::EPSILON {
            rule.chance = new_chance;
            contact_reaction_state.rules[i].chance = new_chance;
        }

        let radius = rule.radius;
        ui.label("        Radius");
        let new_radius = ui
            .push_id(format!("contact_rule_radius_{i}"), |ui| {
                let mut value = radius;
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.add(
                        egui::DragValue::new(&mut value)
                            .range(0.0..=10.0)
                            .speed(0.1),
                    );
                });
                value
            })
            .inner;
        ui.end_row();
        if (new_radius - radius).abs() > f32::EPSILON {
            rule.radius = new_radius;
            contact_reaction_state.rules[i].radius = new_radius;
        }

        let consumes = rule.consumes;
        ui.label("        Consumes");
        let new_consumes = ui
            .push_id(format!("contact_rule_consumes_{i}"), |ui| {
                let mut value = consumes;
                egui::ComboBox::from_id_salt(format!("consumes_combo_{i}"))
                    .selected_text(match value {
                        Consumes::Source => "Source",
                        Consumes::Target => "Target",
                    })
                    .show_ui(ui, |ui| {
                        ui.selectable_value(&mut value, Consumes::Source, "Source");
                        ui.selectable_value(&mut value, Consumes::Target, "Target");
                    });
                value
            })
            .inner;
        ui.end_row();
        if new_consumes != consumes {
            rule.consumes = new_consumes;
            contact_reaction_state.rules[i].consumes = new_consumes;
        }
    }

    if let Some(idx) = to_remove {
        contact_reaction.rules.remove(idx);
        contact_reaction_state.rules.remove(idx);
    }
}

enum NeighborAction {
    Swap(usize, usize),
    Remove(usize),
}

fn direction_label(v: IVec2) -> &'static str {
    match (v.x.signum(), v.y.signum()) {
        (-1, 1) => "NW",
        (0, 1) => "N",
        (1, 1) => "NE",
        (-1, 0) => "W",
        (1, 0) => "E",
        (-1, -1) => "SW",
        (0, -1) => "S",
        (1, -1) => "SE",
        _ => "?",
    }
}

fn direction_arrow(v: IVec2) -> &'static str {
    match (v.x, v.y) {
        (-1, 1) => "NW",
        (0, 1) => "N",
        (1, 1) => "NE",
        (-1, 0) => "W",
        (1, 0) => "E",
        (-1, -1) => "SW",
        (0, -1) => "S",
        (1, -1) => "SE",
        _ => "?",
    }
}

fn show_neighbor_groups(
    ui: &mut egui::Ui,
    movement: &mut Mut<'_, Movement>,
    air_resistance: &mut Mut<'_, AirResistance>,
) {
    ui.label("Neighbor Groups");
    if ui.button("Add Tier").clicked() {
        movement.push_outer(NeighborGroup::empty());
    }
    ui.end_row();

    // Render all tiers in a single grid row to avoid polluting the outer grid layout
    skip_grid_column(ui);
    let mut tier_to_remove: Option<usize> = None;
    ui.vertical(|ui| {
        for tier_idx in 0..movement.neighbor_groups.len() {
            ui.push_id(format!("tier_{tier_idx}"), |ui| {
                egui::CollapsingHeader::new(format!("Tier {tier_idx}"))
                    .default_open(false)
                    .show(ui, |ui| {
                        if ui.small_button("Remove Tier").clicked() {
                            tier_to_remove = Some(tier_idx);
                        }
                        ui.horizontal(|ui| {
                            ui.label("Air Resistance");
                            let current = air_resistance
                                .get(tier_idx)
                                .expect("No air resistance found for tier {tier_idx}");
                            let mut drag_value = current;
                            ui.add(
                                egui::DragValue::new(&mut drag_value)
                                    .range(0.0..=1.0)
                                    .speed(1.0),
                            );
                            if drag_value != current {
                                air_resistance.set(tier_idx, drag_value);
                            }
                        });

                        // Pending direction state: [dir_x, dir_y, magnitude]
                        let pending_id = egui::Id::new(format!("pending_{tier_idx}"));
                        let mut pending: [i32; 3] =
                            ui.data_mut(|d| *d.get_temp_mut_or(pending_id, [0, 0, 0]));

                        // 3x3 directional arrow grid
                        egui::Grid::new(format!("arrow_grid_{tier_idx}"))
                            .spacing(egui::vec2(2.0, 2.0))
                            .show(ui, |ui| {
                                for row in 0..3i32 {
                                    for col in 0..3i32 {
                                        if row == 1 && col == 1 {
                                            ui.label("  ");
                                        } else {
                                            let dir = IVec2::new(col - 1, 1 - row);
                                            let is_pending_dir = pending[2] > 0
                                                && pending[0] == dir.x
                                                && pending[1] == dir.y;
                                            let arrow = direction_arrow(dir);
                                            let btn = if is_pending_dir {
                                                egui::Button::new(arrow)
                                                    .fill(egui::Color32::from_rgb(70, 130, 180))
                                            } else {
                                                egui::Button::new(arrow)
                                            };
                                            if ui.add(btn).clicked() {
                                                if is_pending_dir {
                                                    // Same direction: increment
                                                    pending[2] += 1;
                                                } else {
                                                    // New direction: reset
                                                    pending = [dir.x, dir.y, 1];
                                                }
                                            }
                                        }
                                    }
                                    ui.end_row();
                                }
                            });

                        // Pending neighbor display + confirm/decrement
                        if pending[2] > 0 {
                            let pending_dir = IVec2::new(pending[0], pending[1]);
                            let pending_offset = pending_dir * pending[2];
                            let already_present = movement.neighbor_groups[tier_idx]
                                .neighbor_group
                                .contains(&pending_offset);
                            ui.horizontal(|ui| {
                                ui.label(format!(
                                    "Pending: {} ({}, {})",
                                    direction_label(pending_offset),
                                    pending_offset.x,
                                    pending_offset.y,
                                ));
                                if ui
                                    .add_enabled(pending[2] > 1, egui::Button::new("-").small())
                                    .clicked()
                                {
                                    pending[2] -= 1;
                                }
                                if ui
                                    .add_enabled(!already_present, egui::Button::new("Add").small())
                                    .clicked()
                                {
                                    movement.neighbor_groups[tier_idx].push(pending_offset);
                                    pending = [0, 0, 0];
                                }
                                if ui.small_button("Cancel").clicked() {
                                    pending = [0, 0, 0];
                                }
                            });
                        }

                        ui.data_mut(|d| *d.get_temp_mut_or(pending_id, [0, 0, 0]) = pending);

                        // Manual X/Y entry
                        let mx_id = egui::Id::new(format!("manual_x_{tier_idx}"));
                        let my_id = egui::Id::new(format!("manual_y_{tier_idx}"));
                        let mut mx: i32 = ui.data_mut(|d| *d.get_temp_mut_or(mx_id, 0));
                        let mut my: i32 = ui.data_mut(|d| *d.get_temp_mut_or(my_id, 0));
                        ui.horizontal(|ui| {
                            ui.label("X:");
                            ui.add(egui::DragValue::new(&mut mx).speed(0.1));
                            ui.label("Y:");
                            ui.add(egui::DragValue::new(&mut my).speed(0.1));
                            let manual_offset = IVec2::new(mx, my);
                            let already = movement.neighbor_groups[tier_idx]
                                .neighbor_group
                                .contains(&manual_offset);
                            if ui.add_enabled(!already, egui::Button::new("Add")).clicked() {
                                movement.neighbor_groups[tier_idx].push(manual_offset);
                            }
                        });
                        ui.data_mut(|d| *d.get_temp_mut_or(mx_id, 0) = mx);
                        ui.data_mut(|d| *d.get_temp_mut_or(my_id, 0) = my);

                        // Current neighbor list
                        let group_len = movement.neighbor_groups[tier_idx].neighbor_group.len();
                        let mut action: Option<NeighborAction> = None;

                        for j in 0..group_len {
                            let v = movement.neighbor_groups[tier_idx].neighbor_group[j];
                            ui.horizontal(|ui| {
                                if ui
                                    .add_enabled(j > 0, egui::Button::new("Up").small())
                                    .clicked()
                                {
                                    action = Some(NeighborAction::Swap(j, j - 1));
                                }
                                if ui
                                    .add_enabled(j < group_len - 1, egui::Button::new("Dn").small())
                                    .clicked()
                                {
                                    action = Some(NeighborAction::Swap(j, j + 1));
                                }
                                ui.label(format!("{} ({}, {})", direction_label(v), v.x, v.y));
                                if ui.small_button("X").clicked() {
                                    action = Some(NeighborAction::Remove(j));
                                }
                            });
                        }

                        if let Some(action) = action {
                            match action {
                                NeighborAction::Swap(a, b) => {
                                    let _ = movement.neighbor_groups[tier_idx].swap(a, b);
                                }
                                NeighborAction::Remove(idx) => {
                                    movement.neighbor_groups[tier_idx]
                                        .neighbor_group
                                        .remove(idx);
                                }
                            }
                        }
                    });
            });
        }
    });
    ui.end_row();

    if let Some(idx) = tier_to_remove {
        movement.remove(idx);
    }
}
