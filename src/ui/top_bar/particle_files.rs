use std::path::PathBuf;

use crate::ui::file_browser::{FileBrowser, FileBrowserState};
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_falling_sand::prelude::{LoadParticleTypes, ParticleType, SaveParticleDefinitionsMessage};

pub struct ParticleFilesPlugin;

impl Plugin for ParticleFilesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleFileDialog>()
            .insert_resource(FileBrowserState::new(
                "assets/particles",
                "ron",
                "Particle Files",
            ))
            .add_message::<SaveParticlesSceneMessage>()
            .add_message::<LoadParticlesSceneMessage>()
            .add_systems(
                Update,
                (save_particles_scene_system, load_particles_scene_system),
            );
    }
}

#[derive(Resource, Default)]
pub struct ParticleFileDialog {
    pub last_error: Option<String>,
    pub last_success: Option<String>,
}

#[derive(Message)]
pub struct SaveParticlesSceneMessage(pub PathBuf);

#[derive(Message)]
pub struct LoadParticlesSceneMessage(pub PathBuf);

pub fn spawn_save_scene_dialog(browser_state: &mut ResMut<FileBrowserState>) {
    browser_state.show_save("Save Particle Set");
}

pub fn spawn_load_scene_dialog(browser_state: &mut ResMut<FileBrowserState>) {
    browser_state.show_load("Load Particle Set");
}

pub struct ParticleFileBrowser;

impl ParticleFileBrowser {
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        browser_state: &mut ResMut<FileBrowserState>,
        ev_save_particles_scene: &mut MessageWriter<SaveParticlesSceneMessage>,
        ev_load_particles_scene: &mut MessageWriter<LoadParticlesSceneMessage>,
    ) {
        let file_browser = FileBrowser;

        file_browser.render_save_dialog(ui, browser_state, |path| {
            ev_save_particles_scene.write(SaveParticlesSceneMessage(path));
        });

        file_browser.render_load_dialog(ui, browser_state, |path| {
            ev_load_particles_scene.write(LoadParticlesSceneMessage(path));
        });
    }
}

// Scene-based particle definition systems
fn save_particles_scene_system(
    mut ev_save_particles_scene: MessageReader<SaveParticlesSceneMessage>,
    mut ev_save_definitions: MessageWriter<SaveParticleDefinitionsMessage>,
    mut dialog_state: ResMut<ParticleFileDialog>,
) {
    for SaveParticlesSceneMessage(save_path) in ev_save_particles_scene.read() {
        // Convert .ron to .particles.scn.ron for scene format
        let mut scene_path = save_path.clone();
        scene_path.set_extension("particles.scn.ron");

        ev_save_definitions.write(SaveParticleDefinitionsMessage(scene_path.clone()));

        dialog_state.last_success = Some(format!(
            "Saving particle definitions to scene format: {}",
            scene_path.display()
        ));
        dialog_state.last_error = None;
    }
}

fn load_particles_scene_system(
    mut commands: Commands,
    mut ev_load_particles_scene: MessageReader<LoadParticlesSceneMessage>,
    mut ev_load_scene: MessageWriter<LoadParticleTypes>,
    mut dialog_state: ResMut<ParticleFileDialog>,
    particle_query: Query<Entity, With<ParticleType>>,
) {
    for LoadParticlesSceneMessage(path) in ev_load_particles_scene.read() {
        // First, despawn all existing particle types
        for entity in particle_query.iter() {
            commands.entity(entity).despawn();
        }

        // Then load from the scene
        ev_load_scene.write(LoadParticleTypes(path.clone()));

        dialog_state.last_success = Some(format!(
            "Loading particle definitions from scene: {}",
            path.display()
        ));
        dialog_state.last_error = None;
    }
}
