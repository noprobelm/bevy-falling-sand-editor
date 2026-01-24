mod exit;
mod help;
mod particles;

use bevy::prelude::*;

pub use exit::*;
pub use help::*;
pub use particles::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            HelpDirectivePlugin,
            ExitDirectivePlugin,
            ParticlesDirectivePlugin,
        ));
    }
}
