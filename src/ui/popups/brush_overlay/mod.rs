mod systems;
mod ui;
use ui::*;

use bevy::prelude::*;
use systems::*;

pub(super) struct BrushOverlayPlugin;

impl Plugin for BrushOverlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, SystemsPlugin));
    }
}

#[derive(Resource, Default)]
pub struct ShowBrushOverlay;
