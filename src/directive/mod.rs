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

/// Stores registered directives and their tree structure for completions.
#[derive(Resource, Default)]
pub struct DirectiveRegistry {
    directives: HashMap<String, Box<dyn Directive>>,
    tree: HashMap<String, DirectiveNode>,
}

impl DirectiveRegistry {
    pub fn register(&mut self, directive: impl Directive) {
        let name = directive.name().to_string();
        self.tree.insert(name.clone(), build_node(&directive));
        self.directives.insert(name, Box::new(directive));
    }

    pub fn get(&self, name: &str) -> Option<&dyn Directive> {
        self.directives.get(name).map(|d| d.as_ref())
    }

    pub fn root_names(&self) -> impl Iterator<Item = &String> {
        self.directives.keys()
    }

    pub fn tree(&self) -> &HashMap<String, DirectiveNode> {
        &self.tree
    }
}

/// Node for command tree (used for help and completions).
#[derive(Clone, Default, Debug)]
pub struct DirectiveNode {
    pub name: String,
    pub description: String,
    pub children: HashMap<String, DirectiveNode>,
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
}

fn build_node(directive: &dyn Directive) -> DirectiveNode {
    let mut node = DirectiveNode {
        name: directive.name().to_string(),
        description: directive.description().to_string(),
        children: HashMap::new(),
    };
    for sub in directive.subdirectives() {
        let child = build_node(sub.as_ref());
        node.children.insert(child.name.clone(), child);
    }
    node
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

    let Some(root) = registry.get(&tokens[0]) else {
        error!("Unknown command: {}", tokens[0]);
        return;
    };

    run_directive(root, &tokens[1..], commands);
}

fn run_directive(directive: &dyn Directive, args: &[String], commands: &mut Commands) {
    let subs = directive.subdirectives();

    // If subdirectives exist and first arg matches one, recurse
    if !subs.is_empty() && !args.is_empty() {
        if let Some(sub) = subs.iter().find(|s| s.name() == args[0]) {
            run_directive(sub.as_ref(), &args[1..], commands);
            return;
        }
    }

    // Run this directive (either leaf or no matching subdirective)
    directive.run(args, commands);
}
