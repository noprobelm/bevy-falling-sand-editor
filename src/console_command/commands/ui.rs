use crate::console_command::ConsoleCommand;
use crate::ui::{
    UiToggleCursorOverlaySignal, UiToggleParticleEditorSignal, UiToggleSettingsSignal,
    UiToggleSignal,
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
        vec![Box::new(ToggleUiConsoleCommand)]
    }
}

#[derive(Default)]
pub struct ToggleUiConsoleCommand;

impl ConsoleCommand for ToggleUiConsoleCommand {
    fn name(&self) -> &'static str {
        "toggle"
    }

    fn description(&self) -> &'static str {
        "Toggle Ui components"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        info!("Toggling UI");
        commands.trigger(UiToggleSignal);
    }

    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![
            Box::new(ToggleParticleEditorConsoleCommand),
            Box::new(ToggleSettingsConsoleCommand),
            Box::new(ToggleCursorOverlayConsoleCommand),
        ]
    }
}

#[derive(Default)]
pub struct ToggleParticleEditorConsoleCommand;

impl ConsoleCommand for ToggleParticleEditorConsoleCommand {
    fn name(&self) -> &'static str {
        "particle_editor"
    }

    fn description(&self) -> &'static str {
        "Toggle Particle Editor"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(UiToggleParticleEditorSignal);
    }
}

#[derive(Default)]
pub struct ToggleSettingsConsoleCommand;

impl ConsoleCommand for ToggleSettingsConsoleCommand {
    fn name(&self) -> &'static str {
        "settings"
    }

    fn description(&self) -> &'static str {
        "Toggle settings"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(UiToggleSettingsSignal);
    }
}

#[derive(Default)]
pub struct ToggleCursorOverlayConsoleCommand;

impl ConsoleCommand for ToggleCursorOverlayConsoleCommand {
    fn name(&self) -> &'static str {
        "cursor_overlay"
    }

    fn description(&self) -> &'static str {
        "Toggle Cursor Overlay"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(UiToggleCursorOverlaySignal);
    }
}
