use bevy::prelude::*;
use bevy_falling_sand::debug::{ChunkColor, DebugDirtyRects, DebugParticleMap, DirtyRectColor};
use bevy_persistent::Persistent;

use crate::{
    brush::{BrushKeyBindings, BrushSize, BrushSpawnState, BrushTypeState},
    camera::CameraKeyBindings,
    config::{
        AvianDebugConfig, BevyFallingSandDebugConfig, BrushConfig, Keybindings, OptionalColor,
        SettingsConfig,
    },
    ui::UiKeyBindings,
};

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveSettingsBuilder>()
            .add_observer(on_prepare_save_brush)
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
    pub brush: Option<BrushConfig>,
    pub bfs_debug: Option<BevyFallingSandDebugConfig>,
    pub avian_debug: Option<AvianDebugConfig>,
    pub keys: Option<Keybindings>,
}

fn on_prepare_save_brush(
    _trigger: On<PrepareSaveSettingsEvent>,
    brush_type_state: Res<State<BrushTypeState>>,
    brush_mode_state: Res<State<BrushSpawnState>>,
    brush_size: Single<&BrushSize>,
    mut builder: ResMut<SaveSettingsBuilder>,
) {
    builder.brush = Some(BrushConfig {
        btype: **brush_type_state,
        mode: **brush_mode_state,
        size: **brush_size,
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
    brush: Res<BrushKeyBindings>,
    mut builder: ResMut<SaveSettingsBuilder>,
) {
    builder.keys = Some(Keybindings {
        camera: camera.clone(),
        ui: ui.clone(),
        brush: brush.clone(),
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
            brush: builder.brush.take().expect("brush config not set"),
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
