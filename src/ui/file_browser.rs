use bevy::prelude::*;
use bevy_egui::egui;
use std::path::PathBuf;

#[derive(Resource, Default)]
pub struct FileBrowserState {
    pub show_save_dialog: bool,
    pub show_load_dialog: bool,
    pub save_input_text: String,
    pub selected_file: Option<String>,
    pub current_directory: String,
    pub file_extension: String,
    pub dialog_title: String,
    /// Optional checkbox state for "preserve colors" in save dialogs.
    pub preserve_color: bool,
}

fn get_project_root_path(relative_path: &str) -> String {
    let manifest_dir = env!("CARGO_MANIFEST_DIR");
    format!("{}/{}", manifest_dir, relative_path)
}

impl FileBrowserState {
    pub fn new(directory: &str, extension: &str, title: &str) -> Self {
        Self {
            current_directory: get_project_root_path(directory),
            file_extension: extension.to_string(),
            dialog_title: title.to_string(),
            ..Default::default()
        }
    }

    pub fn show_save(&mut self, title: &str) {
        self.dialog_title = title.to_string();
        self.show_save_dialog = true;
        self.selected_file = None;
        self.save_input_text.clear();
    }

    pub fn show_load(&mut self, title: &str) {
        self.dialog_title = title.to_string();
        self.show_load_dialog = true;
        self.selected_file = None;
    }
}

pub struct FileBrowser;

impl FileBrowser {
    pub fn render_save_dialog<F>(
        &self,
        ui: &mut egui::Ui,
        state: &mut FileBrowserState,
        mut on_save: F,
    ) where
        F: FnMut(PathBuf),
    {
        self.render_save_dialog_with_options(ui, state, |_, _| {}, |path, _| on_save(path));
    }

    pub fn render_save_dialog_with_options<O, F>(
        &self,
        ui: &mut egui::Ui,
        state: &mut FileBrowserState,
        mut render_options: O,
        mut on_save: F,
    ) where
        O: FnMut(&mut egui::Ui, &mut FileBrowserState),
        F: FnMut(PathBuf, &FileBrowserState),
    {
        if !state.show_save_dialog {
            return;
        }

        let all_files: Vec<(String, String)> = std::fs::read_dir(&state.current_directory)
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.extension()? == state.file_extension.as_str() {
                            let file_name = path.file_name()?.to_str()?;
                            let display_name =
                                file_name.trim_end_matches(&format!(".{}", state.file_extension));
                            let full_path = format!("{}/{}", state.current_directory, file_name);
                            Some((display_name.to_string(), full_path))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        let mut should_save = false;
        let mut should_close = false;
        let mut save_path = None;

        egui::Window::new(&state.dialog_title)
            .collapsible(false)
            .resizable(true)
            .default_size([400.0, 300.0])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    ui.label("File name:");
                    ui.text_edit_singleline(&mut state.save_input_text);
                });

                ui.separator();
                ui.label("Existing files:");

                egui::ScrollArea::vertical()
                    .max_height(150.0)
                    .show(ui, |ui| {
                        for (display_name, _) in &all_files {
                            if ui
                                .selectable_label(
                                    state.selected_file.as_ref() == Some(display_name),
                                    display_name,
                                )
                                .clicked()
                            {
                                state.selected_file = Some(display_name.clone());
                                state.save_input_text = display_name.clone();
                            }
                        }
                    });

                ui.separator();
                render_options(ui, state);

                ui.separator();
                ui.horizontal(|ui| {
                    if ui.button("Save").clicked() {
                        if !state.save_input_text.is_empty() {
                            let mut path = PathBuf::from(&state.current_directory);
                            path.push(&state.save_input_text);
                            path.set_extension(&state.file_extension);
                            save_path = Some(path);
                            should_save = true;
                        }
                        should_close = true;
                    }
                    if ui.button("Cancel").clicked() {
                        should_close = true;
                    }
                });
            });

        if should_save {
            if let Some(path) = save_path {
                on_save(path, state);
            }
        }
        if should_close {
            state.show_save_dialog = false;
            state.selected_file = None;
        }
    }

    pub fn render_load_dialog<F>(
        &self,
        ui: &mut egui::Ui,
        state: &mut FileBrowserState,
        mut on_load: F,
    ) where
        F: FnMut(PathBuf),
    {
        if !state.show_load_dialog {
            return;
        }

        let all_files: Vec<(String, String)> = std::fs::read_dir(&state.current_directory)
            .map(|entries| {
                entries
                    .filter_map(|entry| {
                        let entry = entry.ok()?;
                        let path = entry.path();
                        if path.extension()? == state.file_extension.as_str() {
                            let file_name = path.file_name()?.to_str()?;
                            let display_name =
                                file_name.trim_end_matches(&format!(".{}", state.file_extension));
                            let full_path = format!("{}/{}", state.current_directory, file_name);
                            Some((display_name.to_string(), full_path))
                        } else {
                            None
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        egui::Window::new(&state.dialog_title)
            .collapsible(false)
            .resizable(true)
            .default_size([400.0, 300.0])
            .show(ui.ctx(), |ui| {
                ui.label("Select a file to load:");
                ui.separator();

                egui::ScrollArea::vertical()
                    .max_height(200.0)
                    .show(ui, |ui| {
                        for (display_name, file_path) in &all_files {
                            let is_selected = state.selected_file.as_ref() == Some(display_name);

                            if ui.selectable_label(is_selected, display_name).clicked() {
                                state.selected_file = Some(display_name.clone());
                            }

                            if is_selected
                                && ui.input(|i| {
                                    i.pointer
                                        .button_double_clicked(egui::PointerButton::Primary)
                                })
                            {
                                on_load(PathBuf::from(file_path));
                                state.show_load_dialog = false;
                                state.selected_file = None;
                                break;
                            }
                        }
                    });

                ui.separator();
                if let Some(selected) = &state.selected_file {
                    ui.label(format!("Selected: {}", selected));
                }

                ui.horizontal(|ui| {
                    let load_enabled = state.selected_file.is_some();
                    if ui
                        .add_enabled(load_enabled, egui::Button::new("Load"))
                        .clicked()
                    {
                        if let Some(selected) = &state.selected_file {
                            if let Some((_, file_path)) =
                                all_files.iter().find(|(name, _)| name == selected)
                            {
                                on_load(PathBuf::from(file_path));
                            }
                        }
                        state.show_load_dialog = false;
                        state.selected_file = None;
                    }
                    if ui.button("Cancel").clicked() {
                        state.show_load_dialog = false;
                        state.selected_file = None;
                    }
                });
            });
    }
}
