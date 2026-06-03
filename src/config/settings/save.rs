use bevy::prelude::*;
use bevy_falling_sand::debug::{ChunkColor, DebugDirtyRects, DebugParticleMap, DirtyRectColor};
use bevy_persistent::Persistent;

use crate::{
    camera::CameraKeyBindings,
    config::{
        AvianDebugConfig, BevyFallingSandDebugConfig, EarthquakeConfig, Keybindings, OptionalColor,
        PainterConfig, SettingsConfig,
    },
    tools::{
        brush::ToolBrushSize,
        earthquake::{
            DebugEarthquake, EarthquakeBrush, EarthquakeConfiguration, EarthquakeFractureShape,
            EarthquakeShape,
        },
        painter::{
            PainterBrush, PainterConfiguration, PainterKeyBindings, PainterShape, PainterSpawnState,
        },
    },
    ui::UiKeyBindings,
};

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveSettingsBuilder>()
            .add_observer(on_prepare_save_brush)
            .add_observer(on_prepare_save_earthquake)
            .add_observer(on_prepare_save_bfs_debug)
            .add_observer(on_prepare_save_avian_debug)
            .add_observer(on_prepare_save_keys)
            .add_observer(on_prepare_save_settings)
            .add_observer(on_save_settings);
    }
}

/// Trigger this event to begin the settings save process.
#[derive(Event, Default, Debug)]
pub struct PrepareSaveSettingsEvent;

/// Triggered automatically after PrepareSettingsSaveEvent handlers complete.
#[derive(Event, Default, Debug)]
pub struct SaveSettingsEvent;

#[derive(Resource, Default)]
pub struct SaveSettingsBuilder {
    pub painter: Option<PainterConfig>,
    pub earthquake: Option<EarthquakeConfig>,
    pub bfs_debug: Option<BevyFallingSandDebugConfig>,
    pub avian_debug: Option<AvianDebugConfig>,
    pub keys: Option<Keybindings>,
}

fn on_prepare_save_brush(
    _trigger: On<PrepareSaveSettingsEvent>,
    brush_type_state: Res<State<PainterShape>>,
    brush_mode_state: Res<State<PainterSpawnState>>,
    brush_size: Single<&ToolBrushSize, With<PainterBrush>>,
    configuration: Res<PainterConfiguration>,
    mut builder: ResMut<SaveSettingsBuilder>,
) {
    builder.painter = Some(PainterConfig {
        shape: **brush_type_state,
        mode: **brush_mode_state,
        size: **brush_size,
        configuration: configuration.clone(),
    });
}

fn on_prepare_save_earthquake(
    _trigger: On<PrepareSaveSettingsEvent>,
    region_state: Res<State<EarthquakeShape>>,
    fracture_shape_state: Res<State<EarthquakeFractureShape>>,
    brush_size: Single<&ToolBrushSize, With<EarthquakeBrush>>,
    configuration: Res<EarthquakeConfiguration>,
    debug: Option<Res<DebugEarthquake>>,
    mut builder: ResMut<SaveSettingsBuilder>,
) {
    builder.earthquake = Some(EarthquakeConfig {
        region: **region_state,
        fracture_shape: **fracture_shape_state,
        size: **brush_size,
        debug: debug.is_some(),
        configuration: configuration.clone(),
    });
}

fn on_prepare_save_bfs_debug(
    _trigger: On<PrepareSaveSettingsEvent>,
    map: Option<Res<DebugParticleMap>>,
    map_color: Res<ChunkColor>,
    dirty_rect: Option<Res<DebugDirtyRects>>,
    dirty_rect_color: Res<DirtyRectColor>,
    mut builder: ResMut<SaveSettingsBuilder>,
) {
    let map_color = map_color.0.to_srgba();
    let dirty_rect_color = dirty_rect_color.0.to_srgba();
    builder.bfs_debug = Some(BevyFallingSandDebugConfig {
        map: OptionalColor {
            enabled: map.is_some(),
            color: [
                map_color.red,
                map_color.green,
                map_color.blue,
                map_color.alpha,
            ],
        },
        dirty_rects: OptionalColor {
            enabled: dirty_rect.is_some(),
            color: [
                dirty_rect_color.red,
                dirty_rect_color.green,
                dirty_rect_color.blue,
                dirty_rect_color.alpha,
            ],
        },
    });
}

fn on_prepare_save_avian_debug(
    _trigger: On<PrepareSaveSettingsEvent>,
    avian_config: Res<AvianDebugConfig>,
    mut builder: ResMut<SaveSettingsBuilder>,
) {
    builder.avian_debug = Some(avian_config.clone());
}

fn on_prepare_save_keys(
    _trigger: On<PrepareSaveSettingsEvent>,
    camera: Res<CameraKeyBindings>,
    ui: Res<UiKeyBindings>,
    painter: Res<PainterKeyBindings>,
    mut builder: ResMut<SaveSettingsBuilder>,
) {
    builder.keys = Some(Keybindings {
        camera: camera.clone(),
        ui: ui.clone(),
        painter: painter.clone(),
    });
}

fn on_prepare_save_settings(_trigger: On<PrepareSaveSettingsEvent>, mut commands: Commands) {
    commands.trigger(SaveSettingsEvent);
}

fn on_save_settings(
    _trigger: On<SaveSettingsEvent>,
    mut builder: ResMut<SaveSettingsBuilder>,
    mut persistent: ResMut<Persistent<SettingsConfig>>,
) {
    persistent
        .set(SettingsConfig {
            painter: builder.painter.take().expect("painter config not set"),
            earthquake: builder
                .earthquake
                .take()
                .expect("earthquake config not set"),
            bfs_debug: builder.bfs_debug.take().expect("bfs debug config not set"),
            avian_debug: builder
                .avian_debug
                .take()
                .expect("avian debug config not set"),
            keys: builder.keys.take().expect("Keybindings not set"),
        })
        .expect("Failed to save settings");
    persistent
        .persist()
        .expect("Failed to write settings to disk");
}
