mod ui;
mod signals;
use ui::*;
use signals::*;

use bevy::prelude::*;
pub use signals::*;


pub(super) struct CursorOverlayPlugin;

impl Plugin for CursorOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, SignalsPlugin));
    }
}

#[derive(Resource, Default)]
pub struct ShowCursorOverlay;
