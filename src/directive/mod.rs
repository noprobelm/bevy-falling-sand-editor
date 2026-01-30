use bevy::{platform::collections::HashMap, prelude::*};
use shlex::Shlex;

pub struct DirectivePlugin;

impl Plugin for DirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DirectiveQueued>()
            .add_systems(Update, msgr_directive_queued);
    }
}

/// Core trait for implementing directives.
pub trait Directive: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    /// Return subdirectives if this directive has them.
    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        vec![]
    }

    /// Execute this directive with the given arguments.
    fn run(&self, _args: &[String], _commands: &mut Commands) {}
}

/// Message to queue a directive for execution.
#[derive(Message, Default, Debug)]
pub struct DirectiveQueued {
    pub input: String,
}

/// Stores registered directives as a tree structure.
#[derive(Resource, Default)]
pub struct DirectiveRegistry {
    directives: HashMap<String, DirectiveNode>,
}

impl DirectiveRegistry {
    pub fn register(&mut self, directive: impl Directive) {
        let node = build_node(Box::new(directive));
        self.directives.insert(node.name.clone(), node);
    }

    pub fn get(&self, name: &str) -> Option<&DirectiveNode> {
        self.directives.get(name)
    }

    pub fn root_names(&self) -> impl Iterator<Item = &String> {
        self.directives.keys()
    }

    pub fn directives(&self) -> &HashMap<String, DirectiveNode> {
        &self.directives
    }
}

type RunFn = Box<dyn Fn(&[String], &mut Commands) + Send + Sync>;

/// Node for command tree
#[derive(Default)]
pub struct DirectiveNode {
    pub name: String,
    pub description: String,
    pub children: HashMap<String, DirectiveNode>,
    run_fn: Option<RunFn>,
}

impl DirectiveNode {
    /// Get completions at this level of the tree.
    pub fn completions(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }

    /// Navigate to a child node by path.
    pub fn get_node(&self, path: &[String]) -> Option<&DirectiveNode> {
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

fn build_node(directive: Box<dyn Directive>) -> DirectiveNode {
    let name = directive.name().to_string();
    let description = directive.description().to_string();

    let mut children = HashMap::new();
    for sub in directive.subdirectives() {
        let child = build_node(sub);
        children.insert(child.name.clone(), child);
    }

    let run_fn: RunFn = Box::new(move |args, commands| {
        directive.run(args, commands);
    });

    DirectiveNode {
        name,
        description,
        children,
        run_fn: Some(run_fn),
    }
}

fn msgr_directive_queued(
    mut messages: MessageReader<DirectiveQueued>,
    registry: Res<DirectiveRegistry>,
    mut commands: Commands,
) {
    for msg in messages.read() {
        execute(&msg.input, &registry, &mut commands);
    }
}

fn execute(input: &str, registry: &DirectiveRegistry, commands: &mut Commands) {
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

fn run_node(node: &DirectiveNode, args: &[String], commands: &mut Commands) {
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
