mod systems;
mod ui;
use ui::*;

use bevy::prelude::*;
use systems::*;

pub(super) struct CursorOverlayPlugin;

impl Plugin for CursorOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, SystemsPlugin));
    }
}

#[derive(Resource, Default)]
pub struct ShowCursorOverlay;
