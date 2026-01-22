mod help;

use bevy::prelude::*;

pub use help::*;

use crate::ui::ConsoleConfiguration;

pub struct CommandsPlugin;

impl Plugin for CommandsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HelpCommandPlugin);
    }
}
