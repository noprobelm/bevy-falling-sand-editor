use bevy::{platform::collections::HashMap, prelude::*};
use trie_rs::{Trie, TrieBuilder};

use crate::{directive::DirectiveRegistry, ui::LogCapture};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                rebuild_console_cache.run_if(resource_changed::<DirectiveRegistry>),
                update_information_area,
            ),
        );
    }
}

/// Rebuild the trie when directives change.
fn rebuild_console_cache(mut cache: ResMut<ConsoleCache>, registry: Res<DirectiveRegistry>) {
    cache.rebuild_tries(&registry);
}

#[derive(Resource, Default)]
pub struct ConsoleCache {
    pub context_tries: HashMap<Vec<String>, Trie<u8>>,
}

impl ConsoleCache {
    pub fn rebuild_tries(&mut self, registry: &DirectiveRegistry) {
        self.context_tries.clear();
        self.build_context_tries_recursive(&[], registry);
    }

    fn build_context_tries_recursive(
        &mut self,
        current_path: &[String],
        registry: &DirectiveRegistry,
    ) {
        let completions = self.get_context_completions(current_path, registry);
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
            self.build_context_tries_recursive(&next_path, registry);
        }
    }

    fn get_context_completions(
        &self,
        context_path: &[String],
        registry: &DirectiveRegistry,
    ) -> Vec<String> {
        if context_path.is_empty() {
            registry.root_names().cloned().collect()
        } else {
            registry
                .directives()
                .get(&context_path[0])
                .and_then(|root_node| root_node.get_node(&context_path[1..]))
                .map(|node| node.completions())
                .unwrap_or_default()
        }
    }
}

#[derive(Resource, Default)]
pub struct ConsoleState {
    pub information_area: InformationAreaState,
    pub prompt: PromptState,
}

pub struct InformationAreaState {
    pub is_open: bool,
    pub log_history: Vec<String>,
}

impl Default for InformationAreaState {
    fn default() -> Self {
        Self {
            is_open: true,
            log_history: vec![],
        }
    }
}

#[derive(Default)]
pub struct PromptState {
    pub input_text: String,
    pub request_focus: bool,
}

fn update_information_area(mut console_state: ResMut<ConsoleState>, log_capture: Res<LogCapture>) {
    for log in log_capture.drain() {
        console_state.information_area.log_history.push(log);
    }
}
