mod resources;
mod setup;
mod signals;
mod states;
mod ui;

pub use resources::*;
use setup::*;
pub use signals::*;
pub use states::*;
use ui::*;

use bevy::prelude::*;

pub(super) struct ParticleEditorPlugin;

impl Plugin for ParticleEditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            UiPlugin,
            ResourcesPlugin,
            StatesPlugin,
            SetupPlugin,
            SignalsPlugin,
        ));
    }
}
