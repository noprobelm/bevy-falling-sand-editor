use bevy::{prelude::*, reflect::Enum};
use bevy_egui::egui;

use crate::tools::{
    brush::{ToolBrushColor, ToolBrushSize},
    earthquake::{DebugEarthquake, EarthquakeFractureShape, EarthquakeOptions, EarthquakeShape},
};

const OPTION_GAP: f32 = 40.0;

pub fn show_earthquake_options(
    ui: &mut egui::Ui,
    mut commands: Commands,
    mut earthquake_options: EarthquakeOptions,
) {
    egui::Grid::new("earthquake_grid")
        .num_columns(2)
        .spacing(egui::vec2(OPTION_GAP, ui.spacing().item_spacing.y))
        .show(ui, |ui| {
            show_earthquake_configuration(ui, &mut commands, &mut earthquake_options);
        });
}

fn show_brush_size(ui: &mut egui::Ui, earthquake_options: &mut EarthquakeOptions) {
    ui.label("Size");
    let mut new_value = earthquake_options.size.0;
    ui.add(egui::Slider::new(&mut new_value, 1.0..=256.0).step_by(1.0));
    ui.end_row();

    earthquake_options.size.set_if_neq(ToolBrushSize(new_value));
}

fn show_earthquake_debug_toggle(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    earthquake_options: &EarthquakeOptions,
) {
    let debug_enabled = earthquake_options.debug.is_some();
    let mut new_debug_enabled = debug_enabled;
    ui.label("Debug");
    ui.add(crate::ui::widgets::toggle_switch::toggle(
        &mut new_debug_enabled,
    ));
    ui.end_row();

    if new_debug_enabled != debug_enabled {
        if new_debug_enabled {
            commands.insert_resource(DebugEarthquake);
        } else {
            commands.remove_resource::<DebugEarthquake>();
        }
    }
}

fn show_earthquake_region_selection(ui: &mut egui::Ui, earthquake_options: &mut EarthquakeOptions) {
    ui.label("Shape");
    egui::ComboBox::from_id_salt("brush_shape_combo")
        .selected_text(earthquake_options.current_region_state.get().variant_name())
        .show_ui(ui, |ui| {
            for region in [EarthquakeShape::Circle, EarthquakeShape::Rect] {
                if ui
                    .selectable_label(
                        *earthquake_options.current_region_state.get() == region,
                        region.variant_name(),
                    )
                    .clicked()
                {
                    earthquake_options.next_region_state.set(region);
                }
            }
        });
    ui.end_row();
}

fn show_earthquake_configuration(
    ui: &mut egui::Ui,
    commands: &mut Commands,
    earthquake_options: &mut EarthquakeOptions,
) {
    ui.heading("Brush");
    ui.end_row();
    show_brush_color(ui, earthquake_options);
    show_brush_size(ui, earthquake_options);
    show_configuration_f32(
        ui,
        "Resize Step",
        &mut earthquake_options.configuration.brush.resize_step,
        0.1..=64.0,
        0.1,
    );
    show_earthquake_region_selection(ui, earthquake_options);

    ui.heading("Voronoi");
    ui.end_row();
    show_configuration_f32(
        ui,
        "Cells / Area",
        &mut earthquake_options.configuration.voronoi_cells_per_area,
        0.0..=1.0,
        0.001,
    );
    show_configuration_usize(
        ui,
        "Min Sites",
        &mut earthquake_options.configuration.voronoi_min_sites,
        1..=4096,
    );
    show_configuration_usize(
        ui,
        "Max Sites",
        &mut earthquake_options.configuration.voronoi_max_sites,
        1..=4096,
    );

    ui.heading("Fractures");
    ui.end_row();
    show_earthquake_fracture_shape_selection(ui, earthquake_options);
    show_configuration_usize(
        ui,
        "Min Body Cells",
        &mut earthquake_options.configuration.min_fracture_body_cells,
        1..=4096,
    );
    show_configuration_f32(
        ui,
        "Render Z",
        &mut earthquake_options.configuration.rigid_body_render_z,
        -100.0..=100.0,
        0.1,
    );
    show_configuration_f32(
        ui,
        "Collision Margin",
        &mut earthquake_options.configuration.collision_margin,
        0.0..=10.0,
        0.01,
    );

    ui.heading("Debug");
    ui.end_row();
    show_earthquake_debug_toggle(ui, commands, earthquake_options);
    show_configuration_f32(
        ui,
        "Gizmo Duration",
        &mut earthquake_options.configuration.debug_gizmo_duration_secs,
        0.0..=60.0,
        0.1,
    );
    show_configuration_color(
        ui,
        "Region Color",
        &mut earthquake_options.configuration.debug_region_color,
    );
    show_configuration_color(
        ui,
        "Fracture Color",
        &mut earthquake_options.configuration.debug_fracture_color,
    );
}

fn show_brush_color(ui: &mut egui::Ui, earthquake_options: &mut EarthquakeOptions) {
    show_configuration_color(
        ui,
        "Color",
        &mut earthquake_options.configuration.brush.color,
    );
    earthquake_options
        .color
        .set_if_neq(ToolBrushColor(earthquake_options.configuration.brush.color));
}

fn show_configuration_f32(
    ui: &mut egui::Ui,
    label: &'static str,
    value: &mut f32,
    range: std::ops::RangeInclusive<f32>,
    step: f64,
) {
    ui.label(label);
    ui.add(egui::Slider::new(value, range).step_by(step));
    ui.end_row();
}

fn show_configuration_usize(
    ui: &mut egui::Ui,
    label: &'static str,
    value: &mut usize,
    range: std::ops::RangeInclusive<usize>,
) {
    ui.label(label);
    ui.add(egui::Slider::new(value, range).step_by(1.0));
    ui.end_row();
}

fn show_configuration_color(ui: &mut egui::Ui, label: &'static str, color: &mut Color) {
    let srgba = color.to_srgba();
    let original = egui::Color32::from_rgba_unmultiplied(
        (srgba.red * 255.0) as u8,
        (srgba.green * 255.0) as u8,
        (srgba.blue * 255.0) as u8,
        (srgba.alpha * 255.0) as u8,
    );
    let mut color32 = original;

    ui.label(label);
    ui.color_edit_button_srgba(&mut color32);
    ui.end_row();

    if color32 != original {
        *color = Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
    }
}

fn show_earthquake_fracture_shape_selection(
    ui: &mut egui::Ui,
    earthquake_options: &mut EarthquakeOptions,
) {
    ui.label("Fractures");
    egui::ComboBox::from_id_salt("earthquake_fracture_shape_combo")
        .selected_text(
            earthquake_options
                .current_fracture_shape_state
                .get()
                .variant_name(),
        )
        .show_ui(ui, |ui| {
            for fracture_shape in [
                EarthquakeFractureShape::Convex,
                EarthquakeFractureShape::Concave,
            ] {
                if ui
                    .selectable_label(
                        *earthquake_options.current_fracture_shape_state.get() == fracture_shape,
                        fracture_shape.variant_name(),
                    )
                    .clicked()
                {
                    earthquake_options
                        .next_fracture_shape_state
                        .set(fracture_shape);
                }
            }
        });
    ui.end_row();
}
