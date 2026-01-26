mod commands;
mod log_capture;
mod setup;
mod state;
mod ui;

use bevy::prelude::*;

pub use commands::*;
pub use log_capture::*;
pub use setup::*;
pub use state::*;
use ui::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((UiPlugin, CommandsPlugin, StatePlugin));
    }
}


