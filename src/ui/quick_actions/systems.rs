use bevy::prelude::*;
use bevy_falling_sand::debug::{DebugDirtyRects, DebugParticleMap};
use leafwing_input_manager::{common_conditions::action_just_pressed, prelude::ActionState};

use crate::ui::{QuickAction, ShowUi};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                handle_toggle_ui.run_if(action_just_pressed(QuickAction::ToggleUi)),
                handle_toggle_map.run_if(action_just_pressed(QuickAction::ToggleMapOverlay)),
                handle_toggle_dirty_chunks
                    .run_if(action_just_pressed(QuickAction::ToggleDirtyChunksOverlay)),
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

fn handle_toggle_ui(mut commands: Commands, show_ui: Option<Res<ShowUi>>) {
    toggle_resource(&mut commands, &show_ui);
}

fn handle_toggle_map(mut commands: Commands, debug_map: Option<Res<DebugParticleMap>>) {
    toggle_resource(&mut commands, &debug_map);
}

fn handle_toggle_dirty_chunks(mut commands: Commands, debug_chunks: Option<Res<DebugDirtyRects>>) {
    toggle_resource(&mut commands, &debug_chunks);
}
