use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use super::setup::SidePanelIconTextureIds;
use crate::ui::{
    ParticleEditorApplicationState, PopupState, SettingsApplicationState, ShowUi, UiSystems,
};

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .run_if(resource_exists::<SidePanelIconTextureIds>)
                .in_set(UiSystems::ActionPanel),
        );
    }
}

fn show(
    mut contexts: EguiContexts,
    icons: Res<SidePanelIconTextureIds>,
    current_particle_editor_app_state: Res<State<PopupState<ParticleEditorApplicationState>>>,
    mut next_particle_editor_app_state: ResMut<
        NextState<PopupState<ParticleEditorApplicationState>>,
    >,
    current_settings_app_state: Res<State<PopupState<SettingsApplicationState>>>,
    mut next_settings_app_state: ResMut<NextState<PopupState<SettingsApplicationState>>>,
) -> Result {
    const IMAGE_SIZE: f32 = 32.;
    const WIDGET_WIDTH: f32 = 40.;
    const IMAGE_MARGIN: f32 = 5.;
    const LOWER_MARGIN: f32 = 2.;
    const WIDGET_ACTIVE_BUTTON_COLORS: [u8; 3] = [100, 100, 100];

    let ctx = contexts.ctx_mut()?;

    egui::Window::new("action panel")
        .title_bar(false)
        .resizable(false)
        .min_width(WIDGET_WIDTH)
        .max_width(WIDGET_WIDTH)
        .show(ctx, |ui| {
            ui.style_mut()
                .visuals
                .widgets
                .noninteractive
                .bg_stroke
                .width = 4.0;
            ui.separator();
            ui.add_space(IMAGE_MARGIN);
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                // Particle editor
                ui.scope(|ui| {
                    set_button_styling(
                        ui,
                        WIDGET_ACTIVE_BUTTON_COLORS,
                        current_particle_editor_app_state.get(),
                    );

                    if ui
                        .add(button_builder(icons.particle_editor, IMAGE_SIZE))
                        .on_hover_ui(|ui| {
                            ui.label("Particle Editor");
                        })
                        .clicked()
                    {
                        next_particle_editor_app_state
                            .set(current_particle_editor_app_state.get_next());
                    }
                });

                ui.add_space(IMAGE_MARGIN);

                // Settings
                ui.scope(|ui| {
                    set_button_styling(
                        ui,
                        WIDGET_ACTIVE_BUTTON_COLORS,
                        current_settings_app_state.get(),
                    );

                    if ui
                        .add(button_builder(icons.settings, IMAGE_SIZE))
                        .on_hover_ui(|ui| {
                            ui.label("Settings");
                        })
                        .clicked()
                    {
                        next_settings_app_state.set(current_settings_app_state.get_next());
                    }
                });

                ui.add_space(LOWER_MARGIN);
            });
        });
    Ok(())
}

fn button_builder(texture_id: egui::TextureId, image_size: f32) -> egui::Button<'static> {
    let image = egui::Image::new((texture_id, egui::vec2(image_size, image_size)));
    egui::Button::image(image)
}

fn set_button_styling<
    T: Send + Sync + Default + std::fmt::Debug + Clone + Eq + std::hash::Hash + 'static,
>(
    ui: &mut egui::Ui,
    colors: [u8; 3],
    current: &PopupState<T>,
) {
    let widgets = &mut ui.style_mut().visuals.widgets;
    widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
    widgets.hovered.bg_stroke.width = 0.0;
    widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(colors[0], colors[1], colors[2]);

    if current.is_open() {
        widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(colors[0], colors[1], colors[2]);
    }
}
