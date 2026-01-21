mod help;

use bevy::prelude::*;

pub use help::HelpDirective;

use crate::ui::console::help::HelpDirectivePlugin;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HelpDirectivePlugin);
    }
}
