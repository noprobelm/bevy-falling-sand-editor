use bevy::prelude::*;

use leafwing_input_manager::common_conditions::action_just_pressed;

use crate::ui::{QuickAction, ShowCursorOverlay};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        // app.configure_set(Update);
        app.add_systems(
            Update,
            handle_toggle_overlay.run_if(action_just_pressed(QuickAction::ToggleCursorInfoOverlay)),
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

fn handle_toggle_overlay(mut commands: Commands, show_overlay: Option<Res<ShowCursorOverlay>>) {
    toggle_resource(&mut commands, &show_overlay);
}
