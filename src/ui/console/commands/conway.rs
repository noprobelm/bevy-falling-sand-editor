use crate::console_command::ConsoleCommand;
use crate::game_of_life::GolToggleSignal;
use bevy::prelude::*;

#[derive(Default)]
pub struct ConwayConsoleCommand;

impl ConsoleCommand for ConwayConsoleCommand {
    fn name(&self) -> &'static str {
        "conway"
    }

    fn description(&self) -> &'static str {
        "Game of Life operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(ConwayToggleConsoleCommand)]
    }
}

#[derive(Default)]
struct ConwayToggleConsoleCommand;

impl ConsoleCommand for ConwayToggleConsoleCommand {
    fn name(&self) -> &'static str {
        "toggle"
    }

    fn description(&self) -> &'static str {
        "Toggle the Conway simulation on or off"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(GolToggleSignal);
    }
}
