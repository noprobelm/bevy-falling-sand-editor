mod cursor_overlay;
mod particle_editor;
mod settings;
mod states;
mod tool_options;

use bevy::prelude::*;

pub use cursor_overlay::*;
pub use particle_editor::*;
pub use settings::*;
pub use states::*;
pub use tool_options::*;

pub struct PopupsPlugin;

impl Plugin for PopupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ParticleEditorPlugin,
            SettingsPlugin,
            ToolOptionsPlugin,
            CursorOverlayPlugin,
        ));
    }
}
