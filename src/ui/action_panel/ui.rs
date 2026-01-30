use bevy::{asset::meta::Settings, prelude::*};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};

use super::setup::SidePanelIconTextureIds;
use crate::ui::{
    SettingsApplicationState, ShowUi, UiSystems, particle_editor::ParticleEditorApplicationState,
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
    current_particle_editor_app_state: Res<State<ParticleEditorApplicationState>>,
    mut next_particle_editor_app_state: ResMut<NextState<ParticleEditorApplicationState>>,
    current_settings_app_state: Res<State<SettingsApplicationState>>,
    mut next_settings_app_state: ResMut<NextState<SettingsApplicationState>>,
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
                    let widgets = &mut ui.style_mut().visuals.widgets;
                    widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    widgets.hovered.bg_stroke.width = 0.0;
                    widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(
                        WIDGET_ACTIVE_BUTTON_COLORS[0],
                        WIDGET_ACTIVE_BUTTON_COLORS[1],
                        WIDGET_ACTIVE_BUTTON_COLORS[2],
                    );

                    if current_particle_editor_app_state.get()
                        == &ParticleEditorApplicationState::Open
                    {
                        widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(
                            WIDGET_ACTIVE_BUTTON_COLORS[0],
                            WIDGET_ACTIVE_BUTTON_COLORS[1],
                            WIDGET_ACTIVE_BUTTON_COLORS[2],
                        );
                    }

                    if ui
                        .add(button_builder(icons.particle_editor, IMAGE_SIZE))
                        .on_hover_ui(|ui| {
                            ui.label("Particle Editor");
                        })
                        .clicked()
                    {
                        next_particle_editor_app_state.set(match current_particle_editor_app_state
                            .get()
                        {
                            ParticleEditorApplicationState::Closed => {
                                ParticleEditorApplicationState::Open
                            }
                            ParticleEditorApplicationState::Open => {
                                ParticleEditorApplicationState::Closed
                            }
                        });
                    }
                });

                ui.add_space(IMAGE_MARGIN);

                // Settings
                ui.scope(|ui| {
                    let widgets = &mut ui.style_mut().visuals.widgets;
                    widgets.inactive.weak_bg_fill = egui::Color32::TRANSPARENT;
                    widgets.hovered.bg_stroke.width = 0.0;
                    widgets.hovered.weak_bg_fill = egui::Color32::from_rgb(
                        WIDGET_ACTIVE_BUTTON_COLORS[0],
                        WIDGET_ACTIVE_BUTTON_COLORS[1],
                        WIDGET_ACTIVE_BUTTON_COLORS[2],
                    );

                    if current_settings_app_state.get() == &SettingsApplicationState::Open {
                        widgets.inactive.weak_bg_fill = egui::Color32::from_rgb(
                            WIDGET_ACTIVE_BUTTON_COLORS[0],
                            WIDGET_ACTIVE_BUTTON_COLORS[1],
                            WIDGET_ACTIVE_BUTTON_COLORS[2],
                        );
                    }

                    if ui
                        .add(button_builder(icons.settings, IMAGE_SIZE))
                        .on_hover_ui(|ui| {
                            ui.label("Settings");
                        })
                        .clicked()
                    {
                        next_particle_editor_app_state.set(match current_particle_editor_app_state
                            .get()
                        {
                            ParticleEditorApplicationState::Closed => {
                                ParticleEditorApplicationState::Open
                            }
                            ParticleEditorApplicationState::Open => {
                                ParticleEditorApplicationState::Closed
                            }
                        });
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
