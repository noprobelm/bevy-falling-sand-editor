use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    directive::DirectiveQueued,
    ui::{ConsoleAction, ConsoleState, InformationAreaState, PromptState},
};

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(EguiPrimaryContextPass, show_console);
    }
}

fn show_console(
    mut contexts: EguiContexts,
    msgw_directive_queued: MessageWriter<DirectiveQueued>,
    mut console_state: ResMut<ConsoleState>,
    action_state: Single<&ActionState<ConsoleAction>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    if action_state.just_pressed(&ConsoleAction::ToggleInformationArea) {
        console_state.information_area.is_open = !console_state.information_area.is_open;
    }
    egui::TopBottomPanel::top("information_area").show_animated(
        ctx,
        console_state.information_area.is_open,
        |ui| {
            information_area_ui(ui, &console_state.information_area);
        },
    );
    egui::TopBottomPanel::top("console").show(ctx, |ui| {
        prompt_ui(ui, &mut console_state.prompt);
    });
    Ok(())
}

fn information_area_ui(ui: &mut egui::Ui, information_area_state: &InformationAreaState) {
    let height = 400.0;
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .max_height(height)
        .min_scrolled_height(height)
        .auto_shrink(false)
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                for msg in information_area_state.history.iter().rev() {
                    ui.label(msg);
                }
            });
        });
}

fn prompt_ui(ui: &mut egui::Ui, prompt_state: &mut PromptState) {
    ui.add(
        egui::TextEdit::singleline(&mut prompt_state.input_text)
            .desired_width(ui.available_width()),
    );
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
