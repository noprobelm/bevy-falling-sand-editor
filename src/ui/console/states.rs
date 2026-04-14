use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
};

use bevy::{platform::collections::HashMap, prelude::*};
use trie_rs::{Trie, TrieBuilder};

use crate::{console_command::ConsoleCommandRegistry, ui::LogCapture};

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                rebuild_console_cache.run_if(resource_changed::<ConsoleCommandRegistry>),
                update_information_area,
            ),
        );
    }
}

/// Rebuild the trie when console commands change.
fn rebuild_console_cache(mut cache: ResMut<ConsoleCache>, registry: Res<ConsoleCommandRegistry>) {
    cache.rebuild_tries(&registry);
}

#[derive(Resource, Default)]
pub struct ConsoleCache {
    pub context_tries: HashMap<Vec<String>, Trie<u8>>,
}

impl ConsoleCache {
    pub fn rebuild_tries(&mut self, registry: &ConsoleCommandRegistry) {
        self.context_tries.clear();
        self.build_context_tries_recursive(&[], registry);
    }

    fn build_context_tries_recursive(
        &mut self,
        current_path: &[String],
        registry: &ConsoleCommandRegistry,
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
        registry: &ConsoleCommandRegistry,
    ) -> Vec<String> {
        if context_path.is_empty() {
            registry.root_names().cloned().collect()
        } else {
            registry
                .commands()
                .get(&context_path[0])
                .and_then(|root_node| root_node.get_node(&context_path[1..]))
                .map(|node| node.completions())
                .unwrap_or_default()
        }
    }
}

#[derive(Resource, Default)]
pub struct ConsoleInformationAreaState {
    pub is_open: bool,
    pub log_history: Vec<String>,
}

#[derive(Resource)]
pub struct ConsolePromptState {
    pub input_text: String,
    pub request_focus: bool,
    pub surrender_focus: bool,
}

impl Default for ConsolePromptState {
    fn default() -> Self {
        ConsolePromptState {
            input_text: String::new(),
            request_focus: false,
            surrender_focus: true,
        }
    }
}

const CMDS_LOG_FILE: &str = "cmds.log";

#[derive(Resource)]
pub struct CommandHistory {
    entries: Vec<String>,
    cursor: Option<usize>,
    draft: String,
    path: PathBuf,
}

impl CommandHistory {
    pub fn load(config_path: &Path) -> Self {
        let path = config_path.join(CMDS_LOG_FILE);
        let entries = if path.exists() {
            let file = fs::File::open(&path).unwrap_or_else(|e| {
                panic!("Failed to open command history file {:?}: {}", path, e)
            });
            BufReader::new(file)
                .lines()
                .map_while(Result::ok)
                .filter(|line| !line.is_empty())
                .collect()
        } else {
            Vec::new()
        };

        Self {
            entries,
            cursor: None,
            draft: String::new(),
            path,
        }
    }

    pub fn push(&mut self, command: String) {
        if self.entries.last() != Some(&command) {
            self.entries.push(command);
        }
        self.cursor = None;
        self.save();
    }

    pub fn navigate_up(&mut self, current_input: &str) -> Option<&str> {
        if self.entries.is_empty() {
            return None;
        }

        let new_cursor = match self.cursor {
            None => {
                self.draft = current_input.to_string();
                self.entries.len() - 1
            }
            Some(0) => return None,
            Some(c) => c - 1,
        };

        self.cursor = Some(new_cursor);
        Some(&self.entries[new_cursor])
    }

    pub fn navigate_down(&mut self) -> &str {
        match self.cursor {
            None => &self.draft,
            Some(c) if c + 1 >= self.entries.len() => {
                self.cursor = None;
                &self.draft
            }
            Some(c) => {
                self.cursor = Some(c + 1);
                &self.entries[c + 1]
            }
        }
    }

    fn save(&self) {
        let mut file = fs::File::create(&self.path).unwrap_or_else(|e| {
            panic!(
                "Failed to create command history file {:?}: {}",
                self.path, e
            )
        });
        for entry in &self.entries {
            writeln!(file, "{}", entry).unwrap_or_else(|e| {
                panic!(
                    "Failed to write to command history file {:?}: {}",
                    self.path, e
                )
            });
        }
    }
}

fn update_information_area(
    mut information_area: ResMut<ConsoleInformationAreaState>,
    log_capture: Res<LogCapture>,
) {
    for log in log_capture.drain() {
        information_area.log_history.push(log);
    }
}
