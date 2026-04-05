mod brush;
mod canvas;
mod conway;
mod exit;
mod help;
mod helpers;
mod particles;
pub(crate) mod scene;

use bevy::prelude::*;

pub use brush::*;
pub use canvas::*;
pub use conway::*;
pub use exit::*;
pub use help::*;
use helpers::*;
pub use particles::*;
pub use scene::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HelpConsoleCommandPlugin)
            .add_systems(Startup, scene::load_scene_assets);
    }
}
