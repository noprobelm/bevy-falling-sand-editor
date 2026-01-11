mod particle_editor_registry;

use bevy::prelude::*;
use bevy_egui::egui;
use bevy_falling_sand::color::{ColorAssignment, ColorGradient, ColorSource, Palette};
use bevy_falling_sand::prelude::{
    ParticleTypeMaterialsParam, ParticleTypeRegistry, ResetParticleTypeChildrenSignal,
};

use particle_editor_registry::*;

pub use particle_editor_registry::{
    ApplyEditorChangesAndReset, BurnsConfig, ChangesColorConfig, CreateNewParticle,
    CurrentEditorSelection, FireConfig, LifetimeConfig, LoadParticleIntoEditor, MaterialState,
    ParticleEditorData,
};

pub struct ParticleEditorPlugin;

impl Plugin for ParticleEditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleEditorRegistry>()
            .init_resource::<CurrentEditorSelection>()
            .add_message::<LoadParticleIntoEditor>()
            .add_message::<CreateNewParticle>()
            .add_message::<ApplyEditorChangesAndReset>()
            .add_systems(
                OnEnter(crate::app_state::InitializationState::Finished),
                setup_initial_particle_selection,
            )
            .add_systems(
                Update,
                (
                    sync_particle_editor_registry,
                    handle_load_particle_into_editor,
                    handle_create_new_particle,
                    auto_save_editor_changes,
                    handle_apply_editor_changes_and_reset,
                ),
            );
    }
}
pub struct ParticleEditor;

impl ParticleEditor {
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        particle_materials: &ParticleTypeMaterialsParam,
        current_editor: &CurrentEditorSelection,
        editor_data_query: &mut Query<&mut ParticleEditorData>,
        load_particle_messages: &mut MessageWriter<LoadParticleIntoEditor>,
        create_particle_messages: &mut MessageWriter<CreateNewParticle>,
        apply_editor_and_reset_messages: &mut MessageWriter<ApplyEditorChangesAndReset>,
        reset_particle_children_messages: &mut MessageWriter<ResetParticleTypeChildrenSignal>,
        particle_type_map: &ParticleTypeRegistry,
    ) {
        let text_color = egui::Color32::from_rgb(204, 204, 204);
        ui.visuals_mut().override_text_color = Some(text_color);

        let current_particle_name = if let Some(editor_entity) = current_editor.selected_entity {
            if let Ok(editor_data) = editor_data_query.get(editor_entity) {
                Some(editor_data.name.clone())
            } else {
                None
            }
        } else {
            None
        };

        ui.columns(2, |columns| {
            self.render_particle_list(
                &mut columns[0],
                particle_materials,
                load_particle_messages,
                create_particle_messages,
                current_editor,
                apply_editor_and_reset_messages,
                reset_particle_children_messages,
                particle_type_map,
                current_particle_name,
            );

            self.render_particle_properties(&mut columns[1], current_editor, editor_data_query);
        });
    }

    fn render_particle_list(
        &self,
        ui: &mut egui::Ui,
        particle_materials: &ParticleTypeMaterialsParam,
        load_particle_messages: &mut MessageWriter<LoadParticleIntoEditor>,
        create_particle_messages: &mut MessageWriter<CreateNewParticle>,
        current_editor: &CurrentEditorSelection,
        apply_editor_and_reset_messages: &mut MessageWriter<ApplyEditorChangesAndReset>,
        _reset_particle_children_messages: &mut MessageWriter<ResetParticleTypeChildrenSignal>,
        _particle_type_map: &ParticleTypeRegistry,
        current_particle_name: Option<String>,
    ) {
        egui::ScrollArea::vertical()
            .id_salt("particle_list_scroll")
            .show(ui, |ui| {
                const CATEGORIES: [&str; 7] = [
                    "Walls",
                    "Solids",
                    "Movable Solids",
                    "Liquids",
                    "Gases",
                    "Insects",
                    "Other",
                ];

                for &category in &CATEGORIES {
                    let original_indent = ui.spacing().indent;
                    ui.spacing_mut().indent = 16.0;

                    egui::CollapsingHeader::new(category)
                        .default_open(false)
                        .show(ui, |ui| {
                            ui.spacing_mut().indent = original_indent;
                            let examples: Vec<&str> = match category {
                                "Walls" => particle_materials
                                    .walls()
                                    .iter()
                                    .map(|particle_type| particle_type.name.as_ref())
                                    .collect(),
                                "Solids" => particle_materials
                                    .solids()
                                    .iter()
                                    .map(|particle_type| particle_type.name.as_ref())
                                    .collect(),
                                "Movable Solids" => particle_materials
                                    .movable_solids()
                                    .iter()
                                    .map(|particle_type| particle_type.name.as_ref())
                                    .collect(),
                                "Liquids" => particle_materials
                                    .liquids()
                                    .iter()
                                    .map(|particle_type| particle_type.name.as_ref())
                                    .collect(),
                                "Gases" => particle_materials
                                    .gases()
                                    .iter()
                                    .map(|particle_type| particle_type.name.as_ref())
                                    .collect(),
                                "Insects" => particle_materials
                                    .insects()
                                    .iter()
                                    .map(|particle_type| particle_type.name.as_ref())
                                    .collect(),
                                "Other" => particle_materials
                                    .other()
                                    .iter()
                                    .map(|particle_type| particle_type.name.as_ref())
                                    .collect(),
                                _ => vec![],
                            };

                            for particle_name in examples {
                                if ui.button(particle_name).clicked() {
                                    load_particle_messages.write(LoadParticleIntoEditor {
                                        particle_name: particle_name.to_string(),
                                    });
                                }
                            }
                        });

                    ui.spacing_mut().indent = original_indent;
                }

                ui.add_space(8.0);
                ui.vertical(|ui| {
                    if ui.button("New Particle").clicked() {
                        create_particle_messages.write(CreateNewParticle {
                            duplicate_from: current_particle_name.clone(),
                        });
                    }
                    if ui.button("Reset Children").clicked() {
                        if let Some(editor_entity) = current_editor.selected_entity {
                            apply_editor_and_reset_messages
                                .write(ApplyEditorChangesAndReset { editor_entity });
                        }
                    }
                });
            });
    }

    fn render_particle_properties(
        &self,
        ui: &mut egui::Ui,
        current_editor: &CurrentEditorSelection,
        editor_data_query: &mut Query<&mut ParticleEditorData>,
    ) {
        egui::ScrollArea::vertical()
            .id_salt("particle_properties_scroll")
            .show(ui, |ui| {
                if let Some(editor_entity) = current_editor.selected_entity {
                    if let Ok(mut editor_data) = editor_data_query.get_mut(editor_entity) {
                        self.render_editor_data(ui, &mut editor_data);
                    } else {
                        ui.label("Selected editor entity not found.");
                    }
                } else {
                    ui.label("No particle selected for editing.");
                    ui.label("Select a particle from the list on the left, or create a new one.");
                }
            });
    }

    fn render_editor_data(&self, ui: &mut egui::Ui, editor_data: &mut ParticleEditorData) {
        ui.horizontal(|ui| {
            ui.label("Name:");
            ui.text_edit_singleline(&mut editor_data.name);
        });

        ui.horizontal(|ui| {
            ui.label("State:");
            let previous_state = editor_data.material_state.clone();
            let current_state_text = match editor_data.material_state {
                MaterialState::Wall => "Wall",
                MaterialState::Solid => "Solid",
                MaterialState::MovableSolid => "Movable Solid",
                MaterialState::Liquid => "Liquid",
                MaterialState::Gas => "Gas",
                MaterialState::Insect => "Insect",
                MaterialState::Other => "Other",
            };

            egui::ComboBox::from_id_salt("material_state_combo")
                .selected_text(current_state_text)
                .show_ui(ui, |ui| {
                    if ui
                        .selectable_value(
                            &mut editor_data.material_state,
                            MaterialState::Wall,
                            "Wall",
                        )
                        .clicked()
                    {
                        editor_data.fluidity = None;
                        if previous_state == MaterialState::Gas {
                            editor_data.has_momentum = true;
                        }
                    }
                    if ui
                        .selectable_value(
                            &mut editor_data.material_state,
                            MaterialState::Solid,
                            "Solid",
                        )
                        .clicked()
                    {
                        editor_data.fluidity = None;
                        if previous_state == MaterialState::Gas {
                            editor_data.has_momentum = true;
                        }
                    }
                    if ui
                        .selectable_value(
                            &mut editor_data.material_state,
                            MaterialState::MovableSolid,
                            "Movable Solid",
                        )
                        .clicked()
                    {
                        editor_data.fluidity = None;
                        if previous_state == MaterialState::Gas {
                            editor_data.has_momentum = true;
                        }
                    }
                    if ui
                        .selectable_value(
                            &mut editor_data.material_state,
                            MaterialState::Liquid,
                            "Liquid",
                        )
                        .clicked()
                    {
                        if editor_data.fluidity.is_none() {
                            editor_data.fluidity = Some(3);
                        }
                        if editor_data.liquid_resistance.is_none() {
                            editor_data.liquid_resistance = Some(0.0);
                        }
                        if previous_state == MaterialState::Gas {
                            editor_data.has_momentum = true;
                        }
                    }
                    if ui
                        .selectable_value(
                            &mut editor_data.material_state,
                            MaterialState::Gas,
                            "Gas",
                        )
                        .clicked()
                    {
                        if editor_data.fluidity.is_none() {
                            editor_data.fluidity = Some(3);
                        }
                        editor_data.has_momentum = false;
                    }
                    if ui
                        .selectable_value(
                            &mut editor_data.material_state,
                            MaterialState::Insect,
                            "Insect",
                        )
                        .clicked()
                    {
                        editor_data.has_momentum = false;
                    }

                    if ui
                        .selectable_value(
                            &mut editor_data.material_state,
                            MaterialState::Other,
                            "Other",
                        )
                        .clicked()
                    {
                        editor_data.fluidity = None;
                        if previous_state == MaterialState::Gas {
                            editor_data.has_momentum = true;
                        }
                    }
                });
        });

        ui.separator();

        let label_width = 110.0;

        egui::Grid::new("particle_properties_grid")
            .num_columns(2)
            .min_col_width(label_width)
            .show(ui, |ui| {
                // Density
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Density:");
                });
                ui.horizontal(|ui| {
                    let drag_width = 50.0;
                    ui.add_space(ui.available_width() - drag_width);
                    ui.add_sized(
                        [drag_width, 18.0],
                        egui::DragValue::new(&mut editor_data.density).range(1..=u32::MAX),
                    );
                });
                ui.end_row();

                // Max Speed
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Max Speed:");
                });
                ui.horizontal(|ui| {
                    let mut speed_f32 = editor_data.max_speed as f32;
                    if ui
                        .add(
                            egui::Slider::new(&mut speed_f32, 1.0..=5.0)
                                .step_by(1.0)
                                .show_value(false),
                        )
                        .changed()
                    {
                        editor_data.max_speed = speed_f32 as u8;
                    }
                    let drag_width = 50.0;
                    ui.add_space(ui.available_width() - drag_width);
                    let mut speed_drag = editor_data.max_speed as u32;
                    if ui
                        .add_sized(
                            [drag_width, 18.0],
                            egui::DragValue::new(&mut speed_drag).range(1..=5),
                        )
                        .changed()
                    {
                        editor_data.max_speed = speed_drag as u8;
                    }
                });
                ui.end_row();

                // Fluidity (for Liquid/Gas)
                if matches!(
                    editor_data.material_state,
                    MaterialState::Liquid | MaterialState::Gas
                ) {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label("Fluidity:");
                    });
                    ui.horizontal(|ui| {
                        let fluidity_value = editor_data.fluidity.unwrap_or(3);
                        let mut fluidity_f32 = fluidity_value as f32;
                        if ui
                            .add(
                                egui::Slider::new(&mut fluidity_f32, 1.0..=5.0)
                                    .step_by(1.0)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            editor_data.fluidity = Some(fluidity_f32 as u8);
                        }
                        let drag_width = 50.0;
                        ui.add_space(ui.available_width() - drag_width);
                        let mut fluidity_drag = editor_data.fluidity.unwrap_or(3) as u32;
                        if ui
                            .add_sized(
                                [drag_width, 18.0],
                                egui::DragValue::new(&mut fluidity_drag).range(1..=5),
                            )
                            .changed()
                        {
                            editor_data.fluidity = Some(fluidity_drag as u8);
                        }
                    });
                    ui.end_row();
                }

                // Liquid Resistance (for MovableSolid and Liquid)
                if matches!(
                    editor_data.material_state,
                    MaterialState::MovableSolid | MaterialState::Liquid
                ) {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label("Liquid Resistance:");
                    });
                    ui.horizontal(|ui| {
                        let mut liquid_resistance = editor_data.liquid_resistance.unwrap_or(0.5);
                        if ui
                            .add(
                                egui::Slider::new(&mut liquid_resistance, 0.0..=1.0)
                                    .show_value(false),
                            )
                            .changed()
                        {
                            editor_data.liquid_resistance = Some(liquid_resistance);
                        }
                        let drag_width = 50.0;
                        ui.add_space(ui.available_width() - drag_width);
                        let mut resistance_drag = editor_data.liquid_resistance.unwrap_or(0.5);
                        if ui
                            .add_sized(
                                [drag_width, 18.0],
                                egui::DragValue::new(&mut resistance_drag)
                                    .range(0.0..=1.0)
                                    .speed(0.01),
                            )
                            .changed()
                        {
                            editor_data.liquid_resistance = Some(resistance_drag);
                        }
                    });
                    ui.end_row();
                }

                // Air Resistance (for MovableSolid only)
                if matches!(editor_data.material_state, MaterialState::MovableSolid) {
                    ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                        ui.label("Air Resistance:");
                    });
                    ui.horizontal(|ui| {
                        let mut air_resistance = editor_data.air_resistance.unwrap_or(0.5);
                        if ui
                            .add(
                                egui::Slider::new(&mut air_resistance, 0.0..=1.0).show_value(false),
                            )
                            .changed()
                        {
                            editor_data.air_resistance = Some(air_resistance);
                        }
                        let drag_width = 50.0;
                        ui.add_space(ui.available_width() - drag_width);
                        let mut resistance_drag = editor_data.air_resistance.unwrap_or(0.5);
                        if ui
                            .add_sized(
                                [drag_width, 18.0],
                                egui::DragValue::new(&mut resistance_drag)
                                    .range(0.0..=1.0)
                                    .speed(0.01),
                            )
                            .changed()
                        {
                            editor_data.air_resistance = Some(resistance_drag);
                        }
                    });
                    ui.end_row();
                }

                // Momentum
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Momentum:");
                });
                ui.horizontal(|ui| {
                    let checkbox_width = 20.0;
                    ui.add_space(ui.available_width() - checkbox_width);
                    ui.checkbox(&mut editor_data.has_momentum, "");
                });
                ui.end_row();
            });

        // Lifetime section
        ui.horizontal(|ui| {
            ui.label("Lifetime:");
            let checkbox_width = 20.0;
            ui.add_space(ui.available_width() - checkbox_width);
            let mut has_lifetime = editor_data.lifetime.is_some();
            if ui.checkbox(&mut has_lifetime, "").changed() {
                if has_lifetime {
                    editor_data.lifetime = Some(LifetimeConfig::Timed {
                        duration_ms: 1000,
                        duration_str: "1000".to_string(),
                    });
                } else {
                    editor_data.lifetime = None;
                }
            }
        });

        if let Some(ref mut config) = editor_data.lifetime {
            ui.horizontal(|ui| {
                ui.add_space(20.0); // indent
                ui.label("Type:");

                let mut is_timed = matches!(config, LifetimeConfig::Timed { .. });
                if ui.radio_value(&mut is_timed, true, "Timed").changed() {
                    *config = LifetimeConfig::Timed {
                        duration_ms: 1000,
                        duration_str: "1000".to_string(),
                    };
                }

                let mut is_chance = matches!(config, LifetimeConfig::Chance { .. });
                if ui.radio_value(&mut is_chance, true, "Chance").changed() {
                    *config = LifetimeConfig::Chance {
                        chance: 0.1,
                        tick_rate_ms: 100,
                        tick_rate_str: "100".to_string(),
                    };
                }
            });

            match config {
                LifetimeConfig::Timed {
                    duration_ms,
                    duration_str,
                } => {
                    ui.horizontal(|ui| {
                        ui.add_space(20.0); // indent
                        ui.label("Duration (ms):");
                        if ui
                            .add_sized([80.0, 18.0], egui::TextEdit::singleline(duration_str))
                            .changed()
                        {
                            if let Ok(ms) = duration_str.parse::<u64>() {
                                *duration_ms = ms;
                            }
                        }
                    });
                }
                LifetimeConfig::Chance {
                    chance,
                    tick_rate_ms,
                    tick_rate_str,
                } => {
                    ui.horizontal(|ui| {
                        ui.add_space(20.0); // indent
                        ui.label("Chance:");
                        ui.add(egui::Slider::new(chance, 0.0..=1.0).show_value(false));
                        ui.add_sized(
                            [50.0, 18.0],
                            egui::DragValue::new(chance).range(0.0..=1.0).speed(0.01),
                        );
                    });
                    ui.horizontal(|ui| {
                        ui.add_space(20.0); // indent
                        ui.label("Tick Rate (ms):");
                        if ui
                            .add_sized([80.0, 18.0], egui::TextEdit::singleline(tick_rate_str))
                            .changed()
                        {
                            if let Ok(ms) = tick_rate_str.parse::<u64>() {
                                *tick_rate_ms = ms;
                            }
                        }
                    });
                }
            }
        }

        // Color properties grid
        egui::Grid::new("color_properties_grid")
            .num_columns(2)
            .min_col_width(label_width)
            .show(ui, |ui| {
                // Color Source
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Color Source:");
                });
                ui.horizontal(|ui| {
                    let combo_width = 100.0;
                    ui.add_space(ui.available_width() - combo_width);
                    let current_source = match &editor_data.color_source {
                        ColorSource::Palette(_) => "Palette",
                        ColorSource::Gradient(_) => "Gradient",
                    };
                    egui::ComboBox::from_id_salt("color_source_combo")
                        .width(combo_width - 8.0)
                        .selected_text(current_source)
                        .show_ui(ui, |ui| {
                            if ui
                                .selectable_label(
                                    matches!(editor_data.color_source, ColorSource::Palette(_)),
                                    "Palette",
                                )
                                .clicked()
                            {
                                if !matches!(editor_data.color_source, ColorSource::Palette(_)) {
                                    editor_data.color_source = ColorSource::Palette(Palette {
                                        index: 0,
                                        colors: vec![Color::srgba_u8(255, 255, 255, 255)],
                                    });
                                }
                            }
                            if ui
                                .selectable_label(
                                    matches!(editor_data.color_source, ColorSource::Gradient(_)),
                                    "Gradient",
                                )
                                .clicked()
                            {
                                if !matches!(editor_data.color_source, ColorSource::Gradient(_)) {
                                    editor_data.color_source =
                                        ColorSource::Gradient(ColorGradient {
                                            start: Color::srgb(1.0, 0.0, 0.0),
                                            end: Color::srgb(0.0, 0.0, 1.0),
                                            index: 0,
                                            steps: 50,
                                            hsv_interpolation: false,
                                        });
                                }
                            }
                        });
                });
                ui.end_row();

                // Color Assignment
                ui.with_layout(egui::Layout::left_to_right(egui::Align::Center), |ui| {
                    ui.label("Assignment:");
                });
                ui.horizontal(|ui| {
                    let combo_width = 100.0;
                    ui.add_space(ui.available_width() - combo_width);
                    let current_assignment_text = match editor_data.color_assignment {
                        ColorAssignment::Sequential => "Sequential",
                        ColorAssignment::Random => "Random",
                    };
                    egui::ComboBox::from_id_salt("color_assignment_combo")
                        .width(combo_width - 8.0)
                        .selected_text(current_assignment_text)
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut editor_data.color_assignment,
                                ColorAssignment::Sequential,
                                "Sequential",
                            );
                            ui.selectable_value(
                                &mut editor_data.color_assignment,
                                ColorAssignment::Random,
                                "Random",
                            );
                        });
                });
                ui.end_row();
            });

        // Render appropriate editor based on color source
        match &mut editor_data.color_source {
            ColorSource::Palette(palette) => {
                ui.horizontal(|ui| {
                    ui.label("Palette Colors");
                    let button_width = 80.0;
                    ui.add_space(ui.available_width() - button_width);
                    if ui.button("➕ Add Color").clicked() {
                        let new_color = palette
                            .colors
                            .last()
                            .copied()
                            .unwrap_or(Color::srgba_u8(255, 255, 255, 255));
                        palette.colors.push(new_color);
                    }
                });

                let mut to_remove: Option<usize> = None;
                let colors_len = palette.colors.len();
                for (i, color) in palette.colors.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        let srgba = color.to_srgba();
                        let value_size = [24.0, 18.0];
                        ui.label("R:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.red * 255.0)),
                        );
                        ui.label("G:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.green * 255.0)),
                        );
                        ui.label("B:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.blue * 255.0)),
                        );
                        ui.label("A:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.alpha * 255.0)),
                        );

                        let mut color32 = egui::Color32::from_rgba_unmultiplied(
                            (srgba.red * 255.0) as u8,
                            (srgba.green * 255.0) as u8,
                            (srgba.blue * 255.0) as u8,
                            (srgba.alpha * 255.0) as u8,
                        );

                        ui.push_id(format!("palette_color_{}", i), |ui| {
                            if ui.color_edit_button_srgba(&mut color32).changed() {
                                *color = Color::srgba_u8(
                                    color32.r(),
                                    color32.g(),
                                    color32.b(),
                                    color32.a(),
                                );
                            }
                        });

                        if ui.button(format!("❌ {}", i)).clicked() && colors_len > 1 {
                            to_remove = Some(i);
                        }
                    });
                }

                if let Some(remove_index) = to_remove {
                    palette.colors.remove(remove_index);
                }
            }
            ColorSource::Gradient(gradient) => {
                ui.label("Gradient Settings");

                // Start Color
                ui.horizontal(|ui| {
                    ui.label("Start Color:");
                    let button_width = 50.0;
                    ui.add_space(ui.available_width() - button_width);
                    let srgba = gradient.start.to_srgba();
                    let mut color32 = egui::Color32::from_rgba_unmultiplied(
                        (srgba.red * 255.0) as u8,
                        (srgba.green * 255.0) as u8,
                        (srgba.blue * 255.0) as u8,
                        (srgba.alpha * 255.0) as u8,
                    );
                    if ui.color_edit_button_srgba(&mut color32).changed() {
                        gradient.start =
                            Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
                    }
                });

                // End Color
                ui.horizontal(|ui| {
                    ui.label("End Color:");
                    let button_width = 50.0;
                    ui.add_space(ui.available_width() - button_width);
                    let srgba = gradient.end.to_srgba();
                    let mut color32 = egui::Color32::from_rgba_unmultiplied(
                        (srgba.red * 255.0) as u8,
                        (srgba.green * 255.0) as u8,
                        (srgba.blue * 255.0) as u8,
                        (srgba.alpha * 255.0) as u8,
                    );
                    if ui.color_edit_button_srgba(&mut color32).changed() {
                        gradient.end =
                            Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
                    }
                });

                // Steps
                ui.horizontal(|ui| {
                    ui.label("Steps:");
                    let mut steps_f32 = gradient.steps as f32;
                    let slider_width = ui.available_width();
                    if ui
                        .add_sized(
                            [slider_width, 18.0],
                            egui::Slider::new(&mut steps_f32, 2.0..=10000.0)
                                .step_by(1.0)
                                .show_value(false),
                        )
                        .changed()
                    {
                        gradient.steps = steps_f32 as usize;
                    }
                    let drag_width = 50.0;
                    ui.add_space(ui.available_width() - drag_width);
                    let mut steps_drag = gradient.steps as u32;
                    if ui
                        .add_sized(
                            [drag_width, 18.0],
                            egui::DragValue::new(&mut steps_drag).range(2..=10000),
                        )
                        .changed()
                    {
                        gradient.steps = steps_drag as usize;
                    }
                });
            }
        }

        // Color changes section
        ui.horizontal(|ui| {
            ui.label("Changes Color:");
            let checkbox_width = 20.0;
            ui.add_space(ui.available_width() - checkbox_width);
            let mut has_changes_color = editor_data.changes_color.is_some();
            if ui.checkbox(&mut has_changes_color, "").changed() {
                if has_changes_color {
                    editor_data.changes_color = Some(ChangesColorConfig::Chance(0.1));
                } else {
                    editor_data.changes_color = None;
                }
            }
        });

        if let Some(ref mut config) = editor_data.changes_color {
            ui.horizontal(|ui| {
                ui.add_space(20.0); // indent
                ui.label("Type:");

                let mut is_chance = matches!(config, ChangesColorConfig::Chance(_));
                if ui.radio_value(&mut is_chance, true, "Chance").changed() {
                    *config = ChangesColorConfig::Chance(0.1);
                }

                let mut is_timed = matches!(config, ChangesColorConfig::Timed { .. });
                if ui.radio_value(&mut is_timed, true, "Timed").changed() {
                    *config = ChangesColorConfig::Timed {
                        duration_ms: 1000,
                        duration_str: "1000".to_string(),
                    };
                }
            });

            match config {
                ChangesColorConfig::Chance(chance) => {
                    ui.horizontal(|ui| {
                        ui.add_space(20.0); // indent
                        ui.label("Chance:");
                        ui.add(egui::Slider::new(chance, 0.0..=1.0).show_value(false));
                        ui.add_sized(
                            [50.0, 18.0],
                            egui::DragValue::new(chance).range(0.0..=1.0).speed(0.01),
                        );
                    });
                }
                ChangesColorConfig::Timed {
                    duration_ms,
                    duration_str,
                } => {
                    ui.horizontal(|ui| {
                        ui.add_space(20.0); // indent
                        ui.label("Duration (ms):");
                        if ui
                            .add_sized([80.0, 18.0], egui::TextEdit::singleline(duration_str))
                            .changed()
                        {
                            if let Ok(ms) = duration_str.parse::<u64>() {
                                *duration_ms = ms;
                            }
                        }
                    });
                }
            }
        }

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Flammable:");
            let checkbox_width = 20.0;
            ui.add_space(ui.available_width() - checkbox_width);
            let mut has_burns = editor_data.burns_config.is_some();
            if ui.checkbox(&mut has_burns, "").changed() {
                if has_burns {
                    editor_data.burns_config = Some(BurnsConfig {
                        duration: std::time::Duration::from_millis(1000),
                        tick_rate: std::time::Duration::from_millis(100),
                        duration_str: "1000".to_string(),
                        tick_rate_str: "100".to_string(),
                        chance_destroy_per_tick: None,
                        reaction: None,
                        burning_colors: None,
                        spreads_fire: None,
                        ignites_on_spawn: false,
                    });
                } else {
                    editor_data.burns_config = None;
                }
            }
        });

        if let Some(ref mut burns_config) = editor_data.burns_config {
            ui.horizontal(|ui| {
                ui.label("Duration (ms):");
                let text_width = 80.0;
                ui.add_space(ui.available_width() - text_width);
                if ui
                    .add_sized(
                        [text_width, 18.0],
                        egui::TextEdit::singleline(&mut burns_config.duration_str),
                    )
                    .changed()
                {
                    if let Ok(new_duration) = burns_config.duration_str.parse::<u64>() {
                        burns_config.duration = std::time::Duration::from_millis(new_duration);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Tick Rate (ms):");
                let text_width = 80.0;
                ui.add_space(ui.available_width() - text_width);
                if ui
                    .add_sized(
                        [text_width, 18.0],
                        egui::TextEdit::singleline(&mut burns_config.tick_rate_str),
                    )
                    .changed()
                {
                    if let Ok(new_tick_rate) = burns_config.tick_rate_str.parse::<u64>() {
                        burns_config.tick_rate = std::time::Duration::from_millis(new_tick_rate);
                    }
                }
            });

            ui.horizontal(|ui| {
                ui.label("Destroy Chance:");
                let checkbox_width = 20.0;
                ui.add_space(ui.available_width() - checkbox_width);
                let mut has_destroy_chance = burns_config.chance_destroy_per_tick.is_some();
                if ui.checkbox(&mut has_destroy_chance, "").changed() {
                    if has_destroy_chance {
                        burns_config.chance_destroy_per_tick = Some(0.1);
                    } else {
                        burns_config.chance_destroy_per_tick = None;
                    }
                }
            });

            if let Some(ref mut chance) = burns_config.chance_destroy_per_tick {
                ui.horizontal(|ui| {
                    ui.add_space(20.0); // indent
                    ui.add(egui::Slider::new(chance, 0.0..=1.0).show_value(false));
                    let drag_width = 50.0;
                    ui.add_space(ui.available_width() - drag_width);
                    ui.add_sized(
                        [drag_width, 18.0],
                        egui::DragValue::new(chance).range(0.0..=1.0).speed(0.01),
                    );
                });
            }

            ui.horizontal(|ui| {
                ui.label("Produces particle:");
                let checkbox_width = 20.0;
                ui.add_space(ui.available_width() - checkbox_width);
                let mut has_reaction = burns_config.reaction.is_some();
                if ui.checkbox(&mut has_reaction, "").changed() {
                    if has_reaction {
                        burns_config.reaction = Some(ReactionConfig {
                            produces: "Smoke".to_string(),
                            chance_to_produce: 0.1,
                        });
                    } else {
                        burns_config.reaction = None;
                    }
                }
            });

            if let Some(ref mut reaction) = burns_config.reaction {
                ui.horizontal(|ui| {
                    ui.label("Produces:");
                    let text_width = 80.0;
                    ui.add_space(ui.available_width() - text_width);
                    ui.add_sized(
                        [text_width, 18.0],
                        egui::TextEdit::singleline(&mut reaction.produces),
                    );
                });

                ui.label("(Spawn positions are automatically determined by the produced particle's movement pattern)");

                ui.horizontal(|ui| {
                    ui.label("Chance:");
                    let slider_width = ui.available_width();
                    if ui
                        .add_sized(
                            [slider_width, 18.0],
                            egui::Slider::new(&mut reaction.chance_to_produce, 0.0..=1.0)
                                .show_value(false),
                        )
                        .changed()
                    {}
                    let drag_width = 50.0;
                    ui.add_space(ui.available_width() - drag_width);
                    ui.add_sized(
                        [drag_width, 18.0],
                        egui::DragValue::new(&mut reaction.chance_to_produce)
                            .range(0.0..=1.0)
                            .speed(0.01),
                    );
                });
            }

            ui.horizontal(|ui| {
                ui.label("Custom colors:");
                let checkbox_width = 20.0;
                ui.add_space(ui.available_width() - checkbox_width);
                let mut has_burning_colors = burns_config.burning_colors.is_some();
                if ui.checkbox(&mut has_burning_colors, "").changed() {
                    if has_burning_colors {
                        burns_config.burning_colors = Some(vec![
                            Color::srgba_u8(255, 89, 0, 255),
                            Color::srgba_u8(255, 153, 0, 255),
                            Color::srgba_u8(255, 207, 0, 255),
                        ]);
                    } else {
                        burns_config.burning_colors = None;
                    }
                }
            });

            if let Some(ref mut burning_colors) = burns_config.burning_colors {
                ui.horizontal(|ui| {
                    ui.label("Burning Colors:");
                    let button_width = 80.0;
                    ui.add_space(ui.available_width() - button_width);
                    if ui.button("➕ Add Color").clicked() {
                        let new_color = burning_colors
                            .last()
                            .copied()
                            .unwrap_or(Color::srgba_u8(255, 128, 0, 255));
                        burning_colors.push(new_color);
                    }
                });

                let mut to_remove = None;
                let colors_len = burning_colors.len();
                for (i, color) in burning_colors.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        let srgba = color.to_srgba();
                        let value_size = [24.0, 18.0];
                        ui.label("R:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.red * 255.0)),
                        );
                        ui.label("G:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.green * 255.0)),
                        );
                        ui.label("B:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.blue * 255.0)),
                        );
                        ui.label("A:");
                        ui.add_sized(
                            value_size,
                            egui::Label::new(format!("{:.0}", srgba.alpha * 255.0)),
                        );

                        let mut color32 = egui::Color32::from_rgba_unmultiplied(
                            (srgba.red * 255.0) as u8,
                            (srgba.green * 255.0) as u8,
                            (srgba.blue * 255.0) as u8,
                            (srgba.alpha * 255.0) as u8,
                        );

                        ui.push_id(format!("burning_color_{}", i), |ui| {
                            if ui.color_edit_button_srgba(&mut color32).changed() {
                                *color = Color::srgba_u8(
                                    color32.r(),
                                    color32.g(),
                                    color32.b(),
                                    color32.a(),
                                );
                            }
                        });

                        if ui.button(format!("❌ {}", i)).clicked() && colors_len > 1 {
                            to_remove = Some(i);
                        }
                    });
                }

                if let Some(remove_index) = to_remove {
                    burning_colors.remove(remove_index);
                }
            }

            ui.horizontal(|ui| {
                ui.label("Fire Spreads:");
                let checkbox_width = 20.0;
                ui.add_space(ui.available_width() - checkbox_width);
                let mut has_fire_spreads = burns_config.spreads_fire.is_some();
                if ui.checkbox(&mut has_fire_spreads, "").changed() {
                    if has_fire_spreads {
                        burns_config.spreads_fire = Some(FireConfig {
                            burn_radius: 2.0,
                            chance_to_spread: 0.01,
                            destroys_on_spread: false,
                        });
                    } else {
                        burns_config.spreads_fire = None;
                    }
                }
            });

            if let Some(ref mut fire_config) = burns_config.spreads_fire {
                ui.horizontal(|ui| {
                    ui.label("Burn Radius:");
                    let slider_width = ui.available_width();
                    if ui
                        .add_sized(
                            [slider_width, 18.0],
                            egui::Slider::new(&mut fire_config.burn_radius, 1.0..=100.0)
                                .show_value(false),
                        )
                        .changed()
                    {}
                    let drag_width = 50.0;
                    ui.add_space(ui.available_width() - drag_width);
                    ui.add_sized(
                        [drag_width, 18.0],
                        egui::DragValue::new(&mut fire_config.burn_radius)
                            .range(1.0..=100.0)
                            .speed(0.1),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Chance to spread:");
                    let slider_width = ui.available_width();
                    if ui
                        .add_sized(
                            [slider_width, 18.0],
                            egui::Slider::new(&mut fire_config.chance_to_spread, 0.0..=1.0)
                                .show_value(false),
                        )
                        .changed()
                    {}
                    let drag_width = 50.0;
                    ui.add_space(ui.available_width() - drag_width);
                    ui.add_sized(
                        [drag_width, 18.0],
                        egui::DragValue::new(&mut fire_config.chance_to_spread)
                            .range(0.0..=1.0)
                            .speed(0.01),
                    );
                });

                ui.horizontal(|ui| {
                    ui.label("Destroys on spread:");
                    let checkbox_width = 20.0;
                    ui.add_space(ui.available_width() - checkbox_width);
                    ui.checkbox(&mut fire_config.destroys_on_spread, "");
                });
            }

            ui.horizontal(|ui| {
                ui.label("Ignites on spawn:");
                let checkbox_width = 20.0;
                ui.add_space(ui.available_width() - checkbox_width);
                ui.checkbox(&mut burns_config.ignites_on_spawn, "");
            });
        }

        ui.separator();

        ui.horizontal(|ui| {
            ui.label("Emits Fire:");
            let checkbox_width = 20.0;
            ui.add_space(ui.available_width() - checkbox_width);
            let mut has_fire = editor_data.fire_config.is_some();
            if ui.checkbox(&mut has_fire, "").changed() {
                if has_fire {
                    editor_data.fire_config = Some(FireConfig {
                        burn_radius: 2.0,
                        chance_to_spread: 0.1,
                        destroys_on_spread: false,
                    });
                } else {
                    editor_data.fire_config = None;
                }
            }
        });

        if let Some(ref mut fire_config) = editor_data.fire_config {
            ui.horizontal(|ui| {
                ui.label("Burn Radius:");
                let slider_width = ui.available_width();
                if ui
                    .add_sized(
                        [slider_width, 18.0],
                        egui::Slider::new(&mut fire_config.burn_radius, 1.0..=100.0)
                            .show_value(false),
                    )
                    .changed()
                {}
                let drag_width = 50.0;
                ui.add_space(ui.available_width() - drag_width);
                ui.add_sized(
                    [drag_width, 18.0],
                    egui::DragValue::new(&mut fire_config.burn_radius)
                        .range(1.0..=100.0)
                        .speed(0.1),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Chance to spread:");
                let slider_width = ui.available_width();
                if ui
                    .add_sized(
                        [slider_width, 18.0],
                        egui::Slider::new(&mut fire_config.chance_to_spread, 0.0..=1.0)
                            .show_value(false),
                    )
                    .changed()
                {}
                let drag_width = 50.0;
                ui.add_space(ui.available_width() - drag_width);
                ui.add_sized(
                    [drag_width, 18.0],
                    egui::DragValue::new(&mut fire_config.chance_to_spread)
                        .range(0.0..=1.0)
                        .speed(0.01),
                );
            });

            ui.horizontal(|ui| {
                ui.label("Destroys on spread:");
                let checkbox_width = 20.0;
                ui.add_space(ui.available_width() - checkbox_width);
                ui.checkbox(&mut fire_config.destroys_on_spread, "");
            });
        }
    }
}

