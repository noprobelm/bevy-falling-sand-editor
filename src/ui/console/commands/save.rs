use bevy::prelude::*;

use crate::{console_command::ConsoleCommand, save::SaveApplicationEvent};

#[derive(Default)]
pub struct SaveCommand;

impl ConsoleCommand for SaveCommand {
    fn name(&self) -> &'static str {
        "save"
    }

    fn description(&self) -> &'static str {
        "Save the current application state"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(SaveApplicationEvent);
    }
}
