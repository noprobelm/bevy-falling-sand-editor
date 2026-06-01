use crate::{
    console_command::ConsoleCommand,
    tools::{SelectedTool, SetSelectedToolEvent},
};
use bevy::prelude::*;

#[derive(Default)]
pub struct ToolCommand;

impl ConsoleCommand for ToolCommand {
    fn name(&self) -> &'static str {
        "tool"
    }

    fn description(&self) -> &'static str {
        "Tool operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(ToolSelectCommand)]
    }
}

#[derive(Default)]
pub struct ToolSelectCommand;

impl ConsoleCommand for ToolSelectCommand {
    fn name(&self) -> &'static str {
        "select"
    }

    fn description(&self) -> &'static str {
        "Select a tool"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.is_empty() {
            error!("Tool seletion required (usage: tools select <brush|select>");
            return;
        }

        match args[0].to_lowercase().as_str() {
            "select" => {
                info!("Setting selected tool to 'Select'");
                commands.trigger(SetSelectedToolEvent(SelectedTool::Select));
            }
            "brush" => {
                info!("Setting selected tool to 'Brush'");
                commands.trigger(SetSelectedToolEvent(SelectedTool::Brush));
            }
            _ => error!("Invalid tool. Specify one of 'select', 'brush'"),
        };
    }
}
