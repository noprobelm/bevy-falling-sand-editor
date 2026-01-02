use crate::ui::file_browser::{FileBrowser, FileBrowserState};
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_falling_sand::prelude::{LoadSceneMessage, SaveSceneMessage};

pub(super) struct ScenesPlugin;

impl bevy::prelude::Plugin for ScenesPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.init_resource::<SceneFileBrowserState>()
            .add_systems(Update, handle_scene_dialog_markers);
    }
}

#[derive(Resource)]
pub struct SceneFileBrowserState(pub FileBrowserState);

impl Default for SceneFileBrowserState {
    fn default() -> Self {
        Self(FileBrowserState::new("assets/scenes", "bfs", "Scene Files"))
    }
}

pub struct SceneManagementUI;

impl SceneManagementUI {
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        scene_browser_state: &mut ResMut<SceneFileBrowserState>,
        ev_save_scene: &mut MessageWriter<SaveSceneMessage>,
        ev_load_scene: &mut MessageWriter<LoadSceneMessage>,
    ) {
        let file_browser = FileBrowser;
        let state = &mut scene_browser_state.0;

        file_browser.render_save_dialog_with_options(
            ui,
            state,
            |ui, browser_state| {
                ui.checkbox(&mut browser_state.preserve_color, "Preserve colors");
            },
            |path, browser_state| {
                ev_save_scene.write(SaveSceneMessage::new(path, browser_state.preserve_color));
            },
        );

        file_browser.render_load_dialog(ui, &mut scene_browser_state.0, |path| {
            ev_load_scene.write(LoadSceneMessage(path));
        });
    }
}

fn handle_scene_dialog_markers(
    mut commands: Commands,
    load_markers: Query<Entity, With<ShowLoadSceneDialogMarker>>,
    save_markers: Query<Entity, With<ShowSaveSceneDialogMarker>>,
    mut scene_browser_state: ResMut<SceneFileBrowserState>,
) {
    for entity in load_markers.iter() {
        scene_browser_state.0.show_load("Load Scene");
        commands.entity(entity).despawn();
    }

    for entity in save_markers.iter() {
        scene_browser_state.0.show_save("Save Scene");
        commands.entity(entity).despawn();
    }
}

pub fn spawn_load_scene_dialog(commands: &mut Commands) {
    commands.spawn_empty().insert(ShowLoadSceneDialogMarker);
}

pub fn spawn_save_scene_dialog(commands: &mut Commands) {
    commands.spawn_empty().insert(ShowSaveSceneDialogMarker);
}

#[derive(Component)]
struct ShowLoadSceneDialogMarker;

#[derive(Component)]
struct ShowSaveSceneDialogMarker;
