use bevy::{prelude::*, reflect::Enum};
use bevy_egui::egui;

use crate::tools::{
    brush::{ToolBrushColor, ToolBrushSize},
    painter::{PainterOptions, PainterShape, PainterSpawnState},
};

const OPTION_GAP: f32 = 40.0;

pub fn show_painter_options(ui: &mut egui::Ui, mut brush_settings: PainterOptions) {
    egui::Grid::new("brush_grid")
        .num_columns(2)
        .spacing(egui::vec2(OPTION_GAP, ui.spacing().item_spacing.y))
        .show(ui, |ui| {
            show_brush_size(ui, &mut brush_settings);
            show_brush_color(ui, &mut brush_settings);
            show_brush_type_selection(ui, &mut brush_settings);
            show_brush_mode_selection(ui, &mut brush_settings);
        });
}

fn show_brush_size(ui: &mut egui::Ui, brush_settings: &mut PainterOptions) {
    ui.label("Size");
    let mut new_value = brush_settings.size.0;
    ui.add(egui::Slider::new(&mut new_value, 1.0..=50.0).step_by(1.0));
    ui.end_row();

    brush_settings.size.set_if_neq(ToolBrushSize(new_value));
}

fn show_brush_color(ui: &mut egui::Ui, brush_settings: &mut PainterOptions) {
    let srgba = brush_settings.configuration.brush.color.to_srgba();
    let original = egui::Color32::from_rgba_unmultiplied(
        (srgba.red * 255.0) as u8,
        (srgba.green * 255.0) as u8,
        (srgba.blue * 255.0) as u8,
        (srgba.alpha * 255.0) as u8,
    );
    let mut color32 = original;

    ui.label("Color");
    ui.color_edit_button_srgba(&mut color32);
    ui.end_row();

    if color32 != original {
        brush_settings.configuration.brush.color =
            Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
        brush_settings
            .color
            .set_if_neq(ToolBrushColor(brush_settings.configuration.brush.color));
    }
}

fn show_brush_type_selection(ui: &mut egui::Ui, brush_settings: &mut PainterOptions) {
    ui.label("Type");
    egui::ComboBox::from_id_salt("brush_type_combo")
        .selected_text(brush_settings.current_type_state.get().variant_name())
        .show_ui(ui, |ui| {
            if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_type_state.get(),
                        PainterShape::Circle
                    ),
                    "Circle",
                )
                .clicked()
            {
                brush_settings.next_type_state.set(PainterShape::Circle)
            } else if ui
                .selectable_label(
                    matches!(brush_settings.current_type_state.get(), PainterShape::Line),
                    "Line",
                )
                .clicked()
            {
                brush_settings.next_type_state.set(PainterShape::Line)
            } else if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_type_state.get(),
                        PainterShape::Cursor
                    ),
                    "Cursor",
                )
                .clicked()
            {
                brush_settings.next_type_state.set(PainterShape::Cursor)
            };
        });
    ui.end_row();
}

fn show_brush_mode_selection(ui: &mut egui::Ui, brush_settings: &mut PainterOptions) {
    ui.label("Mode");
    egui::ComboBox::from_id_salt("brush_mode_combo")
        .selected_text(brush_settings.current_mode_state.get().variant_name())
        .show_ui(ui, |ui| {
            if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_mode_state.get(),
                        PainterSpawnState::Spawn
                    ),
                    "Spawn",
                )
                .clicked()
            {
                brush_settings.next_mode_state.set(PainterSpawnState::Spawn)
            } else if ui
                .selectable_label(
                    matches!(
                        brush_settings.current_mode_state.get(),
                        PainterSpawnState::Despawn
                    ),
                    "Despawn",
                )
                .clicked()
            {
                brush_settings
                    .next_mode_state
                    .set(PainterSpawnState::Despawn)
            };
        });
    ui.end_row();
}
