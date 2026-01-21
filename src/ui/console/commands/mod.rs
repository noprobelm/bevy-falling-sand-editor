mod help;

use bevy::prelude::*;

pub use help::*;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HelpDirectivePlugin);
    }
}
