use avian2d::prelude::PhysicsGizmos;
use bevy::prelude::*;
use bevy_falling_sand::debug::{ChunkColor, DebugDirtyRects, DebugParticleMap, DirtyRectColor};
use bevy_persistent::Persistent;

use crate::{config::SettingsConfig, setup::SetupSystems};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Startup,
            (
                load_bfs_settings
                    .run_if(condition_bfs_debug_ready)
                    .in_set(SetupSystems::Debug),
                load_avian_settings.in_set(SetupSystems::Debug),
            ),
        );
    }
}

fn load_bfs_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    if settings_config.get().bfs_debug.map.enabled {
        commands.insert_resource(DebugParticleMap);
    } else {
        commands.remove_resource::<DebugParticleMap>();
    }
    let color = settings_config.get().bfs_debug.map.color;
    commands.insert_resource(ChunkColor(Color::srgba(
        color[0], color[1], color[2], color[3],
    )));

    if settings_config.get().bfs_debug.dirty_rects.enabled {
        commands.insert_resource(DebugDirtyRects)
    } else {
        commands.remove_resource::<DebugDirtyRects>();
    }
    let color = settings_config.get().bfs_debug.dirty_rects.color;
    commands.insert_resource(DirtyRectColor(Color::srgba(
        color[0], color[1], color[2], color[3],
    )));
}

fn load_avian_settings(
    mut commands: Commands,
    mut gizmo_store: ResMut<GizmoConfigStore>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let avian_config = settings_config.get().avian_debug.clone();
    let (_, physics_gizmos) = gizmo_store.config_mut::<PhysicsGizmos>();
    *physics_gizmos = avian_config.clone().into();
    commands.insert_resource(avian_config);
}

fn condition_bfs_debug_ready(
    debug_particle_map: Option<Res<DebugParticleMap>>,
    debug_dirty_rects: Option<Res<DebugDirtyRects>>,
) -> bool {
    debug_particle_map.is_some() && debug_dirty_rects.is_some()
}
