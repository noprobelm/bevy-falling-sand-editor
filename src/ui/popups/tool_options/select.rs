use bevy::reflect::Enum;
use bevy_egui::egui;

use crate::tools::select::{SelectOptions, states::SelectModeState};

pub fn show_select_settings(ui: &mut egui::Ui, mut select_options: SelectOptions) {
    egui::Grid::new("select_grid")
        .num_columns(2)
        .show(ui, |ui| {
            ui.label("Mode");
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                egui::ComboBox::from_id_salt("select_mode_combo")
                    .selected_text(select_options.current_mode_state.get().variant_name())
                    .show_ui(ui, |ui| {
                        if ui
                            .selectable_label(
                                matches!(
                                    select_options.current_mode_state.get(),
                                    SelectModeState::Throw
                                ),
                                "Throw",
                            )
                            .clicked()
                        {
                            select_options.next_mode_state.set(SelectModeState::Throw);
                        } else if ui
                            .selectable_label(
                                matches!(
                                    select_options.current_mode_state.get(),
                                    SelectModeState::Drag
                                ),
                                "Drag",
                            )
                            .clicked()
                        {
                            select_options.next_mode_state.set(SelectModeState::Drag);
                        }
                    });
            });
        });
}
