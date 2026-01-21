mod commands;

use std::collections::VecDeque;

use bevy::{platform::collections::HashMap, prelude::*};

pub use commands::*;
use shlex::Shlex;
use trie_rs::{Trie, TrieBuilder};

use crate::directive::{Directive, DirectiveNode, DirectiveQueued};

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(CommandsPlugin)
            .insert_resource(ConsoleCache::default())
            .add_systems(
                Update,
                rebuild_console_cache.run_if(resource_changed::<ConsoleConfiguration>),
            );
    }
}

/// Rebuild the trie as new console commands are added or removed.
fn rebuild_console_cache(mut cache: ResMut<ConsoleCache>, config: Res<ConsoleConfiguration>) {
    cache.rebuild_tries(&config);
}

#[derive(Resource, Default)]
pub struct ConsoleCache {
    pub context_tries: HashMap<Vec<String>, Trie<u8>>,
}

impl ConsoleCache {
    pub fn rebuild_tries(&mut self, config: &ConsoleConfiguration) {
        self.context_tries.clear();
        self.build_context_tries_recursive(&[], config);
    }

    fn build_context_tries_recursive(
        &mut self,
        current_path: &[String],
        config: &ConsoleConfiguration,
    ) {
        let completions = self.get_context_completions(current_path, config);
        if !completions.is_empty() {
            let mut builder: TrieBuilder<u8> = TrieBuilder::new();
            for completion in &completions {
                builder.push(completion.as_bytes());
            }
            self.context_tries
                .insert(current_path.to_vec(), builder.build());
        }

        for completion in completions {
            let mut next_path = current_path.to_vec();
            next_path.push(completion);
            self.build_context_tries_recursive(&next_path, config);
        }
    }

    fn get_context_completions(
        &self,
        context_path: &[String],
        config: &ConsoleConfiguration,
    ) -> Vec<String> {
        if context_path.is_empty() {
            config.command_tree.keys().cloned().collect()
        } else {
            config
                .command_tree
                .get(&context_path[0])
                .and_then(|root_node| root_node.get_node(&context_path[1..]))
                .map(|node| node.get_completions())
                .unwrap_or_default()
        }
    }
}

#[derive(Resource)]
pub struct ConsoleConfiguration {
    pub command_tree: HashMap<String, DirectiveNode>,
    pub history_size: usize,
    pub symbol: String,
}

impl Default for ConsoleConfiguration {
    fn default() -> Self {
        Self {
            command_tree: HashMap::new(),
            history_size: 20,
            symbol: "> ".to_owned(),
        }
    }
}

impl ConsoleConfiguration {
    pub fn register_directive<T: Directive + Default>(&mut self) {
        let directive = T::default();
        let name = directive.name().to_string();
        let command_node = directive.build_directive_node();
        self.command_tree.insert(name, command_node);
    }
}

#[derive(Resource)]
pub struct ConsoleState {
    pub messages: Vec<String>,
    pub input: String,
    pub history: VecDeque<String>,
    pub history_index: usize,
    pub expanded: bool,
    pub height: f32,
    pub suggestions: Vec<String>,
    pub suggestion_index: Option<usize>,
    pub initial_focus: bool,
    pub user_typed_input: String,
    pub in_completion_mode: bool,
    pub needs_cursor_at_end: bool,
    pub request_focus_and_cursor: bool,
}

impl Default for ConsoleState {
    fn default() -> Self {
        let mut state = Self {
            messages: Vec::new(),
            input: String::new(),
            history: VecDeque::from([String::new()]),
            history_index: 0,
            expanded: false,
            height: 300.0,
            suggestions: Vec::new(),
            suggestion_index: None,
            initial_focus: false,
            user_typed_input: String::new(),
            in_completion_mode: false,
            needs_cursor_at_end: false,
            request_focus_and_cursor: false,
        };

        state.add_message("--- Bevy Falling Sand Editor Console ---".to_string());
        state.add_message("Console ready. Type 'help' for available commands.".to_string());

        state
    }
}

impl ConsoleState {
    pub fn toggle(&mut self) {
        self.expanded = !self.expanded;
    }

    pub fn add_message(&mut self, message: String) {
        self.messages.push(message);
    }

    pub fn handle_tab_completion(&mut self) {
        if self.suggestions.is_empty() {
            return;
        }

        if !self.in_completion_mode {
            self.user_typed_input = self.input.clone();
            self.in_completion_mode = true;
            self.suggestion_index = Some(0);
        } else {
            if let Some(index) = self.suggestion_index {
                let next_index = (index + 1) % self.suggestions.len();
                self.suggestion_index = Some(next_index);
            } else {
                self.suggestion_index = Some(0);
            }
        }

        if let Some(index) = self.suggestion_index {
            if let Some(suggestion) = self.suggestions.get(index).cloned() {
                self.apply_suggestion(&suggestion);
                self.needs_cursor_at_end = true;
            }
        }
    }

    fn apply_suggestion(&mut self, suggestion: &str) {
        self.input.clear();

        let user_input = &self.user_typed_input;

        if user_input.is_empty() {
            self.input = suggestion.to_string();
            return;
        }

        if user_input.ends_with(' ') {
            self.input = format!("{}{}", user_input, suggestion);
        } else {
            let words: Vec<&str> = user_input.trim().split_whitespace().collect();

            if words.len() == 1 {
                self.input = suggestion.to_string();
            } else {
                let mut complete_words = words[..words.len() - 1].to_vec();
                complete_words.push(suggestion);
                self.input = complete_words.join(" ");
            }
        }
    }

    pub fn commit_completion(&mut self) {
        self.in_completion_mode = false;
        self.user_typed_input.clear();
        self.suggestions.clear();
        self.suggestion_index = None;
        self.needs_cursor_at_end = true;
    }

    pub fn on_input_changed(&mut self) {
        if self.in_completion_mode {
            self.commit_completion();
        }
        self.history_index = 0;
        self.needs_cursor_at_end = false;
    }

    pub fn execute_command(
        &mut self,
        command: String,
        config: &ConsoleConfiguration,
        command_writer: &mut MessageWriter<DirectiveQueued>,
    ) {
        if command.trim().is_empty() {
            return;
        }

        self.add_message(format!("{}{}", config.symbol, command));
        self.history.insert(1, command.clone());
        if self.history.len() > config.history_size + 1 {
            self.history.pop_back();
        }
        self.history_index = 0;

        let args = Shlex::new(&command).collect::<Vec<_>>();
        if !args.is_empty() {
            let (directive_path, remaining_args) = self.find_command_path(&args, config);

            if !directive_path.is_empty() {
                if let Some(root_node) = config.command_tree.get(&directive_path[0]) {
                    if let Some(node) = root_node.get_node(&directive_path[1..]) {
                        if node.is_executable {
                            self.add_message(format!(
                                "Executing command: {}",
                                directive_path.join(" ")
                            ));
                            command_writer.write(DirectiveQueued {
                                directive_path,
                                args: remaining_args,
                            });
                            return;
                        } else {
                            self.add_message(format!(
                                "Executing command: {}",
                                directive_path.join(" ")
                            ));
                            command_writer.write(DirectiveQueued {
                                directive_path,
                                args: remaining_args,
                            });
                            return;
                        }
                    }
                }

                let directive_name = &directive_path[0];
                if config.command_tree.contains_key(directive_name) {
                    self.add_message(format!("Executing command: {}", directive_name));
                    command_writer.write(DirectiveQueued {
                        directive_path: vec![directive_name.clone()],
                        args: args[1..].to_vec(),
                    });
                } else {
                    self.add_message(format!("error: Unknown command '{}'", directive_name));
                    self.list_available_commands(config);
                }
            } else {
                self.add_message("error: Empty command".to_string());
                self.list_available_commands(config);
            }
        }
    }

    fn find_command_path(
        &self,
        args: &[String],
        config: &ConsoleConfiguration,
    ) -> (Vec<String>, Vec<String>) {
        if args.is_empty() {
            return (vec![], vec![]);
        }

        let first_arg = &args[0];
        if let Some(root_node) = config.command_tree.get(first_arg) {
            let mut path = vec![first_arg.clone()];
            let mut current_node = root_node;
            let mut arg_index = 1;

            while arg_index < args.len() {
                if let Some(child) = current_node.children.get(&args[arg_index]) {
                    path.push(args[arg_index].clone());
                    current_node = child;
                    arg_index += 1;
                } else {
                    break;
                }
            }

            (path, args[arg_index..].to_vec())
        } else {
            (vec![first_arg.clone()], args[1..].to_vec())
        }
    }

    fn list_available_commands(&mut self, config: &ConsoleConfiguration) {
        if !config.command_tree.is_empty() {
            let commands: Vec<String> = config.command_tree.keys().cloned().collect();
            self.add_message(format!("Available commands: {}", commands.join(", ")));
        } else {
            self.add_message("Available commands: help, clear, echo".to_string());
        }
    }

    pub fn navigate_history(&mut self, up: bool) {
        if self.history.len() <= 1 {
            return;
        }

        if up && self.history_index < self.history.len() - 1 {
            if self.history_index == 0 && !self.input.trim().is_empty() {
                *self.history.get_mut(0).unwrap() = self.input.clone();
            }
            self.history_index += 1;
            self.input = self.history.get(self.history_index).unwrap().clone();
            self.needs_cursor_at_end = true;
        } else if !up && self.history_index > 0 {
            self.history_index -= 1;
            self.input = self.history.get(self.history_index).unwrap().clone();
            self.needs_cursor_at_end = true;
        }
    }

    fn parse_command_context(
        &self,
        words: &[&str],
        config: &ConsoleConfiguration,
        input: &str,
    ) -> (Vec<String>, String) {
        if words.is_empty() {
            return (vec![], String::new());
        }

        let word_strings: Vec<String> = words.iter().map(|s| s.to_string()).collect();

        let input_ends_with_space = input.ends_with(' ');

        if words.len() == 1 && !input_ends_with_space {
            return (vec![], words[0].to_string());
        }

        let first_word = &word_strings[0];
        if let Some(root_node) = config.command_tree.get(first_word) {
            let mut context_path = vec![first_word.clone()];
            let mut current_node = root_node;
            let mut word_index = 1;

            if words.len() == 1 && input_ends_with_space {
                return (context_path, String::new());
            }

            let max_word_index = if input_ends_with_space {
                word_strings.len()
            } else {
                word_strings.len() - 1
            };

            while word_index < max_word_index {
                if let Some(child) = current_node.children.get(&word_strings[word_index]) {
                    context_path.push(word_strings[word_index].clone());
                    current_node = child;
                    word_index += 1;
                } else {
                    break;
                }
            }

            let partial_word = if input_ends_with_space {
                String::new()
            } else if word_index < word_strings.len() {
                word_strings[word_index].clone()
            } else {
                String::new()
            };

            (context_path, partial_word)
        } else {
            (vec![], words[0].to_string())
        }
    }

    fn get_all_completions_for_context(
        &self,
        context_path: &[String],
        config: &ConsoleConfiguration,
    ) -> Vec<String> {
        if context_path.is_empty() {
            return config.command_tree.keys().cloned().collect();
        }

        if let Some(root_node) = config.command_tree.get(&context_path[0]) {
            if let Some(node) = root_node.get_node(&context_path[1..]) {
                return node.get_completions();
            }
        }

        vec![]
    }
}
