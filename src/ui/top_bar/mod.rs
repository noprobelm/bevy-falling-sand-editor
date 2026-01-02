pub mod particle_files;

use super::*;
use crate::scenes::{spawn_load_scene_dialog, spawn_save_scene_dialog};
use crate::ui::file_browser::FileBrowserState;
use crate::ui::settings::SettingsState;
use particle_files::{
    spawn_load_scene_dialog as spawn_load_particles_scene_dialog,
    spawn_save_scene_dialog as spawn_save_particles_scene_dialog,
};

pub use particle_files::ParticleFilesPlugin;

pub(super) struct UiTopBar;

impl UiTopBar {
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        commands: &mut Commands,
        particle_browser_state: &mut ResMut<FileBrowserState>,
        settings_state: &mut ResMut<SettingsState>,
        hovered_particle: Res<HoveredParticle>,
        cursor_position: Res<CursorPosition>,
    ) {
        ui.menu_button(egui::RichText::new("File").size(16.0), |ui| {
            if ui.button("Save Scene").clicked() {
                spawn_save_scene_dialog(commands);
                egui::Ui::close(ui);
            }
            if ui.button("Load Scene").clicked() {
                spawn_load_scene_dialog(commands);
                egui::Ui::close(ui);
            }
            ui.separator();
            if ui.button("Save Particle Set").clicked() {
                spawn_save_particles_scene_dialog(particle_browser_state);
                egui::Ui::close(ui);
            }
            if ui.button("Load Particle Set").clicked() {
                spawn_load_particles_scene_dialog(particle_browser_state);
                egui::Ui::close(ui);
            }
            ui.separator();
            if ui.button("Settings").clicked() {
                settings_state.open();
                egui::Ui::close(ui);
            }
            if ui.button("Documentation").clicked() {
                egui::Ui::close(ui);
            }
            ui.separator();
            if ui.button("Exit").clicked() {
                egui::Ui::close(ui);
            }
        });

        let label = hovered_particle
            .particle
            .as_ref()
            .map(|p| p.name.clone())
            .unwrap_or_default();

        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
            ui.colored_label(
                egui::Color32::WHITE,
                format!(
                    "X: {:.2}    Y: {:.2}    -    {}",
                    cursor_position.current.x, cursor_position.current.y, label
                ),
            );
        });
    }
}
