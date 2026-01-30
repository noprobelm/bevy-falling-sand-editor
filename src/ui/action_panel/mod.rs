mod particle_editor;
mod settings;
mod setup;
mod ui;

use bevy::prelude::*;

pub use particle_editor::*;
pub use settings::*;
use setup::*;
use ui::*;

pub struct ActionPanelPlugin;

impl Plugin for ActionPanelPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, SetupPlugin, ParticleEditorPlugin, SettingsPlugin));
    }
}
