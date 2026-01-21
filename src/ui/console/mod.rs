mod commands;
mod console_meta;

use bevy::prelude::*;

use bevy_egui::egui;

use crate::directive::DirectiveQueued;

pub use commands::*;
pub use console_meta::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CommandsPlugin, ConsoleMetaPlugin));
    }
}

pub struct Console;

impl Console {
    pub fn render(
        &self,
        ui: &mut egui::Ui,
        console_state: &mut ConsoleState,
        cache: &ConsoleCache,
        config: &ConsoleConfiguration,
        msgw_directed_queued: &mut MessageWriter<DirectiveQueued>,
    ) {
    }
}

fn calculate_completed_input(current_input: &str, suggestion: &str) -> String {
    if current_input.is_empty() {
        return suggestion.to_string();
    }

    if current_input.ends_with(' ') {
        format!("{}{}", current_input, suggestion)
    } else {
        let words: Vec<&str> = current_input.trim().split_whitespace().collect();

        if words.len() == 1 {
            suggestion.to_string()
        } else {
            let mut complete_words = words[..words.len() - 1].to_vec();
            complete_words.push(suggestion);
            complete_words.join(" ")
        }
    }
}
