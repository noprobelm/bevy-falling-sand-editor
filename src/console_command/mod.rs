use bevy::{platform::collections::HashMap, prelude::*};
use shlex::Shlex;

pub struct ConsoleCommandPlugin;

impl Plugin for ConsoleCommandPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<ConsoleCommandQueued>()
            .add_systems(Update, msgr_console_command_queued);
    }
}

/// Core trait for implementing console commands.
pub trait ConsoleCommand: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    /// Return subcommands if this console command has them.
    fn subcommands(&self) -> Vec<Box<dyn ConsoleCommand>> {
        vec![]
    }

    /// Execute this console command with the given arguments.
    fn run(&self, _args: &[String], _commands: &mut Commands) {}
}

/// Message to queue a console command for execution.
#[derive(Message, Default, Debug)]
pub struct ConsoleCommandQueued {
    pub input: String,
}

/// Stores registered console commands as a tree structure.
#[derive(Resource, Default)]
pub struct ConsoleCommandRegistry {
    commands: HashMap<String, ConsoleCommandNode>,
}

impl ConsoleCommandRegistry {
    pub fn register(&mut self, command: impl ConsoleCommand) {
        let node = build_node(Box::new(command));
        self.commands.insert(node.name.clone(), node);
    }

    pub fn get(&self, name: &str) -> Option<&ConsoleCommandNode> {
        self.commands.get(name)
    }

    pub fn root_names(&self) -> impl Iterator<Item = &String> {
        self.commands.keys()
    }

    pub fn commands(&self) -> &HashMap<String, ConsoleCommandNode> {
        &self.commands
    }
}

type RunFn = Box<dyn Fn(&[String], &mut Commands) + Send + Sync>;

/// Node for command tree
#[derive(Default)]
pub struct ConsoleCommandNode {
    pub name: String,
    pub description: String,
    pub children: HashMap<String, ConsoleCommandNode>,
    run_fn: Option<RunFn>,
}

impl ConsoleCommandNode {
    /// Get completions at this level of the tree.
    pub fn completions(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }

    /// Navigate to a child node by path.
    pub fn get_node(&self, path: &[String]) -> Option<&ConsoleCommandNode> {
        if path.is_empty() {
            return Some(self);
        }
        self.children.get(&path[0])?.get_node(&path[1..])
    }

    /// Execute this node's run function if it has one.
    pub fn run(&self, args: &[String], commands: &mut Commands) {
        if let Some(ref f) = self.run_fn {
            f(args, commands);
        }
    }
}

fn build_node(command: Box<dyn ConsoleCommand>) -> ConsoleCommandNode {
    let name = command.name().to_string();
    let description = command.description().to_string();

    let mut children = HashMap::new();
    for sub in command.subcommands() {
        let child = build_node(sub);
        children.insert(child.name.clone(), child);
    }

    let run_fn: RunFn = Box::new(move |args, commands| {
        command.run(args, commands);
    });

    ConsoleCommandNode {
        name,
        description,
        children,
        run_fn: Some(run_fn),
    }
}

fn msgr_console_command_queued(
    mut messages: MessageReader<ConsoleCommandQueued>,
    registry: Res<ConsoleCommandRegistry>,
    mut commands: Commands,
) {
    for msg in messages.read() {
        execute(&msg.input, &registry, &mut commands);
    }
}

fn execute(input: &str, registry: &ConsoleCommandRegistry, commands: &mut Commands) {
    let tokens: Vec<String> = Shlex::new(input).collect();
    if tokens.is_empty() {
        return;
    }

    let Some(node) = registry.get(&tokens[0]) else {
        error!("Unknown command: {}", tokens[0]);
        return;
    };

    run_node(node, &tokens[1..], commands);
}

fn run_node(node: &ConsoleCommandNode, args: &[String], commands: &mut Commands) {
    // If children exist and first arg matches one, recurse
    if !node.children.is_empty()
        && !args.is_empty()
        && let Some(child) = node.children.get(&args[0])
    {
        run_node(child, &args[1..], commands);
        return;
    }

    // Run this node (either leaf or no matching child)
    node.run(args, commands);
}
