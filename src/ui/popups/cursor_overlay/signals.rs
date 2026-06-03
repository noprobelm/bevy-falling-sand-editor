use crate::ui::ShowCursorOverlay;
use bevy::prelude::*;

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_toggle_cursor_overlay);
    }
}

#[derive(Event)]
pub struct UiToggleCursorOverlayEvent;

fn on_toggle_cursor_overlay(
    _trigger: On<UiToggleCursorOverlayEvent>,
    mut commands: Commands,
    enabled: Option<Res<ShowCursorOverlay>>,
) {
    if enabled.is_some() {
        commands.remove_resource::<ShowCursorOverlay>();
    } else {
        commands.insert_resource(ShowCursorOverlay);
    }
}
