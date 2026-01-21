use bevy::{platform::collections::HashMap, prelude::*};

pub struct DirectivePlugin;

impl Plugin for DirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<DirectiveQueued>()
            .add_systems(Update, msgr_console_command_queued);
    }
}

#[derive(Message, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct DirectiveQueued {
    pub command_path: Vec<String>,
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

    pub fn get_completions(&self) -> Vec<String> {
        self.children.keys().cloned().collect()
    }
}

pub trait Directive: Send + Sync + 'static {
    fn name(&self) -> &'static str;
    fn description(&self) -> &'static str;

    fn subcommand_types(&self) -> Vec<Box<dyn Directive>> {
        vec![]
    }

    fn execute_action(&self, _args: &[String], _commands: &mut Commands) {}

    fn execute(&self, path: &[String], args: &[String], commands: &mut Commands) {
        let subcommands = self.subcommand_types();

        let current_depth = path
            .iter()
            .position(|part| part == self.name())
            .unwrap_or(0);

        if current_depth + 1 >= path.len() {
            if subcommands.is_empty() {
                self.execute_action(args, commands);
            } else {
                error!("'{}' requires a subcommand", self.name());
                let subcmd_names: Vec<String> = subcommands
                    .iter()
                    .map(|cmd| cmd.name().to_string())
                    .collect();
                info!("Available subcommands: {}", subcmd_names.join(", "));
            }
            return;
        }

        let next_command = &path[current_depth + 1];
        for subcmd in subcommands {
            if subcmd.name() == next_command {
                subcmd.execute(path, args, commands);
                return;
            }
        }

        error!("Unknown subcommand '{} {}'", self.name(), next_command);
    }

    #[allow(dead_code)]
    fn subcommands(&self) -> Vec<Box<dyn Directive>> {
        self.subcommand_types()
    }

    fn build_command_node(&self) -> DirectiveNode {
        let mut node = DirectiveNode::new(self.name(), self.description());

        let subcommands = self.subcommand_types();
        if subcommands.is_empty() {
            node = node.executable();
        } else {
            for subcmd in subcommands {
                node = node.with_child(subcmd.build_command_node());
            }
        }

        node
    }
}

#[derive(Resource, Default)]
pub struct DirectiveRegistry {
    pub commands: Vec<Box<dyn Directive>>,
}

impl DirectiveRegistry {
    pub fn register<T: Directive + Default>(&mut self) {
        self.commands.push(Box::new(CommandWrapper::<T>::new()));
    }

    pub fn find_command(&self, name: &str) -> Option<&dyn Directive> {
        self.commands
            .iter()
            .find(|cmd| cmd.name() == name)
            .map(|cmd| cmd.as_ref())
    }
}

struct CommandWrapper<T: Directive> {
    _phantom: std::marker::PhantomData<T>,
}

impl<T: Directive> CommandWrapper<T> {
    fn new() -> Self {
        Self {
            _phantom: std::marker::PhantomData,
        }
    }
}

impl<T: Directive + Default> Directive for CommandWrapper<T> {
    fn name(&self) -> &'static str {
        T::default().name()
    }

    fn description(&self) -> &'static str {
        T::default().description()
    }

    fn execute(&self, path: &[String], args: &[String], commands: &mut Commands) {
        T::default().execute(path, args, commands);
    }

    fn subcommands(&self) -> Vec<Box<dyn Directive>> {
        T::default().subcommands()
    }

    fn build_command_node(&self) -> DirectiveNode {
        T::default().build_command_node()
    }
}

pub fn msgr_console_command_queued(
    mut cmd: MessageReader<DirectiveQueued>,
    registry: Res<DirectiveRegistry>,
    mut commands: Commands,
) {
    for command_message in cmd.read() {
        if command_message.command_path.is_empty() {
            continue;
        }

        let root_command_name = &command_message.command_path[0];
        if let Some(command) = registry.find_command(root_command_name) {
            command.execute(
                &command_message.command_path,
                &command_message.args,
                &mut commands,
            );
        }
    }
}
