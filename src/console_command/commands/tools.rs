use crate::{
    console_command::ConsoleCommand,
    tools::{SelectedTool, SetSelectedToolEvent, brush::SetSelectedToolBrushSize},
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
        vec![Box::new(ToolSelectCommand), Box::new(ToolBrushCommand)]
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
            error!("Tool selection required (usage: tool select <painter|earthquake|select>)");
            return;
        }

        match args[0].to_lowercase().as_str() {
            "select" => {
                info!("Setting selected tool to 'Select'");
                commands.trigger(SetSelectedToolEvent(SelectedTool::Select));
            }
            "painter" => {
                info!("Setting selected tool to 'Painter'");
                commands.trigger(SetSelectedToolEvent(SelectedTool::Painter));
            }
            "earthquake" => {
                info!("Setting selected tool to 'Earthquake'");
                commands.trigger(SetSelectedToolEvent(SelectedTool::Earthquake))
            }
            _ => error!("Invalid tool. Specify one of 'select', 'painter', 'earthquake'"),
        };
    }
}

#[derive(Default)]
pub struct ToolBrushCommand;

impl ConsoleCommand for ToolBrushCommand {
    fn name(&self) -> &'static str {
        "brush"
    }

    fn description(&self) -> &'static str {
        "Configure the selected tool's brush"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(ToolBrushSetCommand)]
    }
}

#[derive(Default)]
pub struct ToolBrushSetCommand;

impl ConsoleCommand for ToolBrushSetCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set selected tool brush configuration"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(ToolBrushSetSizeCommand)]
    }
}

#[derive(Default)]
pub struct ToolBrushSetSizeCommand;

impl ConsoleCommand for ToolBrushSetSizeCommand {
    fn name(&self) -> &'static str {
        "size"
    }

    fn description(&self) -> &'static str {
        "Change the selected tool brush size"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.is_empty() {
            error!("Size value required (usage: tool brush set size <value>)");
            return;
        }

        match args[0].parse::<f32>() {
            Ok(size) if size > 0.0 => {
                info!("Setting brush size to {}", size);
                commands.trigger(SetSelectedToolBrushSize(size));
            }
            Ok(_) => {
                error!("Brush size must be greater than 0");
            }
            Err(_) => {
                error!("'{}' is not a valid size value", args[0]);
            }
        }
    }
}
