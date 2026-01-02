use bevy::prelude::*;
use bevy_egui::EguiContextSettings;

use super::super::core::{ConsoleCommand, PrintConsoleLine};

pub struct UiCommandPlugin;

impl Plugin for UiCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_ui_scale_event);
    }
}

#[derive(Clone, Event, Debug, PartialEq, PartialOrd)]
pub struct SetUiScaleEvent {
    pub scale_factor: f32,
}

fn on_set_ui_scale_event(
    trigger: On<SetUiScaleEvent>,
    //ui_scale_factor: Res<UiScaleFactor>,
    egui_context: Single<&mut EguiContextSettings>,
) {
    let ev = trigger.event();
    let mut egui_settings = egui_context.into_inner();
    egui_settings.scale_factor = ev.scale_factor;
}

#[derive(Default)]
pub struct UiCommand;

impl ConsoleCommand for UiCommand {
    fn name(&self) -> &'static str {
        "ui"
    }

    fn description(&self) -> &'static str {
        "Control and view UI settings"
    }

    fn subcommand_types(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(Set)]
    }
}

#[derive(Default)]
pub struct Set;

impl ConsoleCommand for Set {
    fn name(&self) -> &'static str {
        "set"
    }

    fn description(&self) -> &'static str {
        "Set some element in the UI to another value"
    }

    fn subcommand_types(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(Scale)]
    }
}

#[derive(Default)]
pub struct Scale;

impl ConsoleCommand for Scale {
    fn name(&self) -> &'static str {
        "scale"
    }

    fn description(&self) -> &'static str {
        "Set the UI scale factor"
    }

    fn execute_action(
        &self,
        args: &[String],
        console_writer: &mut MessageWriter<PrintConsoleLine>,
        commands: &mut Commands,
    ) {
        if args.is_empty() {
            console_writer.write(PrintConsoleLine::new(
                "Error: scale value required (usage: ui set scale <value>)".to_string(),
            ));
            return;
        }

        match args[0].parse::<f32>() {
            Ok(scale_factor) => {
                if scale_factor == 0. {
                    console_writer.write(PrintConsoleLine::new(
                        "Error: scale factor must be greater than 0".to_string(),
                    ));
                } else {
                    console_writer.write(PrintConsoleLine::new(format!(
                        "Setting scale factor to {}",
                        scale_factor
                    )));
                    commands.trigger(SetUiScaleEvent { scale_factor })
                }
            }
            Err(_) => {
                console_writer.write(PrintConsoleLine::new(format!(
                    "Error: '{}' is not a valid scale value",
                    args[0]
                )));
            }
        }

        console_writer.write(PrintConsoleLine::new(
            "Executing ui set scale...".to_string(),
        ));
    }
}
