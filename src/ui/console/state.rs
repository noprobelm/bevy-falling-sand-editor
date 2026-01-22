use bevy::{platform::collections::HashMap, prelude::*};

use trie_rs::{Trie, TrieBuilder};

use crate::{
    directive::{Directive, DirectiveNode},
    ui::LogCapture,
};

pub struct ConsoleMetaPlugin;

impl Plugin for ConsoleMetaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                rebuild_console_cache.run_if(resource_changed::<ConsoleConfiguration>),
                update_information_area,
            ),
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

#[derive(Resource, Default)]
pub struct ConsoleState {
    pub information_area: InformationAreaState,
    pub prompt: PromptState,
}

pub struct InformationAreaState {
    pub is_open: bool,
    pub history: Vec<String>,
}

impl Default for InformationAreaState {
    fn default() -> Self {
        Self {
            is_open: true,
            history: vec![],
        }
    }
}

#[derive(Default)]
pub struct PromptState {
    pub input_text: String,
}

fn update_information_area(mut console_state: ResMut<ConsoleState>, log_capture: Res<LogCapture>) {
    for log in log_capture.drain() {
        console_state.information_area.history.push(log);
    }
}
