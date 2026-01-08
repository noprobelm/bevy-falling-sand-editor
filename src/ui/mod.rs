mod console;
pub mod file_browser;
mod overlays;
mod particle_editor;
pub mod particle_search;
mod quick_actions;
pub mod settings;
mod statistics_panel;
mod top_bar;

use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy_falling_sand::prelude::{
    ActiveParticleCount, ChunkMovementMode, DynamicParticleCount, LoadSceneMessage, MovementSource,
    ParticleTypeMaterialsParam, ParticleTypeRegistry, ResetParticleTypeChildrenSignal, RigidBodyCount,
    SaveSceneMessage, TotalParticleCount, WallParticleCount,
};
use console::core::{ConsoleCache, ConsoleCommandEntered, ConsoleConfiguration};
use console::{Console, ConsolePlugin};
use quick_actions::*;

use crate::brush::{
    Brush, BrushColor, BrushForceColor, BrushMode, BrushOverwriteExisting, BrushSize, BrushType,
    MaxBrushSize,
};
use crate::cursor::{CursorPosition, HoveredParticle};
use crate::scenes::{SceneFileBrowserState, SceneManagementUI};
use crate::ui::file_browser::FileBrowserState;
pub use console::core::ConsoleState;
use overlays::OverlaysPlugin;
use particle_editor::{
    ApplyEditorChangesAndReset, CreateNewParticle, CurrentEditorSelection, LoadParticleIntoEditor,
    ParticleEditorData,
};
use particle_editor::{ParticleEditor, ParticleEditorPlugin};
use particle_search::{
    handle_particle_search_input, update_particle_search_cache, ParticleSearch,
    ParticleSearchCache, ParticleSearchState,
};
use bevy_falling_sand::debug::{
    ChunkBorderColor, DebugDirtyRects, DebugParticleCount, DebugParticleMap, DirtyRectColor,
};
use settings::{
    BrushSettingsParams, DebugSettingsParams, SettingsPlugin, SettingsState, SettingsWindow,
};
use statistics_panel::StatisticsPanel;
pub use top_bar::particle_files::ParticleFileDialog;
use top_bar::particle_files::{
    LoadParticlesSceneMessage, ParticleFileBrowser, SaveParticlesSceneMessage,
};
use top_bar::{ParticleFilesPlugin, UiTopBar};

use bevy::prelude::*;
pub(super) use bevy_egui::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            ConsolePlugin,
            ParticleEditorPlugin,
            ParticleFilesPlugin,
            FrameTimeDiagnosticsPlugin::default(),
            QuickActionsPlugin,
            OverlaysPlugin,
            SettingsPlugin,
        ))
        .init_resource::<RenderGui>()
        .init_resource::<ParticleSearchState>()
        .init_resource::<ParticleSearchCache>()
        .init_resource::<StatisticsPanel>()
        .add_systems(
            Update,
            (console::receive_console_line, update_particle_search_cache),
        )
        .add_systems(
            EguiPrimaryContextPass,
            render_ui_panels.run_if(resource_exists::<RenderGui>),
        )
        .add_systems(EguiPrimaryContextPass, render_particle_search)
        .add_systems(EguiPrimaryContextPass, handle_particle_search_input)
        .add_systems(Update, set_default_ui_scale.run_if(run_once));
    }
}

fn set_default_ui_scale(mut egui_settings: Single<&mut EguiContextSettings>) {
    egui_settings.scale_factor = 1.25;
}

#[derive(Resource, Clone, Default, Debug)]
pub struct RenderGui;

type UiSystemParams<'w, 's> = (
    Commands<'w, 's>,
    EguiContexts<'w, 's>,
    ResMut<'w, console::core::ConsoleState>,
    Res<'w, ConsoleCache>,
    Res<'w, ConsoleConfiguration>,
    MessageWriter<'w, ConsoleCommandEntered>,
    ParticleTypeMaterialsParam<'w, 's>,
    Res<'w, CurrentEditorSelection>,
    Query<'w, 's, &'static mut ParticleEditorData>,
    MessageWriter<'w, LoadParticleIntoEditor>,
    MessageWriter<'w, CreateNewParticle>,
    MessageWriter<'w, ApplyEditorChangesAndReset>,
    MessageWriter<'w, ResetParticleTypeChildrenSignal>,
    Res<'w, ParticleTypeRegistry>,
    Res<'w, ParticleFileDialog>,
);

type UiRenderParams<'w> = (
    Res<'w, StatisticsPanel>,
    Res<'w, DynamicParticleCount>,
    Res<'w, WallParticleCount>,
    Res<'w, TotalParticleCount>,
    Res<'w, ActiveParticleCount>,
    Res<'w, RigidBodyCount>,
    Res<'w, DiagnosticsStore>,
    Res<'w, MovementSource>,
    Res<'w, ParticleSearchCache>,
    Res<'w, ChunkMovementMode>,
    ResMut<'w, BrushForceColor>,
);

type UiSceneParams<'w> = (
    ResMut<'w, SceneFileBrowserState>,
    MessageWriter<'w, SaveSceneMessage>,
    MessageWriter<'w, LoadSceneMessage>,
    ResMut<'w, FileBrowserState>,
    MessageWriter<'w, SaveParticlesSceneMessage>,
    MessageWriter<'w, LoadParticlesSceneMessage>,
    ResMut<'w, SettingsState>,
    ResMut<'w, GizmoConfigStore>,
);

type UiBrushParams<'w, 's> = (
    ResMut<'w, MaxBrushSize>,
    Query<'w, 's, (&'static mut BrushSize, &'static mut BrushColor), With<Brush>>,
    Res<'w, State<BrushType>>,
    ResMut<'w, NextState<BrushType>>,
    Res<'w, State<BrushMode>>,
    ResMut<'w, NextState<BrushMode>>,
    ResMut<'w, BrushOverwriteExisting>,
);

type CanvasParams<'w> = (Res<'w, HoveredParticle>, Res<'w, CursorPosition>);

type UiDebugParams<'w> = (
    Option<Res<'w, DebugParticleCount>>,
    Option<Res<'w, DebugParticleMap>>,
    Option<Res<'w, DebugDirtyRects>>,
    ResMut<'w, ChunkBorderColor>,
    ResMut<'w, DirtyRectColor>,
);

fn render_ui_panels(
    (
        mut commands,
        mut contexts,
        mut console_state,
        cache,
        config,
        mut command_writer,
        particle_materials,
        current_editor,
        mut editor_data_query,
        mut load_particle_messages,
        mut create_particle_messages,
        mut apply_editor_and_reset_messages,
        mut reset_particle_children_messages,
        particle_type_map,
        particle_file_dialog,
    ): UiSystemParams,
    (
        statistics_panel,
        dynamic_particle_count,
        wall_particle_count,
        total_particle_count,
        active_particle_count,
        rigid_body_count,
        diagnostics,
        particle_movement_state_current,
        particle_search_cache,
        chunk_movement_mode,
        mut brush_force_color,
    ): UiRenderParams,
    (
        mut scene_browser_state,
        mut ev_save_scene,
        mut ev_load_scene,
        mut particle_file_browser_state,
        mut ev_save_particles_scene,
        mut ev_load_particles_scene,
        mut settings_state,
        mut gizmo_store,
    ): UiSceneParams,
    (
        max_brush_size,
        brush_query,
        brush_type,
        brush_type_next,
        brush_mode,
        brush_mode_next,
        brush_overwrite,
    ): UiBrushParams,
    (hovered_particle, cursor_position): CanvasParams,
    (
        debug_particle_count,
        debug_particle_map,
        debug_dirty_rects,
        chunk_border_color,
        dirty_rect_color,
    ): UiDebugParams,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let _top_response = egui::TopBottomPanel::top("Top panel").show(ctx, |ui| {
        egui::MenuBar::new().ui(ui, |ui| {
            UiTopBar.render(
                ui,
                &mut commands,
                &mut particle_file_browser_state,
                &mut settings_state,
                hovered_particle,
                cursor_position,
            );

            if let Some(ref error) = particle_file_dialog.last_error {
                ui.separator();
                ui.colored_label(egui::Color32::RED, format!("Error: {}", error));
            }

            if let Some(ref success) = particle_file_dialog.last_success {
                ui.separator();
                ui.colored_label(egui::Color32::GREEN, success);
            }
        });
    });

    let screen_width = ctx.screen_rect().width();
    let panel_width = if screen_width < 1600.0 { 500.0 } else { 700.0 };

    let _left_response = egui::SidePanel::left("Left panel")
        .resizable(true)
        .default_width(panel_width)
        .show(ctx, |ui| {
            egui::ScrollArea::vertical()
                .auto_shrink([false, false])
                .show(ui, |ui| {
                    ui.heading("Particle Editor");
                    ui.separator();

                    ParticleEditor.render(
                        ui,
                        &particle_materials,
                        &current_editor,
                        &mut editor_data_query,
                        &mut load_particle_messages,
                        &mut create_particle_messages,
                        &mut apply_editor_and_reset_messages,
                        &mut reset_particle_children_messages,
                        &particle_type_map,
                    );

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);

                    ui.horizontal(|ui| {
                        ui.checkbox(&mut brush_force_color.enabled, "Force Particle Color");
                        let buttons_width = 170.0;
                        ui.add_space(ui.available_width() - buttons_width);
                        if ui.button("🗑 Clear All").clicked() {
                            brush_force_color.colors.clear();
                            brush_force_color.colors.push(Color::srgba_u8(255, 255, 255, 255));
                            brush_force_color.selected_index = 0;
                        }
                        if ui.button("➕ Add Color").clicked() {
                            let new_color = brush_force_color
                                .colors
                                .last()
                                .copied()
                                .unwrap_or(Color::srgba_u8(255, 255, 255, 255));
                            brush_force_color.colors.push(new_color);
                        }
                    });

                    if brush_force_color.enabled {
                        let mut to_remove: Option<usize> = None;
                        let colors_len = brush_force_color.colors.len();
                        for i in 0..colors_len {
                            ui.horizontal(|ui| {
                                let is_selected = brush_force_color.selected_index == i;
                                if ui.radio(is_selected, "").clicked() {
                                    brush_force_color.selected_index = i;
                                }

                                let srgba = brush_force_color.colors[i].to_srgba();
                                let value_size = [24.0, 18.0];
                                ui.label("R:");
                                ui.add_sized(
                                    value_size,
                                    egui::Label::new(format!("{:.0}", srgba.red * 255.0)),
                                );
                                ui.label("G:");
                                ui.add_sized(
                                    value_size,
                                    egui::Label::new(format!("{:.0}", srgba.green * 255.0)),
                                );
                                ui.label("B:");
                                ui.add_sized(
                                    value_size,
                                    egui::Label::new(format!("{:.0}", srgba.blue * 255.0)),
                                );
                                ui.label("A:");
                                ui.add_sized(
                                    value_size,
                                    egui::Label::new(format!("{:.0}", srgba.alpha * 255.0)),
                                );

                                let mut color32 = egui::Color32::from_rgba_unmultiplied(
                                    (srgba.red * 255.0) as u8,
                                    (srgba.green * 255.0) as u8,
                                    (srgba.blue * 255.0) as u8,
                                    (srgba.alpha * 255.0) as u8,
                                );

                                ui.push_id(format!("force_color_{}", i), |ui| {
                                    if ui.color_edit_button_srgba(&mut color32).changed() {
                                        brush_force_color.colors[i] = Color::srgba_u8(
                                            color32.r(),
                                            color32.g(),
                                            color32.b(),
                                            color32.a(),
                                        );
                                    }
                                });

                                if ui.button(format!("❌ {}", i)).clicked() && colors_len > 1 {
                                    to_remove = Some(i);
                                }
                            });
                        }

                        if let Some(remove_index) = to_remove {
                            brush_force_color.colors.remove(remove_index);
                            if brush_force_color.selected_index >= brush_force_color.colors.len() {
                                brush_force_color.selected_index =
                                    brush_force_color.colors.len().saturating_sub(1);
                            }
                        }
                    }

                    ui.add_space(16.0);
                    ui.separator();
                    ui.add_space(16.0);

                    ui.heading("Statistics");
                    ui.separator();

                    let fps = diagnostics
                        .get(&FrameTimeDiagnosticsPlugin::FPS)
                        .and_then(|fps| fps.smoothed())
                        .unwrap_or(0.0) as f32;

                    statistics_panel.as_ref().render(
                        ui,
                        &particle_movement_state_current,
                        chunk_movement_mode.as_ref(),
                        fps,
                        dynamic_particle_count.0 as u32,
                        wall_particle_count.0 as u32,
                        total_particle_count.0 as u32,
                        active_particle_count.0 as u32,
                        rigid_body_count.0 as u32,
                        &mut commands,
                    );

                    ui.add_space(16.0);
                });
        });

    let screen_height = ctx.screen_rect().height();
    let console_height = if console_state.expanded {
        console_state.height.min(screen_height * 0.5).max(80.0)
    } else {
        25.0
    };

    let console_frame = if console_state.expanded {
        egui::Frame::NONE.fill(egui::Color32::from_rgb(46, 46, 46))
    } else {
        egui::Frame::NONE
    };

    let _console_response = egui::TopBottomPanel::bottom("Console panel")
        .exact_height(console_height)
        .frame(console_frame)
        .show(ctx, |ui| {
            if !console_state.expanded {
                ui.spacing_mut().item_spacing = egui::Vec2::ZERO;
                ui.spacing_mut().button_padding = egui::Vec2::ZERO;
                ui.spacing_mut().menu_margin = egui::Margin::ZERO;
            }

            Console.render(
                ui,
                &mut console_state,
                &cache,
                &config,
                &mut command_writer,
                Some(&particle_search_cache),
            );
        });

    SceneManagementUI.render(
        &mut egui::Ui::new(
            ctx.clone(),
            egui::Id::new("scene_management"),
            egui::UiBuilder::new().max_rect(ctx.screen_rect()),
        ),
        &mut scene_browser_state,
        &mut ev_save_scene,
        &mut ev_load_scene,
    );

    ParticleFileBrowser.render(
        &mut egui::Ui::new(
            ctx.clone(),
            egui::Id::new("particle_file_browser"),
            egui::UiBuilder::new().max_rect(ctx.screen_rect()),
        ),
        &mut particle_file_browser_state,
        &mut ev_save_particles_scene,
        &mut ev_load_particles_scene,
    );

    let mut brush_params = BrushSettingsParams {
        max_brush_size,
        brush_query,
        brush_type,
        brush_type_next,
        brush_mode,
        brush_mode_next,
        brush_overwrite,
    };

    let mut debug_params = DebugSettingsParams {
        commands,
        debug_particle_count,
        debug_particle_map,
        debug_dirty_rects,
        chunk_border_color,
        dirty_rect_color,
    };

    SettingsWindow.render(
        &mut egui::Ui::new(
            ctx.clone(),
            egui::Id::new("settings_window"),
            egui::UiBuilder::new().max_rect(ctx.screen_rect()),
        ),
        &mut settings_state,
        &mut gizmo_store,
        &mut brush_params,
        &mut debug_params,
    );

    Ok(())
}

type ParticleSearchParams<'w, 's> = (
    EguiContexts<'w, 's>,
    ResMut<'w, ParticleSearchState>,
    Res<'w, ParticleSearchCache>,
    MessageWriter<'w, LoadParticleIntoEditor>,
);

fn render_particle_search(
    (
        mut contexts,
        mut particle_search_state,
        particle_search_cache,
        mut load_particle_messages,
    ): ParticleSearchParams,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let mut particle_search_ui = egui::Ui::new(
        ctx.clone(),
        egui::Id::new("particle_search"),
        egui::UiBuilder::new().max_rect(ctx.screen_rect()),
    );

    ParticleSearch.render(
        &mut particle_search_ui,
        &mut particle_search_state,
        &particle_search_cache,
        &mut load_particle_messages,
    );
    Ok(())
}
