mod ui;
use ui::*;

use bevy::prelude::*;

pub(super) struct CursorOverlayPlugin;

impl Plugin for CursorOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiPlugin);
    }
}

#[derive(Resource, Default)]
pub struct ShowCursorOverlay;

#[derive(Event)]
pub struct UiToggleCursorOverlaySignal;
