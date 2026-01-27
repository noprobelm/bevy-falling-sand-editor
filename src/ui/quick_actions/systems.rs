use bevy::prelude::*;
use bevy_falling_sand::debug::{DebugDirtyRects, DebugParticleMap};
use leafwing_input_manager::prelude::ActionState;

use crate::ui::{QuickAction, ShowUi};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_toggle_ui,
                handle_toggle_map,
                handle_toggle_dirty_chunks,
            ),
        );
    }
}

fn toggle_resource<T: Resource + Default>(commands: &mut Commands, resource: &Option<Res<T>>) {
    if resource.is_some() {
        commands.remove_resource::<T>();
    } else {
        commands.init_resource::<T>();
    }
}

fn handle_toggle_ui(
    mut commands: Commands,
    show_ui: Option<Res<ShowUi>>,
    action_state: Single<&ActionState<QuickAction>>,
) {
    if action_state.just_pressed(&QuickAction::ToggleUi) {
        toggle_resource(&mut commands, &show_ui);
    }
}

fn handle_toggle_map(
    mut commands: Commands,
    debug_map: Option<Res<DebugParticleMap>>,
    action_state: Single<&ActionState<QuickAction>>,
) {
    if action_state.just_pressed(&QuickAction::ToggleMapOverlay) {
        toggle_resource(&mut commands, &debug_map);
    }
}

fn handle_toggle_dirty_chunks(
    mut commands: Commands,
    debug_chunks: Option<Res<DebugDirtyRects>>,
    action_state: Single<&ActionState<QuickAction>>,
) {
    if action_state.just_pressed(&QuickAction::ToggleDirtyChunksOverlay) {
        toggle_resource(&mut commands, &debug_chunks);
    }
}
