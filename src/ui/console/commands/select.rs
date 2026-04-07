use bevy::prelude::*;

use crate::{
    canvas::select::states::{SelectModeState, SetSelectModeEvent},
    console_command::ConsoleCommand,
};

#[derive(Default)]
pub struct SelectCommand;

impl ConsoleCommand for SelectCommand {
    fn name(&self) -> &'static str {
        "select"
    }

    fn description(&self) -> &'static str {
        "Select tool operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(SelectSetCommand)]
    }
}

#[derive(Default)]
struct SelectSetCommand;

impl ConsoleCommand for SelectSetCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set select tool configurations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(SelectSetModeCommand)]
    }
}

#[derive(Default)]
struct SelectSetModeCommand;

impl ConsoleCommand for SelectSetModeCommand {
    fn name(&self) -> &'static str {
        "mode"
    }

    fn description(&self) -> &'static str {
        "Set the select tool mode (drag, throw)"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        match args[0].to_lowercase().as_str() {
            "drag" => {
                info!("Select mode set to Drag");
                commands.trigger(SetSelectModeEvent(SelectModeState::Drag));
            }
            "throw" => {
                info!("Select mode set to Throw");
                commands.trigger(SetSelectModeEvent(SelectModeState::Throw));
            }
            _ => error!("Invalid select mode. Specify one of 'drag', 'throw'"),
        };
    }
}
