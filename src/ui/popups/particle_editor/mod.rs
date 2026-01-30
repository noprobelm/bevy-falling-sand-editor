mod resources;
mod setup;
mod states;
mod systems;
mod ui;

pub use resources::*;
use setup::*;
pub use states::*;
use systems::*;
use ui::*;

use bevy::prelude::*;

pub(super) struct ParticleEditorPlugin;

impl Plugin for ParticleEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, SystemsPlugin, StatesPlugin, SetupPlugin));
    }
}
