use bevy::prelude::*;
use bevy_falling_sand::prelude::{
    DespawnAllParticlesSignal, DespawnDynamicParticlesSignal, DespawnParticleTypeChildrenSignal,
    DespawnStaticParticlesSignal, ParticleType,
};

use super::parse_position;
use crate::{
    console_command::ConsoleCommand,
    particles::{
        ParticleName, SpawnBarnsleyEvent, SpawnTextEvent, TextAlignment,
        carpet::SpawnSierpinskiCarpetEvent, triangle::SpawnSierpinskiTriangleEvent,
    },
};

pub struct ParticlesConsoleCommandPlugin;

impl Plugin for ParticlesConsoleCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DespawnParticleTypeByNameCommand>()
            .add_systems(Update, msgr_despawn_particle_type_by_name_command);
    }
}

#[derive(Message, Clone, Debug)]
struct DespawnParticleTypeByNameCommand {
    name: String,
}

#[derive(Default)]
pub struct ParticlesConsoleCommand;

impl ConsoleCommand for ParticlesConsoleCommand {
    fn name(&self) -> &'static str {
        "particles"
    }

    fn description(&self) -> &'static str {
        "Particle system operations"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(ParticlesResetConsoleCommand),
            Box::new(ParticlesDespawnConsoleCommand),
            Box::new(ParticlesSpawnConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesResetConsoleCommand;

impl ConsoleCommand for ParticlesResetConsoleCommand {
    fn name(&self) -> &'static str {
        "reset"
    }

    fn description(&self) -> &'static str {
        "Reset particle-related components"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(ParticlesResetWallConsoleCommand),
            Box::new(ParticlesResetDynamicConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesResetWallConsoleCommand;

impl ConsoleCommand for ParticlesResetWallConsoleCommand {
    fn name(&self) -> &'static str {
        "wall"
    }

    fn description(&self) -> &'static str {
        "Reset wall particles"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(ParticlesResetWallAllConsoleCommand)]
    }
}

#[derive(Default)]
pub struct ParticlesResetDynamicConsoleCommand;

impl ConsoleCommand for ParticlesResetDynamicConsoleCommand {
    fn name(&self) -> &'static str {
        "dynamic"
    }

    fn description(&self) -> &'static str {
        "Reset dynamic particles"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(ParticlesResetDynamicAllConsoleCommand)]
    }
}

#[derive(Default)]
pub struct ParticlesResetWallAllConsoleCommand;

impl ConsoleCommand for ParticlesResetWallAllConsoleCommand {
    fn name(&self) -> &'static str {
        "all"
    }

    fn description(&self) -> &'static str {
        "Reset all wall particles"
    }

    fn run(&self, _args: &[String], _commands: &mut Commands) {
        info!("Resetting all wall particles to parent data");
    }
}

#[derive(Default)]
pub struct ParticlesResetDynamicAllConsoleCommand;

impl ConsoleCommand for ParticlesResetDynamicAllConsoleCommand {
    fn name(&self) -> &'static str {
        "all"
    }

    fn description(&self) -> &'static str {
        "Reset all dynamic particles"
    }

    fn run(&self, _args: &[String], _commands: &mut Commands) {
        info!("Resetting all dynamic particles to parent data")
    }
}

#[derive(Default)]
pub struct ParticlesDespawnConsoleCommand;

impl ConsoleCommand for ParticlesDespawnConsoleCommand {
    fn name(&self) -> &'static str {
        "despawn"
    }

    fn description(&self) -> &'static str {
        "Despawn particles from the world"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(ParticlesDespawnDynamicConsoleCommand),
            Box::new(ParticlesDespawnStaticConsoleCommand),
            Box::new(ParticlesDespawnAllConsoleCommand),
            Box::new(ParticlesDespawnNamedConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesDespawnDynamicConsoleCommand;

impl ConsoleCommand for ParticlesDespawnDynamicConsoleCommand {
    fn name(&self) -> &'static str {
        "dynamic"
    }

    fn description(&self) -> &'static str {
        "Despawn dynamic particles from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all dynamic particles from the world");
        commands.trigger(DespawnDynamicParticlesSignal);
    }
}

#[derive(Default)]
pub struct ParticlesDespawnStaticConsoleCommand;

impl ConsoleCommand for ParticlesDespawnStaticConsoleCommand {
    fn name(&self) -> &'static str {
        "static"
    }

    fn description(&self) -> &'static str {
        "Despawn static particles from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all static particles from the world");
        commands.trigger(DespawnStaticParticlesSignal);
    }
}

#[derive(Default)]
pub struct ParticlesDespawnAllConsoleCommand;

impl ConsoleCommand for ParticlesDespawnAllConsoleCommand {
    fn name(&self) -> &'static str {
        "all"
    }

    fn description(&self) -> &'static str {
        "Despawn all particles from the world"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Despawning all particles from the world");
        commands.trigger(DespawnAllParticlesSignal);
    }
}

#[derive(Default)]
pub struct ParticlesDespawnNamedConsoleCommand;

impl ConsoleCommand for ParticlesDespawnNamedConsoleCommand {
    fn name(&self) -> &'static str {
        "named"
    }

    fn description(&self) -> &'static str {
        "Despawn all particles of a specified particle type"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let Some(name) = particle_type_name_arg(args) else {
            warn!("Usage: particles despawn named <particle_type_name>");
            return;
        };
        commands.write_message(DespawnParticleTypeByNameCommand { name });
    }
}

fn particle_type_name_arg(args: &[String]) -> Option<String> {
    let name = args.join(" ");
    let name = name.trim();
    (!name.is_empty()).then(|| name.to_string())
}

fn msgr_despawn_particle_type_by_name_command(
    mut messages: MessageReader<DespawnParticleTypeByNameCommand>,
    particle_types: Query<(&ParticleType, Option<&ParticleName>)>,
    mut commands: Commands,
) {
    for message in messages.read() {
        let matches: Vec<_> = particle_types
            .iter()
            .filter_map(|(particle_type, name)| {
                let name = name?;
                name.0
                    .eq_ignore_ascii_case(&message.name)
                    .then_some((particle_type.id(), name.0.clone()))
            })
            .collect();

        match matches.as_slice() {
            [] => {
                warn!("No particle type named '{}' found", message.name);
            }
            [(id, name)] => {
                info!("Despawning all particles of type '{name}'");
                commands.trigger(DespawnParticleTypeChildrenSignal::from_particle_type(*id));
            }
            _ => {
                let names = matches
                    .iter()
                    .map(|(_, name)| name.as_str())
                    .collect::<Vec<_>>()
                    .join(", ");
                warn!(
                    "Particle type name '{}' is ambiguous: {names}",
                    message.name
                );
            }
        }
    }
}

#[derive(Default)]
pub struct ParticlesSpawnConsoleCommand;

impl ConsoleCommand for ParticlesSpawnConsoleCommand {
    fn name(&self) -> &'static str {
        "spawn"
    }

    fn description(&self) -> &'static str {
        "Spawn particle patterns"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(ParticlesSpawnSierpinskiConsoleCommand),
            Box::new(ParticlesSpawnBarnsleyConsoleCommand),
            Box::new(ParticlesSpawnTextConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesSpawnSierpinskiConsoleCommand;

impl ConsoleCommand for ParticlesSpawnSierpinskiConsoleCommand {
    fn name(&self) -> &'static str {
        "sierpinski"
    }

    fn description(&self) -> &'static str {
        "Spawn a sierpinski carpet. Usage: sierpinski <x>,<y> [depth]"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(ParticlesSpawnSierpinskiCarpetConsoleCommand),
            Box::new(ParticlesSpawnSierpinskiTriangleConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct ParticlesSpawnSierpinskiCarpetConsoleCommand;

impl ConsoleCommand for ParticlesSpawnSierpinskiCarpetConsoleCommand {
    fn name(&self) -> &'static str {
        "carpet"
    }

    fn description(&self) -> &'static str {
        "Spawn a sierpinski carpet. Usage: sierpinski <x>,<y> [depth]"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let (position_args, depth) = match args.last().and_then(|s| s.parse::<u32>().ok()) {
            Some(d) => (&args[..args.len() - 1], d),
            None => (args, 6),
        };

        match parse_position::<IVec2>(position_args) {
            Ok(center) => {
                info!("Spawning sierpinski carpet at {center} with depth {depth}");
                commands.trigger(SpawnSierpinskiCarpetEvent { center, depth });
            }
            Err(e) => {
                warn!("{e}");
            }
        }
    }
}

#[derive(Default)]
pub struct ParticlesSpawnSierpinskiTriangleConsoleCommand;

impl ConsoleCommand for ParticlesSpawnSierpinskiTriangleConsoleCommand {
    fn name(&self) -> &'static str {
        "triangle"
    }

    fn description(&self) -> &'static str {
        "Spawn a sierpinski triangle. Usage: sierpinski <x>,<y> [depth]"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let (position_args, depth) = match args.last().and_then(|s| s.parse::<u32>().ok()) {
            Some(d) => (&args[..args.len() - 1], d),
            None => (args, 6),
        };

        match parse_position::<IVec2>(position_args) {
            Ok(center) => {
                info!("Spawning sierpinski carpet at {center} with depth {depth}");
                commands.trigger(SpawnSierpinskiTriangleEvent { center, depth });
            }
            Err(e) => {
                warn!("{e}");
            }
        }
    }
}

#[derive(Default)]
pub struct ParticlesSpawnBarnsleyConsoleCommand;

impl ConsoleCommand for ParticlesSpawnBarnsleyConsoleCommand {
    fn name(&self) -> &'static str {
        "barnsley"
    }

    fn description(&self) -> &'static str {
        "Spawn a barnsley fern. Usage: barnsley <center_x>,<center_y> <size_x,size_y> <num_iterations> <f1> <f2> <f3>"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.len() < 6 {
            warn!(
                "Usage: barnsley <center_x>,<center_y> <size_x>,<size_y> <num_iterations> <f1> <f2> <f3>"
            );
            return;
        }

        let center = match parse_position::<IVec2>(&args[0..1]) {
            Ok(c) => c,
            Err(e) => {
                warn!("Invalid center: {e}");
                return;
            }
        };

        let size = match parse_position::<IVec2>(&args[1..2]) {
            Ok(s) => s,
            Err(e) => {
                warn!("Invalid size: {e}");
                return;
            }
        };

        let num_iterations: u32 = match args[2].parse() {
            Ok(v) => v,
            Err(_) => {
                warn!("Invalid num_iterations: must be a positive integer");
                return;
            }
        };

        let f1: f32 = match args[3].parse() {
            Ok(v) => v,
            Err(_) => {
                warn!("Invalid f1: must be a floating point number");
                return;
            }
        };

        let f2: f32 = match args[4].parse() {
            Ok(v) => v,
            Err(_) => {
                warn!("Invalid f2: must be a floating point number");
                return;
            }
        };

        let f3: f32 = match args[5].parse() {
            Ok(v) => v,
            Err(_) => {
                warn!("Invalid f3: must be a floating point number");
                return;
            }
        };

        if f1 <= 0.0 || f1 >= 1.0 {
            warn!("f1 must be greater than 0 and less than 1");
            return;
        }
        if f2 <= 0.0 || f2 >= 1.0 {
            warn!("f2 must be greater than 0 and less than 1");
            return;
        }
        if f3 <= 0.0 || f3 >= 1.0 {
            warn!("f3 must be greater than 0 and less than 1");
            return;
        }
        if f2 <= f1 {
            warn!("f2 must be greater than f1");
            return;
        }
        if f3 <= f2 {
            warn!("f3 must be greater than f2");
            return;
        }

        info!(
            "Spawning barnsley fern at {center} with size {size}, num_iterations={num_iterations}, f1={f1}, f2={f2}, f3={f3}"
        );
        commands.trigger(SpawnBarnsleyEvent {
            center,
            size,
            num_iterations,
            f1,
            f2,
            f3,
        });
    }
}

#[derive(Default)]
pub struct ParticlesSpawnTextConsoleCommand;

impl ConsoleCommand for ParticlesSpawnTextConsoleCommand {
    fn name(&self) -> &'static str {
        "text"
    }

    fn description(&self) -> &'static str {
        "Spawn text as particles. Usage: text <x>,<y> <font_size> [--align left|center|right] <text...>. Use \\n for line breaks."
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if args.len() < 3 {
            warn!(
                "Usage: particles spawn text <x>,<y> <font_size> [--align left|center|right] <text...>"
            );
            return;
        }

        let center = match parse_position::<IVec2>(&args[0..1]) {
            Ok(c) => c,
            Err(e) => {
                warn!("Invalid position: {e}");
                return;
            }
        };

        let font_size: f32 = match args[1].parse() {
            Ok(v) if v > 0.0 => v,
            _ => {
                warn!("Invalid font_size: must be a positive number");
                return;
            }
        };

        let mut text_start = 2;
        let mut alignment = TextAlignment::Left;

        if args.get(2).map(|s| s.as_str()) == Some("--align") {
            match args.get(3).map(|s| s.as_str()) {
                Some("left") => alignment = TextAlignment::Left,
                Some("center") => alignment = TextAlignment::Center,
                Some("right") => alignment = TextAlignment::Right,
                Some(other) => {
                    warn!("Unknown alignment \"{other}\". Expected: left, center, right");
                    return;
                }
                None => {
                    warn!("--align requires a value: left, center, or right");
                    return;
                }
            }
            text_start = 4;
        }

        if args.len() <= text_start {
            warn!("No text provided");
            return;
        }

        // Users write literal \n for line breaks. Single-quote the text so shlex
        // preserves the backslash: 'HELLO\nWORLD'. Replace the two-char sequence
        // with a real newline before passing it along.
        let text = args[text_start..].join(" ").replace(r"\n", "\n");

        info!(
            "Spawning text \"{text}\" at {center} with font size {font_size}, alignment {alignment:?}"
        );
        commands.trigger(SpawnTextEvent {
            center,
            text,
            font_size,
            alignment,
        });
    }
}
