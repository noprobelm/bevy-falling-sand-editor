use crate::{
    brush::{
        BrushModeSpawnState, BrushSetModeSignal, BrushSetSizeSignal, BrushSetTypeSignal,
        BrushTypeState,
    },
    console_command::ConsoleCommand,
};
use bevy::prelude::*;

#[derive(Default)]
pub struct BrushConsoleCommand;

impl ConsoleCommand for BrushConsoleCommand {
    fn name(&self) -> &'static str {
        "brush"
    }

    fn description(&self) -> &'static str {
        "Brush operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(BrushSetConsoleCommand)]
    }
}

#[derive(Default)]
pub struct BrushSetConsoleCommand;

impl ConsoleCommand for BrushSetConsoleCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set brush configurations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(BrushSetTypeConsoleCommand),
            Box::new(BrushSetSizeConsoleCommand),
            Box::new(BrushSetModeConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct BrushSetTypeConsoleCommand;

impl ConsoleCommand for BrushSetTypeConsoleCommand {
    fn name(&self) -> &'static str {
        "type"
    }

    fn description(&self) -> &'static str {
        "Change the brush type"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        match args[0].to_lowercase().as_str() {
            "circle" => commands.trigger(BrushSetTypeSignal(BrushTypeState::Circle)),
            "line" => commands.trigger(BrushSetTypeSignal(BrushTypeState::Line)),
            "cursor" => commands.trigger(BrushSetTypeSignal(BrushTypeState::Cursor)),
            _ => error!("Invalid brush type. Specify one of 'circle', 'line', 'cursor'"),
        };
    }
}

#[derive(Default)]
pub struct BrushSetSizeConsoleCommand;

impl ConsoleCommand for BrushSetSizeConsoleCommand {
    fn name(&self) -> &'static str {
        "size"
    }

    fn description(&self) -> &'static str {
        "Change the brush size"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.is_empty() {
            error!("Size value required (usage: brush set size <value>");
            return;
        }

        match args[0].parse::<usize>() {
            Ok(size) => {
                if size == 0 {
                    error!("Brush size must be greater than 0");
                } else {
                    info!("Setting brush size to {}", size);
                    commands.trigger(BrushSetSizeSignal(size));
                }
            }
            Err(_) => {
                error!("'{}' is not a valid size value", args[0]);
            }
        }
    }
}

#[derive(Default)]
pub struct BrushSetModeConsoleCommand;

impl ConsoleCommand for BrushSetModeConsoleCommand {
    fn name(&self) -> &'static str {
        "mode"
    }

    fn description(&self) -> &'static str {
        "Change the brush spawn mode"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.is_empty() {
            error!("Mode value required (usage: brush set mode <particles|conway>)");
            return;
        }

        match args[0].to_lowercase().as_str() {
            "particles" => commands.trigger(BrushSetModeSignal(BrushModeSpawnState::Particles)),
            "conway" => commands.trigger(BrushSetModeSignal(BrushModeSpawnState::Conway)),
            _ => error!("Invalid brush mode. Specify one of 'particles', 'conway'"),
        };
    }
}
