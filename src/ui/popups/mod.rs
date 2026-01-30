mod particle_editor;
mod settings;
mod states;

use bevy::prelude::*;

pub use particle_editor::*;
pub use settings::*;
pub use states::*;

pub struct PopupsPlugin;

impl Plugin for PopupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ParticleEditorPlugin, SettingsPlugin));
    }
}
