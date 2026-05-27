use bevy::prelude::*;

use super::parse_position;
use crate::{console_command::ConsoleCommand, earthquake::Earthquake};

#[derive(Default)]
pub struct EarthquakeConsoleCommand;

impl ConsoleCommand for EarthquakeConsoleCommand {
    fn name(&self) -> &'static str {
        "earthquake"
    }

    fn description(&self) -> &'static str {
        "Trigger an earthquake. Usage: earthquake <x>,<y> <radius>"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.len() < 2 {
            warn!("Usage: earthquake <x>,<y> <radius>");
            return;
        }

        let center = match parse_position::<Vec2>(&args[0..1]) {
            Ok(c) => c,
            Err(e) => {
                warn!("Invalid position: {e}");
                return;
            }
        };

        let radius: f32 = match args[1].parse() {
            Ok(v) if v > 0.0 => v,
            _ => {
                warn!("Invalid radius: must be a positive number");
                return;
            }
        };

        info!("Triggering earthquake at {center} with radius {radius}");
        commands.trigger(Earthquake { center, radius });
    }
}
