mod brush;
mod conway;
mod earthquake;
mod exit;
mod help;
mod helpers;
mod particles;
mod rigid_body;
mod save;
pub(crate) mod scene;
mod select;
mod setup;
mod tools;
mod ui;

use bevy::prelude::*;

pub use brush::*;
pub use conway::*;
pub use earthquake::*;
pub use exit::*;
pub use help::*;
use helpers::*;
pub use particles::*;
pub use rigid_body::*;
pub use save::*;
pub use scene::*;
pub use select::*;
use setup::SetupPlugin;
pub use tools::*;
pub use ui::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SetupPlugin)
            .add_systems(Startup, scene::load_scene_assets);
    }
}
