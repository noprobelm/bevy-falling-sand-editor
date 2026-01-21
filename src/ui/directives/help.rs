use bevy::prelude::*;

use crate::directive::Directive;

pub struct HelpDirectivePlugin;

impl Plugin for HelpDirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_show_help);
    }
}

#[derive(Event)]
pub struct ShowHelpMessage {
    pub target_command: Option<String>,
}

#[derive(Default)]
pub struct HelpDirective;

impl Directive for HelpDirective {
    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Display help information for directives"
    }

    fn execute_directive(&self, args: &[String], commands: &mut Commands) {
        let target_command = args.first().cloned();
        commands.trigger(ShowHelpMessage { target_command });
    }
}

fn on_show_help(_trigger: On<ShowHelpMessage>) {
    info!("A help command was sent!");
}
