use bevy::prelude::*;

use crate::{console_command::ConsoleCommand, exit::ExitApplicationEvent};

#[derive(Default)]
pub struct ExitConsoleCommand;

impl ConsoleCommand for ExitConsoleCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "Exit the application"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(ExitApplicationEvent);
    }
}
