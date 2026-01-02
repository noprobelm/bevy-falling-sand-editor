use bevy::prelude::*;
use bevy_egui::egui;
use bevy_falling_sand::prelude::{
    ChunkMovementMode, MovementSource, UpdateChunkMovementModeSignal, UpdateMovementSourceSignal,
};

#[derive(Resource, Default)]
pub struct StatisticsPanel;

impl StatisticsPanel {
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        particle_movement_state_current: &MovementSource,
        chunk_movement_mode: &ChunkMovementMode,
        fps: f32,
        dynamic_particles: u32,
        wall_particles: u32,
        total_particles: u32,
        active_particles: u32,
        num_rigid_bodies: u32,
        commands: &mut Commands,
    ) {
        let text_color = egui::Color32::from_rgb(204, 204, 204);
        ui.visuals_mut().override_text_color = Some(text_color);

        ui.add(egui::Label::new(
            egui::RichText::new("Performance").heading().size(16.0),
        ));
        ui.separator();
        egui::Grid::new("performance_grid")
            .num_columns(2)
            .spacing([20.0, 8.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("FPS:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", fps.round() as i32));
                });
                ui.end_row();
            });

        ui.add_space(8.0);

        ui.add(egui::Label::new(
            egui::RichText::new("Particles").heading().size(16.0),
        ));
        ui.separator();
        egui::Grid::new("particles_grid")
            .num_columns(2)
            .spacing([20.0, 8.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Total:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", total_particles));
                });
                ui.end_row();

                ui.label("Wall:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", wall_particles));
                });
                ui.end_row();

                ui.label("Dynamic:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", dynamic_particles));
                });
                ui.end_row();

                ui.label("Active:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", active_particles));
                });
                ui.end_row();
            });

        ui.add_space(8.0);

        ui.add(egui::Label::new(
            egui::RichText::new("Avian 2D").heading().size(16.0),
        ));
        ui.separator();
        egui::Grid::new("avian_grid")
            .num_columns(2)
            .spacing([20.0, 8.0])
            .striped(false)
            .show(ui, |ui| {
                ui.label("Dynamic Rigid Bodies:");
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(format!("{}", num_rigid_bodies));
                });
                ui.end_row();
            });

        ui.add_space(8.0);

        ui.add(egui::Label::new(
            egui::RichText::new("Movement Configuration")
                .heading()
                .size(16.0),
        ));
        ui.separator();

        let new_movement_mode = *chunk_movement_mode;
        let mut new_movement_source = particle_movement_state_current.clone();

        ui.label("Iteration Mode:");
        ui.horizontal(|ui| {
            if ui
                .radio_value(
                    &mut new_movement_source,
                    MovementSource::Particles,
                    "Particles",
                )
                .clicked()
            {
                commands.trigger(UpdateMovementSourceSignal(MovementSource::Particles));
            }
            if ui
                .radio_value(&mut new_movement_source, MovementSource::Chunks, "Chunks")
                .clicked()
            {
                commands.trigger(UpdateMovementSourceSignal(MovementSource::Chunks));
            }
        });

        ui.add_space(4.0);

        ui.label("Chunk Movement Mode:");
        ui.horizontal(|ui| {
            let chunk_mode_enabled = new_movement_source == MovementSource::Chunks;

            if ui
                .add_enabled(
                    chunk_mode_enabled,
                    egui::RadioButton::new(
                        new_movement_mode == ChunkMovementMode::Parallel,
                        "Parallel",
                    ),
                )
                .clicked()
            {
                commands.trigger(UpdateChunkMovementModeSignal(ChunkMovementMode::Parallel));
            }

            if ui
                .add_enabled(
                    chunk_mode_enabled,
                    egui::RadioButton::new(
                        new_movement_mode == ChunkMovementMode::Serial,
                        "Serial",
                    ),
                )
                .clicked()
            {
                commands.trigger(UpdateChunkMovementModeSignal(ChunkMovementMode::Serial));
            }
        });
    }
}
