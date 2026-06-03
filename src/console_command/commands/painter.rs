use bevy::prelude::*;

use crate::{
    console_command::ConsoleCommand,
    tools::painter::{PainterModeState, PainterShape, SetPainterMode, SetPainterShape},
};

#[derive(Default)]
pub struct PainterConsoleCommand;

impl ConsoleCommand for PainterConsoleCommand {
    fn name(&self) -> &'static str {
        "painter"
    }

    fn description(&self) -> &'static str {
        "Painter tool operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(PainterSetConsoleCommand)]
    }
}

#[derive(Default)]
pub struct PainterSetConsoleCommand;

impl ConsoleCommand for PainterSetConsoleCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set painter configuration"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(PainterSetShapeConsoleCommand),
            Box::new(PainterSetModeConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct PainterSetShapeConsoleCommand;

impl ConsoleCommand for PainterSetShapeConsoleCommand {
    fn name(&self) -> &'static str {
        "shape"
    }

    fn description(&self) -> &'static str {
        "Change painter shape"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let Some(shape) = args.first().and_then(|value| parse_painter_shape(value)) else {
            error!("Shape value required (usage: painter set shape <circle|line|cursor>)");
            return;
        };

        commands.trigger(SetPainterShape(shape));
    }
}

#[derive(Default)]
pub struct PainterSetModeConsoleCommand;

impl ConsoleCommand for PainterSetModeConsoleCommand {
    fn name(&self) -> &'static str {
        "mode"
    }

    fn description(&self) -> &'static str {
        "Change painter mode"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let Some(mode) = args.first().and_then(|value| parse_painter_mode(value)) else {
            error!("Mode value required (usage: painter set mode <particles|conway>)");
            return;
        };

        commands.trigger(SetPainterMode(mode));
    }
}

fn parse_painter_shape(value: &str) -> Option<PainterShape> {
    match value.to_lowercase().as_str() {
        "circle" => Some(PainterShape::Circle),
        "line" => Some(PainterShape::Line),
        "cursor" => Some(PainterShape::Cursor),
        _ => {
            error!("Invalid painter shape. Specify one of 'circle', 'line', 'cursor'");
            None
        }
    }
}

fn parse_painter_mode(value: &str) -> Option<PainterModeState> {
    match value.to_lowercase().as_str() {
        "particles" => Some(PainterModeState::Particles),
        "conway" => Some(PainterModeState::Conway),
        _ => {
            error!("Invalid painter mode. Specify one of 'particles', 'conway'");
            None
        }
    }
}
