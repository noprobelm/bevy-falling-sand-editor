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
    mut msgw_directive_queued: MessageWriter<DirectiveQueued>,
    mut console_state: ResMut<ConsoleState>,
    action_state: Single<&ActionState<ConsoleAction>>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    let toggled_info_area = action_state.just_pressed(&ConsoleAction::ToggleInformationArea);
    if toggled_info_area {
        console_state.information_area.is_open = !console_state.information_area.is_open;
        if console_state.information_area.is_open {
            console_state.prompt.request_focus = true;
        }
    }

    egui::TopBottomPanel::top("information_area").show_animated(
        ctx,
        console_state.information_area.is_open,
        |ui| {
            information_area_ui(ui, &console_state.information_area);
        },
    );

    egui::TopBottomPanel::top("console").show(ctx, |ui| {
        prompt_ui(
            ui,
            &mut msgw_directive_queued,
            &mut console_state.prompt,
            toggled_info_area,
            &action_state,
        );
    });

    Ok(())
}

fn information_area_ui(ui: &mut egui::Ui, state: &InformationAreaState) {
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
    msgw: &mut MessageWriter<DirectiveQueued>,
    prompt: &mut PromptState,
    toggled_info_area: bool,
    action_state: &ActionState<ConsoleAction>,
) {
    // Prevent hotkey character from being typed into input
    let response = if toggled_info_area {
        ui.add(
            egui::TextEdit::singleline(&mut prompt.input_text.clone())
                .desired_width(ui.available_width())
                .code_editor(),
        )
    } else {
        ui.add(
            egui::TextEdit::singleline(&mut prompt.input_text)
                .desired_width(ui.available_width())
                .code_editor(),
        )
    };

    if prompt.request_focus {
        response.request_focus();
        prompt.request_focus = false;
    }

    if action_state.just_pressed(&ConsoleAction::SubmitInputText) && !prompt.input_text.is_empty() {
        msgw.write(DirectiveQueued {
            input: prompt.input_text.clone(),
        });
        prompt.input_text.clear();
        response.request_focus();
    }
}
