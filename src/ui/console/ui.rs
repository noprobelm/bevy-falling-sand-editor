use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    console_command::ConsoleCommandQueued,
    ui::{
        CommandHistory, ConsoleAction, ConsoleInformationAreaState, ConsolePromptState, ShowUi,
        UiSystems,
    },
};

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .in_set(UiSystems::Console),
        );
    }
}

fn show(
    mut contexts: EguiContexts,
    mut msgw_console_command_queued: MessageWriter<ConsoleCommandQueued>,
    mut information_area: ResMut<ConsoleInformationAreaState>,
    mut prompt: ResMut<ConsolePromptState>,
    mut command_history: ResMut<CommandHistory>,
    action_state: Single<&ActionState<ConsoleAction>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let toggle_info_area = action_state.just_pressed(&ConsoleAction::ToggleInformationArea);
    if toggle_info_area {
        if !information_area.is_open {
            prompt.request_focus = true;
        } else {
            prompt.surrender_focus = true;
        }
        information_area.is_open = !information_area.is_open;
    }

    egui::TopBottomPanel::top("information_area").show_animated(
        ctx,
        information_area.is_open,
        |ui| {
            information_area_ui(ui, &information_area);
        },
    );

    egui::TopBottomPanel::top("console").show(ctx, |ui| {
        prompt_ui(
            ui,
            &mut msgw_console_command_queued,
            &mut prompt,
            &mut command_history,
            toggle_info_area,
            &action_state,
        );
    });

    Ok(())
}

fn information_area_ui(ui: &mut egui::Ui, state: &ConsoleInformationAreaState) {
    let height = 400.0;
    egui::ScrollArea::vertical()
        .stick_to_bottom(true)
        .max_height(height)
        .min_scrolled_height(height)
        .auto_shrink(false)
        .show(ui, |ui| {
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                for msg in state.log_history.iter().rev() {
                    ui.label(msg);
                }
            });
        });
}

fn prompt_ui(
    ui: &mut egui::Ui,
    msgw: &mut MessageWriter<ConsoleCommandQueued>,
    prompt: &mut ConsolePromptState,
    history: &mut CommandHistory,
    toggled_info_area: bool,
    action_state: &ActionState<ConsoleAction>,
) {
    // Prevent hotkey character from being added to input
    let mut output = if toggled_info_area {
        egui::TextEdit::singleline(&mut prompt.input_text.clone())
            .desired_width(ui.available_width())
            .code_editor()
            .show(ui)
    } else {
        egui::TextEdit::singleline(&mut prompt.input_text)
            .desired_width(ui.available_width())
            .code_editor()
            .show(ui)
    };
    let response_id = output.response.id;

    if prompt.request_focus {
        output.response.request_focus();
        prompt.request_focus = false;
    }
    if prompt.surrender_focus {
        output.response.surrender_focus();
        prompt.surrender_focus = false;
    }

    if output.response.has_focus() {
        let up = ui.input(|i| i.key_pressed(egui::Key::ArrowUp));
        let down = ui.input(|i| i.key_pressed(egui::Key::ArrowDown));

        let mut moved = false;
        if up {
            if let Some(entry) = history.navigate_up(&prompt.input_text) {
                prompt.input_text = entry.to_string();
                moved = true;
            }
        } else if down {
            prompt.input_text = history.navigate_down().to_string();
            moved = true;
        }

        if moved {
            let ccursor = egui::text::CCursor::new(prompt.input_text.chars().count());
            output
                .state
                .cursor
                .set_char_range(Some(egui::text::CCursorRange::one(ccursor)));
            output.state.store(ui.ctx(), response_id);
        }
    }

    if action_state.just_pressed(&ConsoleAction::SubmitInputText) && !prompt.input_text.is_empty() {
        history.push(prompt.input_text.clone());
        msgw.write(ConsoleCommandQueued {
            input: prompt.input_text.clone(),
        });
        prompt.input_text.clear();
        output.response.request_focus();
    }
}
