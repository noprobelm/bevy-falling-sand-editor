use crate::brush::{
    Brush, BrushColor, BrushForceColor, BrushMode, BrushOverwriteExisting, BrushSize, BrushType,
    MaxBrushSize,
};
use crate::ui::console::core::commands::exit::ExitApplicationEvent;
use avian2d::prelude::{ContactGizmoScale, PhysicsGizmos};
use bevy::prelude::*;
use bevy_egui::egui;
use bevy_falling_sand::debug::{
    ChunkBorderColor, DebugDirtyRects, DebugParticleCount, DebugParticleMap, DirtyRectColor,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

pub struct SettingsPlugin;

impl Plugin for SettingsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SettingsState>()
            .init_resource::<SettingsPersistenceTimer>()
            .add_systems(Startup, load_persistent_settings)
            .add_systems(Update, save_persistent_settings_periodic)
            .add_observer(save_settings_on_exit);
    }
}

// Persistence

fn get_config_dir() -> PathBuf {
    let config_dir = PathBuf::from(std::env::var("HOME").unwrap_or_else(|_| ".".to_string()))
        .join(".config")
        .join("bevy_falling_sand");
    if !config_dir.exists() {
        if let Err(e) = fs::create_dir_all(&config_dir) {
            warn!("Failed to create config directory {:?}: {}", config_dir, e);
        }
    }
    config_dir
}

fn get_settings_path() -> PathBuf {
    get_config_dir().join("settings.ron")
}

#[derive(Resource)]
pub struct SettingsPersistenceTimer {
    timer: Timer,
}

impl Default for SettingsPersistenceTimer {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(5.0, TimerMode::Repeating),
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersistentSettings {
    // Brush settings
    pub max_brush_size: usize,
    pub brush_overwrite_existing: bool,
    pub brush_type: PersistentBrushType,
    pub brush_mode: PersistentBrushMode,
    #[serde(default)]
    pub force_color: PersistentBrushForceColor,

    // Physics gizmo settings
    pub physics_gizmos: PersistentPhysicsGizmos,

    // Debug gizmo settings
    #[serde(default)]
    pub debug_gizmos: PersistentDebugGizmos,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersistentBrushForceColor {
    pub enabled: bool,
    pub colors: Vec<[f32; 4]>,
    pub selected_index: usize,
}

impl Default for PersistentBrushForceColor {
    fn default() -> Self {
        Self {
            enabled: false,
            colors: vec![[1.0, 1.0, 1.0, 1.0]],
            selected_index: 0,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum PersistentBrushType {
    Line,
    #[default]
    Circle,
    Cursor,
}

impl From<BrushType> for PersistentBrushType {
    fn from(bt: BrushType) -> Self {
        match bt {
            BrushType::Line => PersistentBrushType::Line,
            BrushType::Circle => PersistentBrushType::Circle,
            BrushType::Cursor => PersistentBrushType::Cursor,
        }
    }
}

impl From<PersistentBrushType> for BrushType {
    fn from(pbt: PersistentBrushType) -> Self {
        match pbt {
            PersistentBrushType::Line => BrushType::Line,
            PersistentBrushType::Circle => BrushType::Circle,
            PersistentBrushType::Cursor => BrushType::Cursor,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, Default)]
pub enum PersistentBrushMode {
    #[default]
    Spawn,
    Despawn,
}

impl From<BrushMode> for PersistentBrushMode {
    fn from(bm: BrushMode) -> Self {
        match bm {
            BrushMode::Spawn => PersistentBrushMode::Spawn,
            BrushMode::Despawn => PersistentBrushMode::Despawn,
        }
    }
}

impl From<PersistentBrushMode> for BrushMode {
    fn from(pbm: PersistentBrushMode) -> Self {
        match pbm {
            PersistentBrushMode::Spawn => BrushMode::Spawn,
            PersistentBrushMode::Despawn => BrushMode::Despawn,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersistentPhysicsGizmos {
    pub hide_meshes: bool,
    pub collider_color: Option<[f32; 4]>,
    pub aabb_color: Option<[f32; 4]>,
    pub contact_point_color: Option<[f32; 4]>,
    pub contact_normal_color: Option<[f32; 4]>,
    pub joint_anchor_color: Option<[f32; 4]>,
    pub joint_separation_color: Option<[f32; 4]>,
    pub raycast_color: Option<[f32; 4]>,
    pub raycast_point_color: Option<[f32; 4]>,
    pub raycast_normal_color: Option<[f32; 4]>,
    pub shapecast_color: Option<[f32; 4]>,
    pub shapecast_shape_color: Option<[f32; 4]>,
    pub shapecast_point_color: Option<[f32; 4]>,
    pub shapecast_normal_color: Option<[f32; 4]>,
    pub axis_lengths: Option<[f32; 2]>,
    pub contact_normal_scale: PersistentContactGizmoScale,
    pub sleeping_color_multiplier: Option<[f32; 4]>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum PersistentContactGizmoScale {
    Constant(f32),
    Scaled(f32),
}

impl Default for PersistentContactGizmoScale {
    fn default() -> Self {
        PersistentContactGizmoScale::Constant(1.0)
    }
}

impl Default for PersistentPhysicsGizmos {
    fn default() -> Self {
        Self {
            hide_meshes: false,
            collider_color: None,
            aabb_color: None,
            contact_point_color: None,
            contact_normal_color: None,
            joint_anchor_color: None,
            joint_separation_color: None,
            raycast_color: None,
            raycast_point_color: None,
            raycast_normal_color: None,
            shapecast_color: None,
            shapecast_shape_color: None,
            shapecast_point_color: None,
            shapecast_normal_color: None,
            axis_lengths: None,
            contact_normal_scale: PersistentContactGizmoScale::default(),
            sleeping_color_multiplier: None,
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct PersistentDebugGizmos {
    pub show_particle_count: bool,
    pub show_particle_map: bool,
    pub show_dirty_rects: bool,
    pub chunk_border_color: [f32; 4],
    pub dirty_rect_color: [f32; 4],
}

impl Default for PersistentDebugGizmos {
    fn default() -> Self {
        Self {
            show_particle_count: false,
            show_particle_map: false,
            show_dirty_rects: false,
            chunk_border_color: color_to_array(ChunkBorderColor::default().0),
            dirty_rect_color: color_to_array(DirtyRectColor::default().0),
        }
    }
}

impl Default for PersistentSettings {
    fn default() -> Self {
        Self {
            max_brush_size: 50,
            brush_overwrite_existing: false,
            brush_type: PersistentBrushType::default(),
            brush_mode: PersistentBrushMode::default(),
            force_color: PersistentBrushForceColor::default(),
            physics_gizmos: PersistentPhysicsGizmos::default(),
            debug_gizmos: PersistentDebugGizmos::default(),
        }
    }
}

fn color_to_array(color: Color) -> [f32; 4] {
    let srgba = color.to_srgba();
    [srgba.red, srgba.green, srgba.blue, srgba.alpha]
}

fn array_to_color(arr: [f32; 4]) -> Color {
    Color::srgba(arr[0], arr[1], arr[2], arr[3])
}

fn load_persistent_settings(
    mut commands: Commands,
    mut max_brush_size: ResMut<MaxBrushSize>,
    mut brush_overwrite: ResMut<BrushOverwriteExisting>,
    mut brush_force_color: ResMut<BrushForceColor>,
    mut brush_type_next: ResMut<NextState<BrushType>>,
    mut brush_mode_next: ResMut<NextState<BrushMode>>,
    mut gizmo_store: ResMut<GizmoConfigStore>,
    mut chunk_border_color: ResMut<ChunkBorderColor>,
    mut dirty_rect_color: ResMut<DirtyRectColor>,
) {
    let path = get_settings_path();
    if !path.exists() {
        info!("No settings file found at {:?}, using defaults", path);
        return;
    }

    let contents = match fs::read_to_string(&path) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to read settings file {:?}: {}", path, e);
            return;
        }
    };

    let settings: PersistentSettings = match ron::from_str(&contents) {
        Ok(s) => s,
        Err(e) => {
            warn!("Failed to parse settings file {:?}: {}", path, e);
            return;
        }
    };

    // Apply brush settings
    max_brush_size.0 = settings.max_brush_size;
    brush_overwrite.0 = settings.brush_overwrite_existing;
    brush_type_next.set(settings.brush_type.into());
    brush_mode_next.set(settings.brush_mode.into());

    // Apply force color settings
    brush_force_color.enabled = settings.force_color.enabled;
    brush_force_color.colors = settings
        .force_color
        .colors
        .iter()
        .map(|c| array_to_color(*c))
        .collect();
    brush_force_color.selected_index = settings
        .force_color
        .selected_index
        .min(brush_force_color.colors.len().saturating_sub(1));

    // Apply physics gizmo settings
    let (_, physics_gizmos) = gizmo_store.config_mut::<PhysicsGizmos>();
    let pg = &settings.physics_gizmos;

    physics_gizmos.hide_meshes = pg.hide_meshes;
    physics_gizmos.collider_color = pg.collider_color.map(array_to_color);
    physics_gizmos.aabb_color = pg.aabb_color.map(array_to_color);
    physics_gizmos.contact_point_color = pg.contact_point_color.map(array_to_color);
    physics_gizmos.contact_normal_color = pg.contact_normal_color.map(array_to_color);
    physics_gizmos.joint_anchor_color = pg.joint_anchor_color.map(array_to_color);
    physics_gizmos.joint_separation_color = pg.joint_separation_color.map(array_to_color);
    physics_gizmos.raycast_color = pg.raycast_color.map(array_to_color);
    physics_gizmos.raycast_point_color = pg.raycast_point_color.map(array_to_color);
    physics_gizmos.raycast_normal_color = pg.raycast_normal_color.map(array_to_color);
    physics_gizmos.shapecast_color = pg.shapecast_color.map(array_to_color);
    physics_gizmos.shapecast_shape_color = pg.shapecast_shape_color.map(array_to_color);
    physics_gizmos.shapecast_point_color = pg.shapecast_point_color.map(array_to_color);
    physics_gizmos.shapecast_normal_color = pg.shapecast_normal_color.map(array_to_color);
    physics_gizmos.axis_lengths = pg.axis_lengths.map(|a| Vec2::new(a[0], a[1]));
    physics_gizmos.contact_normal_scale = match pg.contact_normal_scale {
        PersistentContactGizmoScale::Constant(v) => ContactGizmoScale::Constant(v),
        PersistentContactGizmoScale::Scaled(v) => ContactGizmoScale::Scaled(v),
    };
    physics_gizmos.sleeping_color_multiplier = pg.sleeping_color_multiplier;

    // Apply debug gizmo settings
    let dg = &settings.debug_gizmos;
    if dg.show_particle_count {
        commands.insert_resource(DebugParticleCount);
    } else {
        commands.remove_resource::<DebugParticleCount>();
    }
    if dg.show_particle_map {
        commands.insert_resource(DebugParticleMap);
    } else {
        commands.remove_resource::<DebugParticleMap>();
    }
    if dg.show_dirty_rects {
        commands.insert_resource(DebugDirtyRects);
    } else {
        commands.remove_resource::<DebugDirtyRects>();
    }
    chunk_border_color.0 = array_to_color(dg.chunk_border_color);
    dirty_rect_color.0 = array_to_color(dg.dirty_rect_color);

    info!("Loaded settings from {:?}", path);
}

fn build_persistent_settings(
    max_brush_size: &MaxBrushSize,
    brush_overwrite: &BrushOverwriteExisting,
    brush_force_color: &BrushForceColor,
    brush_type: &State<BrushType>,
    brush_mode: &State<BrushMode>,
    gizmo_store: &GizmoConfigStore,
    has_debug_particle_count: bool,
    has_debug_particle_map: bool,
    has_debug_dirty_rects: bool,
    chunk_border_color: &ChunkBorderColor,
    dirty_rect_color: &DirtyRectColor,
) -> PersistentSettings {
    let (_, physics_gizmos) = gizmo_store.config::<PhysicsGizmos>();

    PersistentSettings {
        max_brush_size: max_brush_size.0,
        brush_overwrite_existing: brush_overwrite.0,
        brush_type: (*brush_type.get()).into(),
        brush_mode: (*brush_mode.get()).into(),
        force_color: PersistentBrushForceColor {
            enabled: brush_force_color.enabled,
            colors: brush_force_color
                .colors
                .iter()
                .map(|c| color_to_array(*c))
                .collect(),
            selected_index: brush_force_color.selected_index,
        },
        physics_gizmos: PersistentPhysicsGizmos {
            hide_meshes: physics_gizmos.hide_meshes,
            collider_color: physics_gizmos.collider_color.map(color_to_array),
            aabb_color: physics_gizmos.aabb_color.map(color_to_array),
            contact_point_color: physics_gizmos.contact_point_color.map(color_to_array),
            contact_normal_color: physics_gizmos.contact_normal_color.map(color_to_array),
            joint_anchor_color: physics_gizmos.joint_anchor_color.map(color_to_array),
            joint_separation_color: physics_gizmos.joint_separation_color.map(color_to_array),
            raycast_color: physics_gizmos.raycast_color.map(color_to_array),
            raycast_point_color: physics_gizmos.raycast_point_color.map(color_to_array),
            raycast_normal_color: physics_gizmos.raycast_normal_color.map(color_to_array),
            shapecast_color: physics_gizmos.shapecast_color.map(color_to_array),
            shapecast_shape_color: physics_gizmos.shapecast_shape_color.map(color_to_array),
            shapecast_point_color: physics_gizmos.shapecast_point_color.map(color_to_array),
            shapecast_normal_color: physics_gizmos.shapecast_normal_color.map(color_to_array),
            axis_lengths: physics_gizmos.axis_lengths.map(|v| [v.x, v.y]),
            contact_normal_scale: match physics_gizmos.contact_normal_scale {
                ContactGizmoScale::Constant(v) => PersistentContactGizmoScale::Constant(v),
                ContactGizmoScale::Scaled(v) => PersistentContactGizmoScale::Scaled(v),
            },
            sleeping_color_multiplier: physics_gizmos.sleeping_color_multiplier,
        },
        debug_gizmos: PersistentDebugGizmos {
            show_particle_count: has_debug_particle_count,
            show_particle_map: has_debug_particle_map,
            show_dirty_rects: has_debug_dirty_rects,
            chunk_border_color: color_to_array(chunk_border_color.0),
            dirty_rect_color: color_to_array(dirty_rect_color.0),
        },
    }
}

fn write_settings_to_disk(settings: &PersistentSettings) {
    let contents = match ron::ser::to_string_pretty(settings, ron::ser::PrettyConfig::default()) {
        Ok(c) => c,
        Err(e) => {
            warn!("Failed to serialize settings: {}", e);
            return;
        }
    };

    let path = get_settings_path();
    if let Err(e) = fs::write(&path, contents) {
        warn!("Failed to save settings to {:?}: {}", path, e);
    }
}

fn save_persistent_settings_periodic(
    max_brush_size: Res<MaxBrushSize>,
    brush_overwrite: Res<BrushOverwriteExisting>,
    brush_force_color: Res<BrushForceColor>,
    brush_type: Res<State<BrushType>>,
    brush_mode: Res<State<BrushMode>>,
    gizmo_store: Res<GizmoConfigStore>,
    debug_particle_count: Option<Res<DebugParticleCount>>,
    debug_particle_map: Option<Res<DebugParticleMap>>,
    debug_dirty_rects: Option<Res<DebugDirtyRects>>,
    chunk_border_color: Res<ChunkBorderColor>,
    dirty_rect_color: Res<DirtyRectColor>,
    mut timer: ResMut<SettingsPersistenceTimer>,
    time: Res<Time>,
) {
    timer.timer.tick(time.delta());
    if !timer.timer.just_finished() {
        return;
    }

    let settings = build_persistent_settings(
        &max_brush_size,
        &brush_overwrite,
        &brush_force_color,
        &brush_type,
        &brush_mode,
        &gizmo_store,
        debug_particle_count.is_some(),
        debug_particle_map.is_some(),
        debug_dirty_rects.is_some(),
        &chunk_border_color,
        &dirty_rect_color,
    );
    write_settings_to_disk(&settings);
}

fn save_settings_on_exit(
    _trigger: On<ExitApplicationEvent>,
    max_brush_size: Res<MaxBrushSize>,
    brush_overwrite: Res<BrushOverwriteExisting>,
    brush_force_color: Res<BrushForceColor>,
    brush_type: Res<State<BrushType>>,
    brush_mode: Res<State<BrushMode>>,
    gizmo_store: Res<GizmoConfigStore>,
    debug_particle_count: Option<Res<DebugParticleCount>>,
    debug_particle_map: Option<Res<DebugParticleMap>>,
    debug_dirty_rects: Option<Res<DebugDirtyRects>>,
    chunk_border_color: Res<ChunkBorderColor>,
    dirty_rect_color: Res<DirtyRectColor>,
) {
    let settings = build_persistent_settings(
        &max_brush_size,
        &brush_overwrite,
        &brush_force_color,
        &brush_type,
        &brush_mode,
        &gizmo_store,
        debug_particle_count.is_some(),
        debug_particle_map.is_some(),
        debug_dirty_rects.is_some(),
        &chunk_border_color,
        &dirty_rect_color,
    );
    write_settings_to_disk(&settings);
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
pub enum SettingsCategory {
    #[default]
    Brush,
    Debug,
}

#[derive(Resource, Default)]
pub struct SettingsState {
    pub show_window: bool,
    pub selected_category: SettingsCategory,
    color_cache: HashMap<String, Color>,
    axis_lengths_cache: Option<Vec2>,
    sleeping_multiplier_cache: Option<[f32; 4]>,
}

impl SettingsState {
    pub fn open(&mut self) {
        self.show_window = true;
    }

    pub fn close(&mut self) {
        self.show_window = false;
    }

    fn cache_color(&mut self, key: &str, color: Color) {
        self.color_cache.insert(key.to_string(), color);
    }

    fn get_cached_color(&self, key: &str) -> Color {
        self.color_cache
            .get(key)
            .copied()
            .unwrap_or_else(|| default_gizmo_color(key))
    }
}

fn default_gizmo_color(key: &str) -> Color {
    match key {
        "Collider" => bevy::color::palettes::css::ORANGE.into(),
        "AABB" => Color::srgb(0.8, 0.8, 0.8),
        "Contact Point" => bevy::color::palettes::css::LIGHT_CYAN.into(),
        "Contact Normal" => bevy::color::palettes::css::RED.into(),
        "Joint Anchor" => bevy::color::palettes::css::PINK.into(),
        "Joint Separation" => bevy::color::palettes::css::RED.into(),
        "Raycast" => bevy::color::palettes::css::RED.into(),
        "Raycast Point" => bevy::color::palettes::css::YELLOW.into(),
        "Raycast Normal" => bevy::color::palettes::css::PINK.into(),
        "Shapecast" => bevy::color::palettes::css::RED.into(),
        "Shapecast Shape" => Color::srgb(0.4, 0.6, 1.0),
        "Shapecast Point" => bevy::color::palettes::css::YELLOW.into(),
        "Shapecast Normal" => bevy::color::palettes::css::PINK.into(),
        _ => Color::WHITE,
    }
}

pub struct SettingsWindow;

pub struct BrushSettingsParams<'w, 's> {
    pub max_brush_size: ResMut<'w, MaxBrushSize>,
    pub brush_query: Query<'w, 's, (&'static mut BrushSize, &'static mut BrushColor), With<Brush>>,
    pub brush_type: Res<'w, State<BrushType>>,
    pub brush_type_next: ResMut<'w, NextState<BrushType>>,
    pub brush_mode: Res<'w, State<BrushMode>>,
    pub brush_mode_next: ResMut<'w, NextState<BrushMode>>,
    pub brush_overwrite: ResMut<'w, BrushOverwriteExisting>,
}

pub struct DebugSettingsParams<'w> {
    pub commands: Commands<'w, 'w>,
    pub debug_particle_count: Option<Res<'w, DebugParticleCount>>,
    pub debug_particle_map: Option<Res<'w, DebugParticleMap>>,
    pub debug_dirty_rects: Option<Res<'w, DebugDirtyRects>>,
    pub chunk_border_color: ResMut<'w, ChunkBorderColor>,
    pub dirty_rect_color: ResMut<'w, DirtyRectColor>,
}

impl SettingsWindow {
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        state: &mut ResMut<SettingsState>,
        gizmo_store: &mut ResMut<GizmoConfigStore>,
        brush_params: &mut BrushSettingsParams,
        debug_params: &mut DebugSettingsParams,
    ) {
        if !state.show_window {
            return;
        }

        let mut should_close = false;

        egui::Window::new("Settings")
            .collapsible(false)
            .resizable(true)
            .default_size([500.0, 600.0])
            .show(ui.ctx(), |ui| {
                ui.horizontal(|ui| {
                    if ui
                        .selectable_label(
                            state.selected_category == SettingsCategory::Brush,
                            "Brush",
                        )
                        .clicked()
                    {
                        state.selected_category = SettingsCategory::Brush;
                    }
                    if ui
                        .selectable_label(
                            state.selected_category == SettingsCategory::Debug,
                            "Debug",
                        )
                        .clicked()
                    {
                        state.selected_category = SettingsCategory::Debug;
                    }
                });

                ui.separator();

                egui::ScrollArea::vertical().show(ui, |ui| {
                    match state.selected_category {
                        SettingsCategory::Brush => {
                            render_brush_settings(ui, state, gizmo_store, brush_params);
                        }
                        SettingsCategory::Debug => {
                            render_debug_settings(ui, state, gizmo_store, debug_params);
                        }
                    }

                    ui.add_space(16.0);
                    ui.separator();

                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            should_close = true;
                        }
                    });
                });
            });

        if should_close {
            state.close();
        }
    }
}

fn render_brush_settings(
    ui: &mut egui::Ui,
    _state: &mut ResMut<SettingsState>,
    _gizmo_store: &mut ResMut<GizmoConfigStore>,
    brush_params: &mut BrushSettingsParams,
) {
    ui.heading("Brush");
    ui.separator();

    if let Ok((mut brush_size, mut brush_color)) = brush_params.brush_query.single_mut() {
        ui.horizontal(|ui| {
            ui.label("Size:");
            let max = brush_params.max_brush_size.0;
            if ui
                .add(
                    egui::DragValue::new(&mut brush_size.0)
                        .speed(1)
                        .range(1..=max),
                )
                .changed()
            {
                brush_size.0 = brush_size.0.clamp(1, max);
            }
        });

        ui.horizontal(|ui| {
            ui.label("Color:");
            let srgba = brush_color.0.to_srgba();
            let mut color32 = egui::Color32::from_rgba_unmultiplied(
                (srgba.red * 255.0) as u8,
                (srgba.green * 255.0) as u8,
                (srgba.blue * 255.0) as u8,
                (srgba.alpha * 255.0) as u8,
            );

            if ui.color_edit_button_srgba(&mut color32).changed() {
                brush_color.0 = Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
            }
        });
    }

    ui.horizontal(|ui| {
        ui.label("Max Size:");
        ui.add(
            egui::DragValue::new(&mut brush_params.max_brush_size.0)
                .speed(1)
                .range(1..=200),
        );
    });

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Type:");
        let current_type = *brush_params.brush_type.get();
        egui::ComboBox::from_id_salt("brush_type")
            .selected_text(format!("{:?}", current_type))
            .show_ui(ui, |ui| {
                if ui
                    .selectable_value(&mut current_type.clone(), BrushType::Circle, "Circle")
                    .clicked()
                {
                    brush_params.brush_type_next.set(BrushType::Circle);
                }
                if ui
                    .selectable_value(&mut current_type.clone(), BrushType::Line, "Line")
                    .clicked()
                {
                    brush_params.brush_type_next.set(BrushType::Line);
                }
                if ui
                    .selectable_value(&mut current_type.clone(), BrushType::Cursor, "Cursor")
                    .clicked()
                {
                    brush_params.brush_type_next.set(BrushType::Cursor);
                }
            });
    });

    ui.add_space(8.0);

    ui.horizontal(|ui| {
        ui.label("Mode:");
        let current_mode = *brush_params.brush_mode.get();
        egui::ComboBox::from_id_salt("brush_mode")
            .selected_text(format!("{:?}", current_mode))
            .show_ui(ui, |ui| {
                if ui
                    .selectable_value(&mut current_mode.clone(), BrushMode::Spawn, "Spawn")
                    .clicked()
                {
                    brush_params.brush_mode_next.set(BrushMode::Spawn);
                }
                if ui
                    .selectable_value(&mut current_mode.clone(), BrushMode::Despawn, "Despawn")
                    .clicked()
                {
                    brush_params.brush_mode_next.set(BrushMode::Despawn);
                }
            });
    });

    ui.add_space(8.0);

    ui.checkbox(&mut brush_params.brush_overwrite.0, "Overwrite Existing");

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    if ui.button("Reset Defaults").clicked() {
        brush_params.max_brush_size.0 = 50;
        brush_params.brush_overwrite.0 = false;
        brush_params.brush_type_next.set(BrushType::Circle);
        brush_params.brush_mode_next.set(BrushMode::Spawn);
    }
}

fn render_color_option(
    ui: &mut egui::Ui,
    state: &mut ResMut<SettingsState>,
    label: &str,
    color_opt: &mut Option<Color>,
) {
    ui.horizontal(|ui| {
        let mut enabled = color_opt.is_some();
        if ui.checkbox(&mut enabled, label).changed() {
            if enabled {
                *color_opt = Some(state.get_cached_color(label));
            } else {
                if let Some(color) = *color_opt {
                    state.cache_color(label, color);
                }
                *color_opt = None;
            }
        }

        if let Some(color) = color_opt {
            let srgba = color.to_srgba();
            let mut color32 = egui::Color32::from_rgba_unmultiplied(
                (srgba.red * 255.0) as u8,
                (srgba.green * 255.0) as u8,
                (srgba.blue * 255.0) as u8,
                (srgba.alpha * 255.0) as u8,
            );

            if ui.color_edit_button_srgba(&mut color32).changed() {
                *color = Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
            }
        }
    });
}

fn render_axis_lengths(
    ui: &mut egui::Ui,
    state: &mut ResMut<SettingsState>,
    axis_lengths: &mut Option<Vec2>,
) {
    ui.horizontal(|ui| {
        let mut enabled = axis_lengths.is_some();
        if ui.checkbox(&mut enabled, "Axis Lengths").changed() {
            if enabled {
                *axis_lengths = Some(state.axis_lengths_cache.unwrap_or(Vec2::new(5.0, 5.0)));
            } else {
                state.axis_lengths_cache = *axis_lengths;
                *axis_lengths = None;
            }
        }

        if let Some(lengths) = axis_lengths {
            ui.label("X:");
            ui.add(
                egui::DragValue::new(&mut lengths.x)
                    .speed(0.1)
                    .range(0.0..=100.0),
            );
            ui.label("Y:");
            ui.add(
                egui::DragValue::new(&mut lengths.y)
                    .speed(0.1)
                    .range(0.0..=100.0),
            );
        }
    });
}

fn render_contact_normal_scale(ui: &mut egui::Ui, scale: &mut ContactGizmoScale) {
    ui.horizontal(|ui| {
        ui.label("Contact Normal Scale:");

        let is_constant = matches!(scale, ContactGizmoScale::Constant(_));
        let mut selected = if is_constant { 0 } else { 1 };

        egui::ComboBox::from_id_salt("contact_normal_scale")
            .selected_text(if is_constant { "Constant" } else { "Scaled" })
            .show_ui(ui, |ui| {
                if ui.selectable_value(&mut selected, 0, "Constant").changed() {
                    *scale = ContactGizmoScale::Constant(1.0);
                }
                if ui.selectable_value(&mut selected, 1, "Scaled").changed() {
                    *scale = ContactGizmoScale::Scaled(0.01);
                }
            });

        match scale {
            ContactGizmoScale::Constant(val) => {
                ui.add(egui::DragValue::new(val).speed(0.1).range(0.0..=100.0));
            }
            ContactGizmoScale::Scaled(val) => {
                ui.add(egui::DragValue::new(val).speed(0.001).range(0.0..=1.0));
            }
        }
    });
}

fn render_sleeping_color_multiplier(
    ui: &mut egui::Ui,
    state: &mut ResMut<SettingsState>,
    multiplier: &mut Option<[f32; 4]>,
) {
    ui.horizontal(|ui| {
        let mut enabled = multiplier.is_some();
        if ui
            .checkbox(&mut enabled, "Sleeping Color Multiplier")
            .changed()
        {
            if enabled {
                *multiplier = Some(
                    state
                        .sleeping_multiplier_cache
                        .unwrap_or([1.0, 1.0, 0.5, 1.0]),
                );
            } else {
                state.sleeping_multiplier_cache = *multiplier;
                *multiplier = None;
            }
        }
    });

    if let Some(hsla) = multiplier {
        ui.indent("sleeping_multiplier", |ui| {
            ui.horizontal(|ui| {
                ui.label("H:");
                ui.add(
                    egui::DragValue::new(&mut hsla[0])
                        .speed(0.01)
                        .range(0.0..=2.0),
                );
                ui.label("S:");
                ui.add(
                    egui::DragValue::new(&mut hsla[1])
                        .speed(0.01)
                        .range(0.0..=2.0),
                );
                ui.label("L:");
                ui.add(
                    egui::DragValue::new(&mut hsla[2])
                        .speed(0.01)
                        .range(0.0..=2.0),
                );
                ui.label("A:");
                ui.add(
                    egui::DragValue::new(&mut hsla[3])
                        .speed(0.01)
                        .range(0.0..=2.0),
                );
            });
        });
    }
}

fn render_debug_settings(
    ui: &mut egui::Ui,
    state: &mut ResMut<SettingsState>,
    gizmo_store: &mut ResMut<GizmoConfigStore>,
    params: &mut DebugSettingsParams,
) {
    // Falling Sand Debug Gizmos
    egui::CollapsingHeader::new("Falling Sand")
        .default_open(true)
        .show(ui, |ui| {
            // Particle Count toggle
            let mut show_particle_count = params.debug_particle_count.is_some();
            if ui
                .checkbox(&mut show_particle_count, "Show Particle Count")
                .changed()
            {
                if show_particle_count {
                    params.commands.insert_resource(DebugParticleCount);
                } else {
                    params.commands.remove_resource::<DebugParticleCount>();
                }
            }

            // Particle Map toggle
            let mut show_particle_map = params.debug_particle_map.is_some();
            if ui
                .checkbox(&mut show_particle_map, "Show Particle Map (Chunk Borders)")
                .changed()
            {
                if show_particle_map {
                    params.commands.insert_resource(DebugParticleMap);
                } else {
                    params.commands.remove_resource::<DebugParticleMap>();
                }
            }

            // Dirty Rects toggle
            let mut show_dirty_rects = params.debug_dirty_rects.is_some();
            if ui
                .checkbox(&mut show_dirty_rects, "Show Dirty Rects")
                .changed()
            {
                if show_dirty_rects {
                    params.commands.insert_resource(DebugDirtyRects);
                } else {
                    params.commands.remove_resource::<DebugDirtyRects>();
                }
            }

            ui.add_space(4.0);

            // Chunk Border Color
            ui.horizontal(|ui| {
                ui.label("Chunk Border Color:");
                let srgba = params.chunk_border_color.0.to_srgba();
                let mut color32 = egui::Color32::from_rgba_unmultiplied(
                    (srgba.red * 255.0) as u8,
                    (srgba.green * 255.0) as u8,
                    (srgba.blue * 255.0) as u8,
                    (srgba.alpha * 255.0) as u8,
                );

                if ui.color_edit_button_srgba(&mut color32).changed() {
                    params.chunk_border_color.0 =
                        Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
                }
            });

            // Dirty Rect Color
            ui.horizontal(|ui| {
                ui.label("Dirty Rect Color:");
                let srgba = params.dirty_rect_color.0.to_srgba();
                let mut color32 = egui::Color32::from_rgba_unmultiplied(
                    (srgba.red * 255.0) as u8,
                    (srgba.green * 255.0) as u8,
                    (srgba.blue * 255.0) as u8,
                    (srgba.alpha * 255.0) as u8,
                );

                if ui.color_edit_button_srgba(&mut color32).changed() {
                    params.dirty_rect_color.0 =
                        Color::srgba_u8(color32.r(), color32.g(), color32.b(), color32.a());
                }
            });
        });

    ui.add_space(8.0);

    // Physics Gizmos (Avian)
    egui::CollapsingHeader::new("Physics (Avian)")
        .default_open(false)
        .show(ui, |ui| {
            let (_, physics_gizmos) = gizmo_store.config_mut::<PhysicsGizmos>();

            ui.checkbox(&mut physics_gizmos.hide_meshes, "Hide Meshes");
            ui.add_space(8.0);

            egui::CollapsingHeader::new("Colors")
                .default_open(true)
                .show(ui, |ui| {
                    render_color_option(ui, state, "Collider", &mut physics_gizmos.collider_color);
                    render_color_option(ui, state, "AABB", &mut physics_gizmos.aabb_color);
                    render_color_option(
                        ui,
                        state,
                        "Contact Point",
                        &mut physics_gizmos.contact_point_color,
                    );
                    render_color_option(
                        ui,
                        state,
                        "Contact Normal",
                        &mut physics_gizmos.contact_normal_color,
                    );
                    render_color_option(
                        ui,
                        state,
                        "Joint Anchor",
                        &mut physics_gizmos.joint_anchor_color,
                    );
                    render_color_option(
                        ui,
                        state,
                        "Joint Separation",
                        &mut physics_gizmos.joint_separation_color,
                    );
                });

            ui.add_space(8.0);

            egui::CollapsingHeader::new("Raycasts")
                .default_open(false)
                .show(ui, |ui| {
                    render_color_option(ui, state, "Raycast", &mut physics_gizmos.raycast_color);
                    render_color_option(
                        ui,
                        state,
                        "Raycast Point",
                        &mut physics_gizmos.raycast_point_color,
                    );
                    render_color_option(
                        ui,
                        state,
                        "Raycast Normal",
                        &mut physics_gizmos.raycast_normal_color,
                    );
                });

            ui.add_space(8.0);

            egui::CollapsingHeader::new("Shapecasts")
                .default_open(false)
                .show(ui, |ui| {
                    render_color_option(
                        ui,
                        state,
                        "Shapecast",
                        &mut physics_gizmos.shapecast_color,
                    );
                    render_color_option(
                        ui,
                        state,
                        "Shapecast Shape",
                        &mut physics_gizmos.shapecast_shape_color,
                    );
                    render_color_option(
                        ui,
                        state,
                        "Shapecast Point",
                        &mut physics_gizmos.shapecast_point_color,
                    );
                    render_color_option(
                        ui,
                        state,
                        "Shapecast Normal",
                        &mut physics_gizmos.shapecast_normal_color,
                    );
                });

            ui.add_space(8.0);

            egui::CollapsingHeader::new("Advanced")
                .default_open(false)
                .show(ui, |ui| {
                    render_axis_lengths(ui, state, &mut physics_gizmos.axis_lengths);
                    ui.add_space(4.0);
                    render_contact_normal_scale(ui, &mut physics_gizmos.contact_normal_scale);
                    ui.add_space(4.0);
                    render_sleeping_color_multiplier(
                        ui,
                        state,
                        &mut physics_gizmos.sleeping_color_multiplier,
                    );
                });
        });

    ui.add_space(16.0);
    ui.separator();
    ui.add_space(8.0);

    if ui.button("Reset Defaults").clicked() {
        // Reset falling sand debug settings
        params.commands.remove_resource::<DebugParticleCount>();
        params.commands.remove_resource::<DebugParticleMap>();
        params.commands.remove_resource::<DebugDirtyRects>();
        params.chunk_border_color.0 = ChunkBorderColor::default().0;
        params.dirty_rect_color.0 = DirtyRectColor::default().0;

        // Reset physics gizmos
        let (_, physics_gizmos) = gizmo_store.config_mut::<PhysicsGizmos>();
        physics_gizmos.hide_meshes = false;
        physics_gizmos.collider_color = None;
        physics_gizmos.aabb_color = None;
        physics_gizmos.contact_point_color = None;
        physics_gizmos.contact_normal_color = None;
        physics_gizmos.joint_anchor_color = None;
        physics_gizmos.joint_separation_color = None;
        physics_gizmos.raycast_color = None;
        physics_gizmos.raycast_point_color = None;
        physics_gizmos.raycast_normal_color = None;
        physics_gizmos.shapecast_color = None;
        physics_gizmos.shapecast_shape_color = None;
        physics_gizmos.shapecast_point_color = None;
        physics_gizmos.shapecast_normal_color = None;
        physics_gizmos.axis_lengths = None;
        physics_gizmos.contact_normal_scale = ContactGizmoScale::default();
        physics_gizmos.sleeping_color_multiplier = None;

        // Clear cached colors
        state.color_cache.clear();
        state.axis_lengths_cache = None;
        state.sleeping_multiplier_cache = None;
    }
}
