use crate::console_command::ConsoleCommand;
use crate::ui::{
    UiToggleCursorOverlayEvent, UiToggleParticleEditorEvent, UiToggleSettingsEvent,
    UiToggleEvent, UiToggleToolOptionsEvent,
};
use bevy::prelude::*;

#[derive(Default)]
pub struct UiConsoleCommand;

impl ConsoleCommand for UiConsoleCommand {
    fn name(&self) -> &'static str {
        "ui"
    }

    fn description(&self) -> &'static str {
        "Ui component management"
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![Box::new(UiToggleConsoleCommand)]
    }
}

#[derive(Default)]
pub struct UiToggleConsoleCommand;

impl ConsoleCommand for UiToggleConsoleCommand {
    fn name(&self) -> &'static str {
        "toggle"
    }

    fn description(&self) -> &'static str {
        "Toggle Ui components"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        if !args.is_empty() {
            warn!("Invalid argument");
            return;
        }
        commands.trigger(UiToggleEvent);
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(UiToggleParticleEditorConsoleCommand),
            Box::new(UiToggleSettingsConsoleCommand),
            Box::new(UiToggleCursorOverlayConsoleCommand),
            Box::new(UiToggleToolOptionsConsoleCommand),

        ]
    }
}

#[derive(Default)]
pub struct UiToggleParticleEditorConsoleCommand;

impl ConsoleCommand for UiToggleParticleEditorConsoleCommand {
    fn name(&self) -> &'static str {
        "particle_editor"
    }

    fn description(&self) -> &'static str {
        "Toggle Particle Editor"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(UiToggleParticleEditorEvent);
    }
}

#[derive(Default)]
pub struct UiToggleSettingsConsoleCommand;

impl ConsoleCommand for UiToggleSettingsConsoleCommand {
    fn name(&self) -> &'static str {
        "settings"
    }

    fn description(&self) -> &'static str {
        "Toggle settings"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(UiToggleSettingsEvent);
    }
}

#[derive(Default)]
pub struct UiToggleCursorOverlayConsoleCommand;

impl ConsoleCommand for UiToggleCursorOverlayConsoleCommand {
    fn name(&self) -> &'static str {
        "cursor_overlay"
    }

    fn description(&self) -> &'static str {
        "Toggle Cursor Overlay"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(UiToggleCursorOverlayEvent);
    }
}

#[derive(Default)]
pub struct UiToggleToolOptionsConsoleCommand;

impl ConsoleCommand for UiToggleToolOptionsConsoleCommand {
    fn name(&self) -> &'static str {
        "tool_options"
    }

    fn description(&self) -> &'static str {
        "Toggle Tool Options"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(UiToggleToolOptionsEvent);
    }
}
