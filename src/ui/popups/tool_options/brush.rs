use bevy::{ecs::system::SystemParam, prelude::*, reflect::Enum};
use bevy_egui::egui;

use crate::{
    tools::brush::{BrushSize, BrushSpawnState, BrushTypeState},
    ui::add_label_with_drag_value,
};

#[derive(SystemParam)]
pub struct BrushSettingsParam<'w, 's> {
    pub size: Single<'w, 's, &'static mut crate::brush::BrushSize>,
    pub current_type_state: Res<'w, State<BrushTypeState>>,
    pub next_type_state: ResMut<'w, NextState<BrushTypeState>>,
    pub current_mode_state: Res<'w, State<BrushSpawnState>>,
    pub next_mode_state: ResMut<'w, NextState<BrushSpawnState>>,
}

pub fn show_brush_settings(ui: &mut egui::Ui, mut brush_settings: BrushSettingsParam) {
    egui::Grid::new("brush_grid").num_columns(2).show(ui, |ui| {
        show_brush_size(ui, &mut brush_settings);
        show_brush_type_selection(ui, &mut brush_settings);
        show_brush_mode_selection(ui, &mut brush_settings);
    });
}

pub fn show_brush_size(ui: &mut egui::Ui, brush_settings: &mut BrushSettingsParam) {
    let new_value = add_label_with_drag_value(ui, 0, "Size", brush_settings.size.0, 0..=50, 1.0);
    brush_settings.size.set_if_neq(BrushSize(new_value));
}

pub fn show_brush_type_selection(ui: &mut egui::Ui, brush_settings: &mut BrushSettingsParam) {
    ui.label("Type");
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
    });
    ui.end_row();
}

pub fn show_brush_mode_selection(ui: &mut egui::Ui, brush_settings: &mut BrushSettingsParam) {
    ui.label("Mode");
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
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
    });
    ui.end_row();
}
