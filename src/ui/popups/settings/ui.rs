use avian2d::prelude::PhysicsGizmos;
use bevy::{ecs::system::SystemParam, prelude::*, reflect::Enum};
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use bevy_falling_sand::debug::{ChunkColor, DebugDirtyRects, DebugParticleMap, DirtyRectColor};
use leafwing_input_manager::prelude::{InputMap, MouseScrollAxis};

use super::{KeybindsListeningState, ListeningForKeybind};
use crate::{
    brush::{
        BrushAction, BrushKeyBindings, BrushSize, BrushSpawnState, BrushTypeState,
        CanvasStateActions,
    },
    camera::{CameraAction, CameraKeyBindings},
    config::{AvianDebugConfig, InputButton, OptionalColor},
    ui::{
        ConsoleAction, QuickAction, SettingsApplicationState, SettingsCategory, ShowUi,
        UiKeyBindings, UiSystems, add_label_with_drag_value, add_label_with_toggle_switch,
        add_major_grid_separator,
    },
};

/// System param to fetch particle types by material type.
#[derive(SystemParam)]
struct SettingsParam<'w, 's> {
    pub commands: Commands<'w, 's>,
    pub current_settings_category: ResMut<'w, State<SettingsCategory>>,
    pub next_settings_category: ResMut<'w, NextState<SettingsCategory>>,
    pub brush: BrushSettingsParam<'w, 's>,
    pub debug_falling_sand: BevyFallingSandDebugSettingsParam<'w>,
    pub avian: AvianDebugSettingsParam<'w>,
    pub keybinds: KeybindsSettingsParam<'w>,
}

#[derive(SystemParam)]
struct KeybindsSettingsParam<'w> {
    pub camera_keys: ResMut<'w, CameraKeyBindings>,
    pub brush_keys: ResMut<'w, BrushKeyBindings>,
    pub ui_keys: ResMut<'w, UiKeyBindings>,
    pub listening: Option<Res<'w, ListeningForKeybind>>,
    pub next_listening_state: ResMut<'w, NextState<KeybindsListeningState>>,
}

#[derive(SystemParam)]
struct BrushSettingsParam<'w, 's> {
    pub size: Single<'w, 's, &'static mut crate::brush::BrushSize>,
    pub current_type_state: Res<'w, State<BrushTypeState>>,
    pub next_type_state: ResMut<'w, NextState<BrushTypeState>>,
    pub current_mode_state: Res<'w, State<BrushSpawnState>>,
    pub next_mode_state: ResMut<'w, NextState<BrushSpawnState>>,
}

#[derive(SystemParam)]
struct BevyFallingSandDebugSettingsParam<'w> {
    pub map: Option<Res<'w, DebugParticleMap>>,
    pub map_color: ResMut<'w, ChunkColor>,
    pub dirty_rects: Option<Res<'w, DebugDirtyRects>>,
    pub dirty_rects_color: ResMut<'w, DirtyRectColor>,
}

#[derive(SystemParam)]
struct AvianDebugSettingsParam<'w> {
    pub gizmo_store: ResMut<'w, GizmoConfigStore>,
    pub avian_debug: ResMut<'w, AvianDebugConfig>,
}

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .run_if(in_state(SettingsApplicationState::Open))
                .in_set(UiSystems::Settings),
        );
    }
}

fn show(mut contexts: EguiContexts, mut settings_param: SettingsParam) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("Settings")
        .constrain_to(ctx.available_rect())
        .show(ctx, |ui| {
            ui.horizontal(|ui| {
                for variant in [
                    SettingsCategory::Brush,
                    SettingsCategory::Debug,
                    SettingsCategory::Keybinds,
                ] {
                    let selected = variant == *settings_param.current_settings_category.get();
                    if ui
                        .selectable_label(selected, variant.variant_name())
                        .clicked()
                    {
                        settings_param.next_settings_category.set(variant);
                    }
                }
            });
            ui.vertical(|ui| {
                ui.separator();
                match *settings_param.current_settings_category.get() {
                    SettingsCategory::Brush => show_brush_settings(ui, &mut settings_param),
                    SettingsCategory::Debug => show_debug_settings(ui, &mut settings_param),
                    SettingsCategory::Keybinds => show_keybinds_settings(ui, &mut settings_param),
                };
            });
        });

    Ok(())
}

fn show_brush_settings(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    egui::Grid::new("brush_grid").num_columns(2).show(ui, |ui| {
        show_brush_size(ui, settings_param);
        show_brush_type_selection(ui, settings_param);
        show_brush_mode_selection(ui, settings_param);
    });
}

fn show_brush_size(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let new_value =
        add_label_with_drag_value(ui, 0, "Size", settings_param.brush.size.0, 0..=50, 1.0);
    settings_param.brush.size.set_if_neq(BrushSize(new_value));
}

fn show_brush_type_selection(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    ui.label("Type");
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        egui::ComboBox::from_id_salt("brush_type_combo")
            .selected_text(settings_param.brush.current_type_state.get().variant_name())
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(
                        matches!(
                            settings_param.brush.current_type_state.get(),
                            BrushTypeState::Circle
                        ),
                        "Circle",
                    )
                    .clicked()
                {
                    settings_param
                        .brush
                        .next_type_state
                        .set(BrushTypeState::Circle)
                } else if ui
                    .selectable_label(
                        matches!(
                            settings_param.brush.current_type_state.get(),
                            BrushTypeState::Line
                        ),
                        "Line",
                    )
                    .clicked()
                {
                    settings_param
                        .brush
                        .next_type_state
                        .set(BrushTypeState::Line)
                } else if ui
                    .selectable_label(
                        matches!(
                            settings_param.brush.current_type_state.get(),
                            BrushTypeState::Cursor
                        ),
                        "Cursor",
                    )
                    .clicked()
                {
                    settings_param
                        .brush
                        .next_type_state
                        .set(BrushTypeState::Cursor)
                };
            });
    });
    ui.end_row();
}

fn show_brush_mode_selection(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    ui.label("Mode");
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        egui::ComboBox::from_id_salt("brush_mode_combo")
            .selected_text(settings_param.brush.current_mode_state.get().variant_name())
            .show_ui(ui, |ui| {
                if ui
                    .selectable_label(
                        matches!(
                            settings_param.brush.current_mode_state.get(),
                            BrushSpawnState::Spawn
                        ),
                        "Spawn",
                    )
                    .clicked()
                {
                    settings_param
                        .brush
                        .next_mode_state
                        .set(BrushSpawnState::Spawn)
                } else if ui
                    .selectable_label(
                        matches!(
                            settings_param.brush.current_mode_state.get(),
                            BrushSpawnState::Despawn
                        ),
                        "Despawn",
                    )
                    .clicked()
                {
                    settings_param
                        .brush
                        .next_mode_state
                        .set(BrushSpawnState::Despawn)
                };
            });
    });
    ui.end_row();
}

fn show_debug_settings(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    egui::Grid::new("debug_grid").num_columns(2).show(ui, |ui| {
        ui.heading("Falling Sand Debug");
        ui.end_row();
        show_bfs_map(ui, settings_param);
        show_bfs_dirty_rects(ui, settings_param);

        add_major_grid_separator(ui);

        ui.heading("Avian Physics Debug");
        ui.end_row();
        show_avian_settings(ui, settings_param);
    });
}

fn show_bfs_map(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let enabled = settings_param.debug_falling_sand.map.is_some();
    let srgba = settings_param.debug_falling_sand.map_color.0.to_srgba();
    let original = egui::Color32::from_rgba_unmultiplied(
        (srgba.red * 255.) as u8,
        (srgba.green * 255.) as u8,
        (srgba.blue * 255.) as u8,
        (srgba.alpha * 255.) as u8,
    );
    let mut color32 = original;

    ui.label("Show Map");
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let mut is_on = enabled;
        ui.add(crate::ui::widgets::toggle_switch::toggle(&mut is_on));
        ui.color_edit_button_srgba(&mut color32);
        if color32 != original {
            settings_param.debug_falling_sand.map_color.0 =
                Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
        }
        if is_on != enabled {
            if is_on {
                settings_param.commands.insert_resource(DebugParticleMap);
            } else {
                settings_param
                    .commands
                    .remove_resource::<DebugParticleMap>();
            }
        }
    });
    ui.end_row();
}

fn show_bfs_dirty_rects(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let enabled = settings_param.debug_falling_sand.dirty_rects.is_some();
    let srgba = settings_param
        .debug_falling_sand
        .dirty_rects_color
        .0
        .to_srgba();
    let original = egui::Color32::from_rgba_unmultiplied(
        (srgba.red * 255.) as u8,
        (srgba.green * 255.) as u8,
        (srgba.blue * 255.) as u8,
        (srgba.alpha * 255.) as u8,
    );
    let mut color32 = original;

    ui.label("Show Dirty Rects");
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let mut is_on = enabled;
        ui.add(crate::ui::widgets::toggle_switch::toggle(&mut is_on));
        ui.color_edit_button_srgba(&mut color32);
        if color32 != original {
            settings_param.debug_falling_sand.dirty_rects_color.0 =
                Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
        }
        if is_on != enabled {
            if is_on {
                settings_param.commands.insert_resource(DebugDirtyRects);
            } else {
                settings_param.commands.remove_resource::<DebugDirtyRects>();
            }
        }
    });
    ui.end_row();
}

fn show_avian_settings(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let config = &mut *settings_param.avian.avian_debug;

    show_avian_optional_color(ui, "AABB Color", &mut config.aabb_color);
    show_avian_optional_color(ui, "Collider Color", &mut config.collider_color);
    show_avian_optional_color(ui, "Contact Point Color", &mut config.contact_point_color);
    show_avian_optional_color(ui, "Contact Normal Color", &mut config.contact_normal_color);
    show_avian_optional_color(ui, "Joint Anchor Color", &mut config.joint_anchor_color);
    show_avian_optional_color(
        ui,
        "Joint Separation Color",
        &mut config.joint_separation_color,
    );
    show_avian_optional_color(ui, "Raycast Color", &mut config.raycast_color);
    show_avian_optional_color(ui, "Raycast Point Color", &mut config.raycast_point_color);
    show_avian_optional_color(ui, "Raycast Normal Color", &mut config.raycast_normal_color);
    show_avian_optional_color(ui, "Shapecast Color", &mut config.shapecast_color);
    show_avian_optional_color(
        ui,
        "Shapecast Shape Color",
        &mut config.shapecast_shape_color,
    );
    show_avian_optional_color(
        ui,
        "Shapecast Point Color",
        &mut config.shapecast_point_color,
    );
    show_avian_optional_color(
        ui,
        "Shapecast Normal Color",
        &mut config.shapecast_normal_color,
    );
    show_avian_optional_color(ui, "Island Color", &mut config.island_color);

    let new_value = add_label_with_toggle_switch(ui, 0, "Hide Meshes", config.hide_meshes);
    if new_value != config.hide_meshes {
        config.hide_meshes = new_value;
    }

    // Sync AvianDebugConfig → PhysicsGizmos
    let (_, gizmos) = settings_param
        .avian
        .gizmo_store
        .config_mut::<PhysicsGizmos>();
    *gizmos = config.clone().into();
}

fn show_avian_optional_color(ui: &mut egui::Ui, label: &str, opt_color: &mut OptionalColor) {
    let original = egui::Color32::from_rgba_unmultiplied(
        (opt_color.color[0] * 255.) as u8,
        (opt_color.color[1] * 255.) as u8,
        (opt_color.color[2] * 255.) as u8,
        (opt_color.color[3] * 255.) as u8,
    );
    let mut color32 = original;

    ui.label(label);
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let mut is_on = opt_color.enabled;
        ui.add(crate::ui::widgets::toggle_switch::toggle(&mut is_on));
        ui.color_edit_button_srgba(&mut color32);

        opt_color.enabled = is_on;
        if color32 != original {
            let c = Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a()).to_srgba();
            opt_color.color = [c.red, c.green, c.blue, c.alpha];
        }
    });
    ui.end_row();
}

fn show_keybinds_settings(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    egui::Grid::new("keybinds_grid")
        .num_columns(2)
        .show(ui, |ui| {
            ui.heading("Camera");
            ui.end_row();
            show_camera_keybinds(ui, settings_param);

            add_major_grid_separator(ui);

            ui.heading("Brush");
            ui.end_row();
            show_brush_keybinds(ui, settings_param);

            add_major_grid_separator(ui);

            ui.heading("Quick Actions");
            ui.end_row();
            show_quick_actions_keybinds(ui, settings_param);

            add_major_grid_separator(ui);

            ui.heading("Console");
            ui.end_row();
            show_console_keybinds(ui, settings_param);

            add_major_grid_separator(ui);

            ui.heading("General");
            ui.end_row();
            show_general_keybinds(ui, settings_param);
        });
}

fn show_keybind_row(
    ui: &mut egui::Ui,
    label: &str,
    binding_id: &'static str,
    current: &InputButton,
    listening: &Option<Res<ListeningForKeybind>>,
) -> bool {
    ui.label(label);
    let mut clicked = false;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        let is_listening = listening
            .as_ref()
            .is_some_and(|l| l.binding_id == binding_id);
        let text = if is_listening {
            "Press a key...".to_string()
        } else {
            current.to_string()
        };
        if ui.button(text).clicked() {
            clicked = true;
        }
    });
    ui.end_row();
    clicked
}

fn start_listening(settings_param: &mut SettingsParam, binding_id: &'static str) {
    settings_param
        .commands
        .insert_resource(ListeningForKeybind { binding_id });
    settings_param
        .keybinds
        .next_listening_state
        .set(KeybindsListeningState::Listening);
}

fn show_camera_keybinds(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let keys = settings_param.keybinds.camera_keys.clone();
    if show_keybind_row(
        ui,
        "Pan Up",
        "camera.pan_up",
        &keys.pan_camera_up,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "camera.pan_up");
    }
    if show_keybind_row(
        ui,
        "Pan Down",
        "camera.pan_down",
        &keys.pan_camera_down,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "camera.pan_down");
    }
    if show_keybind_row(
        ui,
        "Pan Left",
        "camera.pan_left",
        &keys.pan_camera_left,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "camera.pan_left");
    }
    if show_keybind_row(
        ui,
        "Pan Right",
        "camera.pan_right",
        &keys.pan_camera_right,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "camera.pan_right");
    }
}

fn show_brush_keybinds(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let keys = settings_param.keybinds.brush_keys.clone();
    if show_keybind_row(
        ui,
        "Draw",
        "brush.draw",
        &keys.draw,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "brush.draw");
    }
    if show_keybind_row(
        ui,
        "Toggle Mode",
        "brush.toggle_mode",
        &keys.toggle_brush_mode,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "brush.toggle_mode");
    }
}

fn show_quick_actions_keybinds(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let keys = settings_param.keybinds.ui_keys.quick_actions.clone();
    if show_keybind_row(
        ui,
        "Toggle UI",
        "quick_actions.toggle_ui",
        &keys.toggle_ui,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "quick_actions.toggle_ui");
    }
    if show_keybind_row(
        ui,
        "Toggle Map Overlay",
        "quick_actions.toggle_map_overlay",
        &keys.toggle_map_overlay,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "quick_actions.toggle_map_overlay");
    }
    if show_keybind_row(
        ui,
        "Toggle Dirty Chunks",
        "quick_actions.toggle_dirty_chunks",
        &keys.toggle_dirty_chunks_overlay,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "quick_actions.toggle_dirty_chunks");
    }
    if show_keybind_row(
        ui,
        "Toggle Simulation",
        "quick_actions.toggle_simulation_run",
        &keys.toggle_simulation_run,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "quick_actions.toggle_simulation_run");
    }
    if show_keybind_row(
        ui,
        "Simulation Step",
        "quick_actions.toggle_simulation_step",
        &keys.toggle_simulation_step,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "quick_actions.toggle_simulation_step");
    }
    if show_keybind_row(
        ui,
        "Sample Particle",
        "quick_actions.sample_hovered_particle",
        &keys.sample_hovered_particle,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "quick_actions.sample_hovered_particle");
    }
}

fn show_console_keybinds(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let keys = settings_param.keybinds.ui_keys.console.clone();
    if show_keybind_row(
        ui,
        "Toggle Info Area",
        "console.toggle_information_area",
        &keys.toggle_information_area,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "console.toggle_information_area");
    }
}

fn show_general_keybinds(ui: &mut egui::Ui, settings_param: &mut SettingsParam) {
    let keys = settings_param.keybinds.ui_keys.general.clone();
    if show_keybind_row(
        ui,
        "Hold Canvas Edit",
        "general.hold_canvas_mode_edit",
        &keys.hold_canvas_mode_edit,
        &settings_param.keybinds.listening,
    ) {
        start_listening(settings_param, "general.hold_canvas_mode_edit");
    }
}

pub fn listen_for_keybind(
    mut commands: Commands,
    listening: Res<ListeningForKeybind>,
    key_input: Res<ButtonInput<KeyCode>>,
    mouse_input: Res<ButtonInput<MouseButton>>,
    mut next_listening_state: ResMut<NextState<KeybindsListeningState>>,
    mut camera_keys: ResMut<CameraKeyBindings>,
    mut brush_keys: ResMut<BrushKeyBindings>,
    mut ui_keys: ResMut<UiKeyBindings>,
    mut camera_input_map: Query<&mut InputMap<CameraAction>>,
    mut brush_input_map: Query<&mut InputMap<BrushAction>>,
    mut quick_action_input_map: Query<&mut InputMap<QuickAction>>,
    mut console_input_map: Query<&mut InputMap<ConsoleAction>>,
    mut canvas_input_map: Query<&mut InputMap<CanvasStateActions>>,
) {
    // Check for Escape to cancel
    if key_input.just_pressed(KeyCode::Escape) {
        commands.remove_resource::<ListeningForKeybind>();
        next_listening_state.set(KeybindsListeningState::Deafened);
        return;
    }

    // Detect new input
    let new_button: Option<InputButton> = if let Some(&key) = key_input.get_just_pressed().next() {
        Some(InputButton::Key(key))
    } else if let Some(&btn) = mouse_input.get_just_pressed().next() {
        Some(InputButton::Mouse(btn))
    } else {
        None
    };

    let Some(new_button) = new_button else {
        return;
    };

    // Apply the new binding based on binding_id
    match listening.binding_id {
        "camera.pan_up" => camera_keys.pan_camera_up = new_button,
        "camera.pan_down" => camera_keys.pan_camera_down = new_button,
        "camera.pan_left" => camera_keys.pan_camera_left = new_button,
        "camera.pan_right" => camera_keys.pan_camera_right = new_button,
        "brush.draw" => brush_keys.draw = new_button,
        "brush.toggle_mode" => brush_keys.toggle_brush_mode = new_button,
        "quick_actions.toggle_ui" => ui_keys.quick_actions.toggle_ui = new_button,
        "quick_actions.toggle_map_overlay" => ui_keys.quick_actions.toggle_map_overlay = new_button,
        "quick_actions.toggle_dirty_chunks" => {
            ui_keys.quick_actions.toggle_dirty_chunks_overlay = new_button
        }
        "quick_actions.toggle_simulation_run" => {
            ui_keys.quick_actions.toggle_simulation_run = new_button
        }
        "quick_actions.toggle_simulation_step" => {
            ui_keys.quick_actions.toggle_simulation_step = new_button
        }
        "quick_actions.sample_hovered_particle" => {
            ui_keys.quick_actions.sample_hovered_particle = new_button
        }
        "console.toggle_information_area" => ui_keys.console.toggle_information_area = new_button,
        "general.hold_canvas_mode_edit" => ui_keys.general.hold_canvas_mode_edit = new_button,
        _ => {}
    }

    // Rebuild the affected InputMap
    match listening.binding_id {
        id if id.starts_with("camera.") => {
            if let Ok(mut map) = camera_input_map.single_mut() {
                *map = InputMap::default().with_axis(CameraAction::Zoom, MouseScrollAxis::Y);
                camera_keys
                    .pan_camera_up
                    .insert_into_input_map(&mut map, CameraAction::PanUp);
                camera_keys
                    .pan_camera_left
                    .insert_into_input_map(&mut map, CameraAction::PanLeft);
                camera_keys
                    .pan_camera_down
                    .insert_into_input_map(&mut map, CameraAction::PanDown);
                camera_keys
                    .pan_camera_right
                    .insert_into_input_map(&mut map, CameraAction::PanRight);
            }
        }
        id if id.starts_with("brush.") => {
            if let Ok(mut map) = brush_input_map.single_mut() {
                *map = InputMap::default().with_axis(BrushAction::ChangeSize, MouseScrollAxis::Y);
                brush_keys
                    .toggle_brush_mode
                    .insert_into_input_map(&mut map, BrushAction::ToggleMode);
                brush_keys
                    .draw
                    .insert_into_input_map(&mut map, BrushAction::Draw);
            }
        }
        id if id.starts_with("quick_actions.") => {
            if let Ok(mut map) = quick_action_input_map.single_mut() {
                *map = InputMap::default();
                let qa = &ui_keys.quick_actions;
                qa.toggle_ui
                    .insert_into_input_map(&mut map, QuickAction::ToggleUi);
                qa.toggle_map_overlay
                    .insert_into_input_map(&mut map, QuickAction::ToggleMapOverlay);
                qa.toggle_dirty_chunks_overlay
                    .insert_into_input_map(&mut map, QuickAction::ToggleDirtyChunksOverlay);
                qa.toggle_simulation_run
                    .insert_into_input_map(&mut map, QuickAction::ToggleSimulationRun);
                qa.toggle_simulation_step
                    .insert_into_input_map(&mut map, QuickAction::ToggleSimulationStep);
                qa.sample_hovered_particle
                    .insert_into_input_map(&mut map, QuickAction::SampleHoveredParticle);
            }
        }
        id if id.starts_with("console.") => {
            if let Ok(mut map) = console_input_map.single_mut() {
                *map = InputMap::default();
                let c = &ui_keys.console;
                c.toggle_information_area
                    .insert_into_input_map(&mut map, ConsoleAction::ToggleInformationArea);
                c.submit_input_text
                    .insert_into_input_map(&mut map, ConsoleAction::SubmitInputText);
            }
        }
        id if id.starts_with("general.") => {
            if let Ok(mut map) = canvas_input_map.single_mut() {
                *map = InputMap::default();
                ui_keys
                    .general
                    .hold_canvas_mode_edit
                    .insert_into_input_map(&mut map, CanvasStateActions::Modify);
            }
        }
        _ => {}
    }

    commands.remove_resource::<ListeningForKeybind>();
    next_listening_state.set(KeybindsListeningState::Deafened);
}
