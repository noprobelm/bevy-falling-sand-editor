use bevy::{prelude::*, reflect::Enum};
use bevy_egui::egui;

use crate::tools::earthquake::{
    DebugEarthquake, EarthquakeBrushSize, EarthquakeFractureShapeState, EarthquakeOptions,
    EarthquakeRegionState,
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
            show_earthquake_size(ui, &mut earthquake_options);
            show_earthquake_region_selection(ui, &mut earthquake_options);
            show_earthquake_fracture_shape_selection(ui, &mut earthquake_options);
            let mut debug_enabled = earthquake_options.debug.is_some();
            if ui.checkbox(&mut debug_enabled, "Debug").clicked() {
                if debug_enabled {
                    commands.insert_resource(DebugEarthquake);
                } else {
                    commands.remove_resource::<DebugEarthquake>();
                }
            }
        });
}

fn show_earthquake_size(ui: &mut egui::Ui, earthquake_options: &mut EarthquakeOptions) {
    ui.label("Size");
    let mut new_value = earthquake_options.size.0;
    ui.add(
        egui::DragValue::new(&mut new_value)
            .range(1.0..=256.0)
            .speed(1.0),
    );
    ui.end_row();

    earthquake_options
        .size
        .set_if_neq(EarthquakeBrushSize(new_value));
}

fn show_earthquake_region_selection(ui: &mut egui::Ui, earthquake_options: &mut EarthquakeOptions) {
    ui.label("Region");
    egui::ComboBox::from_id_salt("earthquake_region_combo")
        .selected_text(earthquake_options.current_region_state.get().variant_name())
        .show_ui(ui, |ui| {
            for region in [
                EarthquakeRegionState::Circle,
                EarthquakeRegionState::Rect,
                EarthquakeRegionState::Polygon,
            ] {
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
                EarthquakeFractureShapeState::SimplifiedConvexHulls,
                EarthquakeFractureShapeState::ExactVoronoiCells,
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
