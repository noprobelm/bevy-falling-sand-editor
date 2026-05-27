mod log_capture;
mod setup;
mod states;
mod ui;

use bevy::prelude::*;

pub use crate::console_command::commands::CommandsPlugin;
pub use log_capture::*;
pub use setup::*;
pub use states::*;
use ui::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, UiPlugin, CommandsPlugin, StatePlugin));
    }
}
