mod commands;
mod state;

use bevy::prelude::*;

use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use leafwing_input_manager::prelude::ActionState;

use crate::{directive::DirectiveQueued, setup::ConsoleAction};

pub use commands::*;
pub use state::*;

pub struct ConsolePlugin;

impl Plugin for ConsolePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CommandsPlugin, ConsoleMetaPlugin));
        app.add_systems(EguiPrimaryContextPass, show_console);
    }
}

fn show_console(
    mut contexts: EguiContexts,
    msgw_directive_queued: MessageWriter<DirectiveQueued>,
    mut console_state: ResMut<ConsoleState>,
    mut action_state: Single<&ActionState<ConsoleAction>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    if action_state.just_pressed(&ConsoleAction::ToggleInformationArea) {
        console_state.information_area.is_open = !console_state.information_area.is_open;
    }
    egui::TopBottomPanel::top("information_area").show_animated(
        ctx,
        console_state.information_area.is_open,
        |ui| {
            information_area_ui(ui);
        },
    );
    egui::TopBottomPanel::top("console").show(ctx, |ui| {
        prompt_ui(ui);
    });
    Ok(())
}

fn information_area_ui(ui: &mut egui::Ui) {
    let foo = vec!["This", "Is", "A", "Few", "Lines"];
    egui::ScrollArea::vertical()
        .stick_to_bottom(true) // auto-scroll to new messages
        .show(ui, |ui| {
            for msg in foo {
                ui.label(msg);
            }
        });
}

fn prompt_ui(ui: &mut egui::Ui) {
    ui.add(egui::TextEdit::singleline(&mut String::new()).desired_width(ui.available_width()));
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
