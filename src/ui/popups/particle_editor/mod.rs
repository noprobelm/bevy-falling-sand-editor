mod states;
mod ui;

pub use states::*;
use ui::*;

use bevy::prelude::*;

pub(super) struct ParticleEditorPlugin;

impl Plugin for ParticleEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, StatesPlugin));
    }
}
