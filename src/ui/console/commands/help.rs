use bevy::prelude::*;

use crate::{
    directive::{Directive, DirectiveNode},
    ui::ConsoleConfiguration,
};

pub struct HelpDirectivePlugin;

impl Plugin for HelpDirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_show_help);
    }
}

#[derive(Event)]
pub struct ShowHelpEvent {
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
        commands.trigger(ShowHelpEvent { target_command });
    }
}

fn on_show_help(trigger: On<ShowHelpEvent>, config: Res<ConsoleConfiguration>) {
    let event = trigger.event();

    if let Some(target_cmd) = &event.target_command {
        if let Some(root_node) = config.command_tree.get(target_cmd) {
            show_command_tree_help(root_node, vec![target_cmd.clone()]);
        } else {
            error!("Command '{target_cmd}' does not exist");
        }
    } else {
        let mut info_msg = String::from("Available commands:");

        for (name, node) in &config.command_tree {
            info_msg.push_str(format!("{} - {}", name, node.description).as_str());
            if !node.children.is_empty() {
                info_msg.push_str(
                    format!(
                        "\t(has subcommands: {}",
                        node.children.keys().cloned().collect::<Vec<_>>().join(", ")
                    )
                    .as_str(),
                );
            }
        }
        info!(info_msg);
    }
}

fn show_command_tree_help(node: &DirectiveNode, path: Vec<String>) {
    let mut info_msg = format!("{} - {}", path.join(" "), node.description);
    if node.is_executable {
        info_msg.push_str("\t(executable command)");
    }
    if !node.children.is_empty() {
        info_msg.push_str("\tSubcommands:");
        for (name, child) in &node.children {
            let child_path = format!("    {} - {}", name, child.description);
            info_msg.push_str(child_path.as_str());
            if child.is_executable {
                info_msg.push_str("\t(executable)");
            }
        }
    }
    info!(info_msg);
}
