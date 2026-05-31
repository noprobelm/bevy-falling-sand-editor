use bevy::{prelude::*, reflect::Enum};
use bevy_egui::egui;

use crate::tools::brush::{BrushOptions, BrushSize, BrushSpawnState, BrushTypeState};

const BRUSH_OPTION_GAP: f32 = 40.0;

pub fn show_brush_settings(ui: &mut egui::Ui, mut brush_settings: BrushOptions) {
    egui::Grid::new("brush_grid")
        .num_columns(2)
        .spacing(egui::vec2(BRUSH_OPTION_GAP, ui.spacing().item_spacing.y))
        .show(ui, |ui| {
            show_brush_size(ui, &mut brush_settings);
            show_brush_type_selection(ui, &mut brush_settings);
            show_brush_mode_selection(ui, &mut brush_settings);
        });
}

fn show_brush_size(ui: &mut egui::Ui, brush_settings: &mut BrushOptions) {
    ui.label("Size");
    let mut new_value = brush_settings.size.0;
    ui.add(
        egui::DragValue::new(&mut new_value)
            .range(0..=50)
            .speed(1.0),
    );
    ui.end_row();

    brush_settings.size.set_if_neq(BrushSize(new_value));
}

fn show_brush_type_selection(ui: &mut egui::Ui, brush_settings: &mut BrushOptions) {
    ui.label("Type");
    egui::ComboBox::from_id_salt("brush_type_combo")
        .selected_text(brush_settings.current_type_state.get().variant_name())
        .show_ui(ui, |ui| {
            if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_type_state.get(),
                        BrushTypeState::Circle
                    ),
                    "Circle",
                )
                .clicked()
            {
                brush_settings.next_type_state.set(BrushTypeState::Circle)
            } else if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_type_state.get(),
                        BrushTypeState::Line
                    ),
                    "Line",
                )
                .clicked()
            {
                brush_settings.next_type_state.set(BrushTypeState::Line)
            } else if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_type_state.get(),
                        BrushTypeState::Cursor
                    ),
                    "Cursor",
                )
                .clicked()
            {
                brush_settings.next_type_state.set(BrushTypeState::Cursor)
            };
        });
    ui.end_row();
}

fn show_brush_mode_selection(ui: &mut egui::Ui, brush_settings: &mut BrushOptions) {
    ui.label("Mode");
    egui::ComboBox::from_id_salt("brush_mode_combo")
        .selected_text(brush_settings.current_mode_state.get().variant_name())
        .show_ui(ui, |ui| {
            if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_mode_state.get(),
                        BrushSpawnState::Spawn
                    ),
                    "Spawn",
                )
                .clicked()
            {
                brush_settings.next_mode_state.set(BrushSpawnState::Spawn)
            } else if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_mode_state.get(),
                        BrushSpawnState::Despawn
                    ),
                    "Despawn",
                )
                .clicked()
            {
                brush_settings.next_mode_state.set(BrushSpawnState::Despawn)
            };
        });
    ui.end_row();
}
