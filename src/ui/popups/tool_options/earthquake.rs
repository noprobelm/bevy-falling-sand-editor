use bevy::prelude::*;
use bevy_egui::egui;

use crate::tools::earthquake::{EarthquakeOptions, debug::DebugEarthquake};

const OPTION_GAP: f32 = 40.0;

pub fn show_earthquake_options(
    ui: &mut egui::Ui,
    mut commands: Commands,
    earthquake_options: EarthquakeOptions,
) {
    egui::Grid::new("brush_grid")
        .num_columns(2)
        .spacing(egui::vec2(OPTION_GAP, ui.spacing().item_spacing.y))
        .show(ui, |ui| {
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
