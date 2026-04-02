use bevy::prelude::*;

use crate::console_command::{ConsoleCommand, ConsoleCommandNode, ConsoleCommandRegistry};

pub struct HelpConsoleCommandPlugin;

impl Plugin for HelpConsoleCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_show_help);
    }
}

#[derive(Event)]
pub struct ShowHelpEvent {
    pub target_command: Option<String>,
}

#[derive(Default, Debug)]
pub struct HelpConsoleCommand;

impl ConsoleCommand for HelpConsoleCommand {
    fn name(&self) -> &'static str {
        "help"
    }

    fn description(&self) -> &'static str {
        "Display help information for console commands"
    }

    fn run(&self, args: &[String], commands: &mut Commands) {
        let target_command = args.first().cloned();
        commands.trigger(ShowHelpEvent { target_command });
    }
}

fn on_show_help(trigger: On<ShowHelpEvent>, registry: Res<ConsoleCommandRegistry>) {
    let event = trigger.event();

    if let Some(target_cmd) = &event.target_command {
        if let Some(root_node) = registry.commands().get(target_cmd) {
            show_command_tree_help(root_node, vec![target_cmd.clone()]);
        } else {
            error!("Console command '{}' does not exist", target_cmd);
        }
    } else {
        info!("Available console commands:");
        for (name, node) in registry.commands() {
            let mut line = format!("  {} - {}", name, node.description);
            if !node.children.is_empty() {
                let subs: Vec<_> = node.children.keys().cloned().collect();
                line.push_str(&format!(" (subcommands: {})", subs.join(", ")));
            }
            info!("{}", line);
        }
    }
}

fn show_command_tree_help(node: &ConsoleCommandNode, path: Vec<String>) {
    info!("{} - {}", path.join(" "), node.description);

    if !node.children.is_empty() {
        info!("  Subcommands:");
        for (name, child) in &node.children {
            info!("    {} - {}", name, child.description);
        }
    }
}
