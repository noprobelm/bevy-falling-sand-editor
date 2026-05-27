use crate::{
    console_command::ConsoleCommand,
    ui::{CanvasState, SetCanvasStateEvent},
};
use bevy::prelude::*;

#[derive(Default)]
pub struct CanvasCommand;

impl ConsoleCommand for CanvasCommand {
    fn name(&self) -> &'static str {
        "canvas"
    }

    fn description(&self) -> &'static str {
        "Canvas operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(CanvasSetCommand)]
    }
}

#[derive(Default)]
pub struct CanvasSetCommand;

impl ConsoleCommand for CanvasSetCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set canvas configurations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(CanvasSetModeCommand)]
    }
}

#[derive(Default)]
pub struct CanvasSetModeCommand;

impl ConsoleCommand for CanvasSetModeCommand {
    fn name(&self) -> &'static str {
        "state"
    }

    fn description(&self) -> &'static str {
        "Change the canvas state"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        match args[0].to_lowercase().as_str() {
            "select" => {
                info!("Canvas state set to Select");
                commands.trigger(SetCanvasStateEvent(CanvasState::Select));
            }
            "brush" => {
                info!("Canvas state set to Brush");
                commands.trigger(SetCanvasStateEvent(CanvasState::Brush));
            }
            _ => error!("Invalid canvas state. Specify one of 'select', 'brush'"),
        };
    }
}
