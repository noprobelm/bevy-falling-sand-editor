use bevy::prelude::*;

use super::parse_position;
use crate::{
    console_command::ConsoleCommand,
    tools::earthquake::{
        Earthquake, EarthquakeFractureShape, EarthquakeRegion, EarthquakeShape,
        SetEarthquakeFractureShape, SetEarthquakeShape,
    },
};

#[derive(Default)]
pub struct EarthquakeConsoleCommand;

impl ConsoleCommand for EarthquakeConsoleCommand {
    fn name(&self) -> &'static str {
        "earthquake"
    }

    fn description(&self) -> &'static str {
        "Trigger an earthquake. Usage: earthquake circle <x>,<y> <radius> | earthquake rect <x>,<y> <w>,<h> [degrees] | earthquake poly <x1>,<y1> <x2>,<y2> <x3>,<y3> ..."
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(EarthquakeSetConsoleCommand)]
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if let Some(kind) = args.first().map(String::as_str) {
            match kind {
                "circle" => {
                    run_circle_earthquake(&args[1..], commands);
                    return;
                }
                "rect" | "rectangle" => {
                    run_rect_earthquake(&args[1..], commands);
                    return;
                }
                "poly" | "polygon" => {
                    run_polygon_earthquake(&args[1..], commands);
                    return;
                }
                _ => {}
            }
        }

        // Backward-compatible shorthand for the old circle-only command.
        // Equivalent to `earthquake circle <x>,<y> <radius>`.
        run_circle_earthquake(args, commands);
    }
}

#[derive(Default)]
pub struct EarthquakeSetConsoleCommand;

impl ConsoleCommand for EarthquakeSetConsoleCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set earthquake configuration"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(EarthquakeSetShapeConsoleCommand),
            Box::new(EarthquakeSetFractureConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct EarthquakeSetShapeConsoleCommand;

impl ConsoleCommand for EarthquakeSetShapeConsoleCommand {
    fn name(&self) -> &'static str {
        "shape"
    }

    fn description(&self) -> &'static str {
        "Change earthquake brush shape"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let Some(shape) = args.first().and_then(|value| parse_earthquake_shape(value)) else {
            error!("Shape value required (usage: earthquake set shape <circle|rect|polygon>)");
            return;
        };

        commands.trigger(SetEarthquakeShape(shape));
    }
}

#[derive(Default)]
pub struct EarthquakeSetFractureConsoleCommand;

impl ConsoleCommand for EarthquakeSetFractureConsoleCommand {
    fn name(&self) -> &'static str {
        "fracture"
    }

    fn description(&self) -> &'static str {
        "Change earthquake fracture shape"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let Some(shape) = args
            .first()
            .and_then(|value| parse_earthquake_fracture(value))
        else {
            error!("Fracture value required (usage: earthquake set fracture <exact|convex>)");
            return;
        };

        commands.trigger(SetEarthquakeFractureShape(shape));
    }
}

fn parse_earthquake_shape(value: &str) -> Option<EarthquakeShape> {
    match value.to_lowercase().as_str() {
        "circle" => Some(EarthquakeShape::Circle),
        "rect" | "rectangle" => Some(EarthquakeShape::Rect),
        "poly" | "polygon" => Some(EarthquakeShape::Polygon),
        _ => {
            error!("Invalid earthquake shape. Specify one of 'circle', 'rect', 'polygon'");
            None
        }
    }
}

fn parse_earthquake_fracture(value: &str) -> Option<EarthquakeFractureShape> {
    match value.to_lowercase().as_str() {
        "exact" | "voronoi" | "exact-voronoi" | "exact_voronoi" => {
            Some(EarthquakeFractureShape::Concave)
        }
        "convex" | "hull" | "convex-hull" | "convex_hull" => Some(EarthquakeFractureShape::Convex),
        _ => {
            error!("Invalid earthquake fracture. Specify one of 'exact', 'convex'");
            None
        }
    }
}

fn run_circle_earthquake(args: &[String], commands: &mut Commands) {
    if args.len() < 2 {
        warn!("Usage: earthquake circle <x>,<y> <radius>");
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
    commands.trigger(Earthquake {
        region: EarthquakeRegion::circle(center, radius),
    });
}

fn run_rect_earthquake(args: &[String], commands: &mut Commands) {
    if args.len() < 2 {
        warn!("Usage: earthquake rect <x>,<y> <w>,<h> [degrees]");
        return;
    }

    let center = match parse_position::<Vec2>(&args[0..1]) {
        Ok(c) => c,
        Err(e) => {
            warn!("Invalid rectangle center: {e}");
            return;
        }
    };

    let size = match parse_position::<Vec2>(&args[1..2]) {
        Ok(s) if s.x > 0.0 && s.y > 0.0 => s,
        _ => {
            warn!("Invalid rectangle size: width and height must be positive");
            return;
        }
    };

    let rotation = match args.get(2) {
        Some(value) => match value.parse::<f32>() {
            Ok(degrees) => degrees.to_radians(),
            Err(_) => {
                warn!("Invalid rectangle rotation: must be degrees");
                return;
            }
        },
        None => 0.0,
    };

    info!("Triggering rectangular earthquake at {center} with size {size}");
    commands.trigger(Earthquake {
        region: EarthquakeRegion::rect(center, size * 0.5, rotation),
    });
}

fn run_polygon_earthquake(args: &[String], commands: &mut Commands) {
    if args.len() < 3 {
        warn!("Usage: earthquake poly <x1>,<y1> <x2>,<y2> <x3>,<y3> ...");
        return;
    }

    let mut vertices = Vec::with_capacity(args.len());
    for arg in args {
        match parse_position::<Vec2>(std::slice::from_ref(arg)) {
            Ok(vertex) => vertices.push(vertex),
            Err(e) => {
                warn!("Invalid polygon vertex: {e}");
                return;
            }
        }
    }

    info!(
        "Triggering polygon earthquake with {} vertices",
        vertices.len()
    );
    commands.trigger(Earthquake {
        region: EarthquakeRegion::polygon(vertices),
    });
}
