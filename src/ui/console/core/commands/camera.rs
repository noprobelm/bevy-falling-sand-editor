use bevy::prelude::*;
use thiserror::Error;

use crate::camera::{MainCamera, ZoomSpeed, ZoomTarget};

use super::super::core::{ConsoleCommand, PrintConsoleLine};

pub struct CameraCommandPlugin;

impl Plugin for CameraCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_camera_reset)
            .add_observer(on_camera_set_position)
            .add_observer(on_camera_set_scale);
    }
}

#[derive(Debug, Error)]
enum PositionParseError {
    #[error("invalid position {0}.")]
    Invalid(String),
}

fn parse_position(position: &[String]) -> Result<Vec2, PositionParseError> {
    let parse_coord = |s: &str| -> Result<f32, PositionParseError> {
        let filtered: String = s.chars().filter(|c| c.is_numeric() || *c == '.').collect();
        filtered
            .parse::<f32>()
            .map_err(|_| PositionParseError::Invalid(s.to_string()))
    };

    let (x_str, y_str) = if let Some((x, y)) = position.first().and_then(|s| s.split_once(',')) {
        (x.to_string(), y.to_string())
    } else if position.len() >= 2 {
        (position[0].clone(), position[1].clone())
    } else {
        return Err(PositionParseError::Invalid(position.join(" ")));
    };

    let x = parse_coord(&x_str)?;
    let y = parse_coord(&y_str)?;

    Ok(Vec2::new(x, y))
}

#[derive(Event)]
pub struct CameraResetEvent;

#[derive(Event)]
pub struct CameraSetPositionEvent(pub Vec2);

#[derive(Event)]
pub struct CameraSetScaleEvent(pub f32);

#[derive(Default)]
pub struct CameraCommand;

impl ConsoleCommand for CameraCommand {
    fn name(&self) -> &'static str {
        "camera"
    }

    fn description(&self) -> &'static str {
        "Camera system operations"
    }

    fn subcommand_types(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(CameraResetCommand), Box::new(CameraSetCommand)]
    }
}

#[derive(Default)]
pub struct CameraResetCommand;

impl ConsoleCommand for CameraResetCommand {
    fn name(&self) -> &'static str {
        "reset"
    }

    fn description(&self) -> &'static str {
        "Reset camera position and zoom"
    }

    fn execute_action(
        &self,
        _args: &[String],
        console_writer: &mut MessageWriter<PrintConsoleLine>,
        commands: &mut Commands,
    ) {
        console_writer.write(PrintConsoleLine::new(
            "Onging camera reset command...".to_string(),
        ));
        commands.trigger(CameraResetEvent);
    }
}

#[derive(Default)]
pub struct CameraSetCommand;

impl ConsoleCommand for CameraSetCommand {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set camera parameters"
    }

    fn subcommand_types(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(CameraSetPositionCommand),
            Box::new(CameraSetScaleCommand),
        ]
    }
}

#[derive(Default)]
pub struct CameraSetPositionCommand;

impl ConsoleCommand for CameraSetPositionCommand {
    fn name(&self) -> &'static str {
        "position"
    }

    fn description(&self) -> &'static str {
        "Set camera position"
    }

    fn execute_action(
        &self,
        args: &[String],
        console_writer: &mut MessageWriter<PrintConsoleLine>,
        commands: &mut Commands,
    ) {
        console_writer.write(PrintConsoleLine::new(
            "Onging set camera position command...".to_string(),
        ));
        match parse_position(args) {
            Ok(position) => {
                commands.trigger(CameraSetPositionEvent(position));
            }
            Err(e) => {
                console_writer.write(PrintConsoleLine::new(e.to_string()));
            }
        };
    }
}

#[derive(Default)]
pub struct CameraSetScaleCommand;

impl ConsoleCommand for CameraSetScaleCommand {
    fn name(&self) -> &'static str {
        "scale"
    }

    fn description(&self) -> &'static str {
        "Set camera scale"
    }

    fn execute_action(
        &self,
        args: &[String],
        console_writer: &mut MessageWriter<PrintConsoleLine>,
        commands: &mut Commands,
    ) {
        console_writer.write(PrintConsoleLine::new(
            "Onging scale camera command...".to_string(),
        ));
        if let Some(s) = args.first() {
            match s.parse::<f32>() {
                Ok(scale) => {
                    commands.trigger(CameraSetScaleEvent(scale));
                }
                Err(_) => {
                    console_writer
                        .write(PrintConsoleLine::new(format!("Invalid scale value: {s}")));
                }
            }
        } else {
            console_writer.write(PrintConsoleLine::new(
                "No camera scale value passed!".to_string(),
            ));
        }
    }
}

fn on_camera_reset(
    _trigger: On<CameraResetEvent>,
    camera_query: Query<Entity, With<MainCamera>>,
    mut commands: Commands,
) -> Result {
    let initial_scale = 0.25;
    let entity = camera_query.single()?;
    commands.entity(entity).insert((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            near: -1000.0,
            scale: initial_scale,
            ..OrthographicProjection::default_2d()
        }),
        MainCamera,
        ZoomTarget {
            target_scale: initial_scale,
            current_scale: initial_scale,
        },
        ZoomSpeed(8.0),
        Transform::default(),
    ));
    Ok(())
}

fn on_camera_set_position(
    trigger: On<CameraSetPositionEvent>,
    mut camera_query: Query<&mut Transform, With<MainCamera>>,
) -> Result {
    let mut transform = camera_query.single_mut()?;
    transform.translation.x = trigger.event().0.x;
    transform.translation.y = trigger.event().0.y;
    Ok(())
}

fn on_camera_set_scale(
    trigger: On<CameraSetScaleEvent>,
    mut camera_query: Query<&mut ZoomTarget, With<MainCamera>>,
) -> Result {
    let mut zoom_target = camera_query.single_mut()?;
    zoom_target.target_scale = trigger.event().0;
    Ok(())
}
