mod brush;
mod canvas;
mod conway;
mod earthquake;
mod exit;
mod help;
mod helpers;
mod particles;
mod save;
pub(crate) mod scene;
mod select;

use bevy::prelude::*;

pub use brush::*;
pub use canvas::*;
pub use conway::*;
pub use earthquake::*;
pub use exit::*;
pub use help::*;
use helpers::*;
pub use particles::*;
pub use save::*;
pub use scene::*;
pub use select::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HelpConsoleCommandPlugin)
            .add_systems(Startup, scene::load_scene_assets);
    }
}
