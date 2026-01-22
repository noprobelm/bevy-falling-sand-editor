use std::slice::Iter;

use bevy::{platform::collections::HashMap, prelude::*};

pub struct DirectivePlugin;

impl Plugin for DirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DirectiveQueued>()
            .add_systems(Update, msgr_directive_queued);
    }
}

pub trait Directive: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn subdirective_types(&self) -> Vec<Box<dyn Directive>> {
        vec![]
    }

    fn execute_directive(&self, _args: &[String], _commands: &mut Commands) {}

    fn execute(&self, path: &[String], args: &[String], commands: &mut Commands) {
        let subdirectives = self.subdirective_types();

        let current_depth = path
            .iter()
            .position(|part| part == self.name())
            .unwrap_or(0);

        if current_depth + 1 >= path.len() {
            if subdirectives.is_empty() {
                self.execute_directive(args, commands);
            } else {
                error!("'{}' requires a subcommand", self.name());
                let subcmd_names: Vec<String> = subdirectives
                    .iter()
                    .map(|cmd| cmd.name().to_string())
                    .collect();
                info!("Available subcommands: {}", subcmd_names.join(", "));
            }
            return;
        }

        let next_directive = &path[current_depth + 1];
        for subdirective in subdirectives {
            if subdirective.name() == next_directive {
                subdirective.execute(path, args, commands);
                return;
            }
        }

        error!("Unknown subcommand '{} {}'", self.name(), next_directive);
    }

    #[allow(dead_code)]
    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        self.subdirective_types()
    }

    fn build_directive_node(&self) -> DirectiveNode {
        let mut node = DirectiveNode::new(self.name(), self.description());

        let subcommands = self.subdirective_types();
        if subcommands.is_empty() {
            node = node.executable();
        } else {
            for subdirective in subcommands {
                node = node.with_child(subdirective.build_directive_node());
            }
        }

        node
    }
}

#[derive(Message, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct DirectiveQueued {
    pub directive_path: Vec<String>,
    pub args: Vec<String>,
}

#[derive(Clone, Default, Eq, PartialEq, Debug, Reflect)]
pub struct DirectiveNode {
    pub name: String,
    pub description: String,
    pub children: HashMap<String, DirectiveNode>,
    pub is_executable: bool,
}

impl DirectiveNode {
    pub fn new(name: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            description: description.into(),
            children: HashMap::new(),
            is_executable: false,
        }
    }

    pub fn executable(mut self) -> Self {
        self.is_executable = true;
        self
    }

    pub fn with_child(mut self, child: DirectiveNode) -> Self {
        self.children.insert(child.name.clone(), child);
        self
    }

    pub fn get_node(&self, path: &[String]) -> Option<&DirectiveNode> {
        if path.is_empty() {
            return Some(self);
        }

        if let Some(child) = self.children.get(&path[0]) {
            child.get_node(&path[1..])
        } else {
            None
        }
    }

    pub fn get_args(&self, path: &[String]) -> Vec<String> {
        if path.is_empty() {
            return vec![];
        }

        if let Some(child) = self.children.get(&path[0]) {
            child.get_args(&path[1..])
        } else {
            path.to_vec()
        }
    }

    pub fn get_completions(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }
}

#[derive(Resource, Default)]
pub struct DirectiveRegistry {
    pub directives: Vec<Box<dyn Directive>>,
}

impl DirectiveRegistry {
    pub fn register<T: Directive + Default>(&mut self) {
        self.directives.push(Box::new(DirectiveWrapper::<T>::new()));
    }

    pub fn find_command(&self, name: &str) -> Option<&dyn Directive> {
        self.directives
            .iter()
            .find(|cmd| cmd.name() == name)
            .map(|cmd| cmd.as_ref())
    }

    pub fn iter(&self) -> Iter<Box<dyn Directive>> {
        self.directives.iter()
    }
}

struct DirectiveWrapper<T: Directive> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Directive> DirectiveWrapper<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Directive + Default> Directive for DirectiveWrapper<T> {
    fn name(&self) -> &'static str {
        T::default().name()
    }

    fn description(&self) -> &'static str {
        T::default().description()
    }

    fn execute(&self, path: &[String], args: &[String], commands: &mut Commands) {
        T::default().execute(path, args, commands);
    }

    fn subdirectives(&self) -> Vec<Box<dyn Directive>> {
        T::default().subdirectives()
    }

    fn build_directive_node(&self) -> DirectiveNode {
        T::default().build_directive_node()
    }
}

pub fn msgr_directive_queued(
    mut msgr_directive_queued: MessageReader<DirectiveQueued>,
    registry: Res<DirectiveRegistry>,
    mut commands: Commands,
) {
    for msg in msgr_directive_queued.read() {
        if msg.directive_path.is_empty() {
            continue;
        }

        let root_directive_name = &msg.directive_path[0];
        if let Some(command) = registry.find_command(root_directive_name) {
            command.execute(&msg.directive_path, &msg.args, &mut commands);
        }
    }
}
