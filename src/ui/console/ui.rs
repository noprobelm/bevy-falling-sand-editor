use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use leafwing_input_manager::prelude::ActionState;

use crate::{
    directive::DirectiveQueued,
    ui::{ConsoleAction, ConsoleInformationAreaState, ConsolePromptState, ShowUi, UiSystems},
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
    mut msgw_directive_queued: MessageWriter<DirectiveQueued>,
    mut information_area: ResMut<ConsoleInformationAreaState>,
    mut prompt: ResMut<ConsolePromptState>,
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
            &mut msgw_directive_queued,
            &mut prompt,
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
    msgw: &mut MessageWriter<DirectiveQueued>,
    prompt: &mut ConsolePromptState,
    toggled_info_area: bool,
    action_state: &ActionState<ConsoleAction>,
) {
    // Prevent hotkey character from being added to input
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
    if prompt.surrender_focus {
        response.surrender_focus();
        prompt.surrender_focus = false;
    }

    if action_state.just_pressed(&ConsoleAction::SubmitInputText) && !prompt.input_text.is_empty() {
        msgw.write(DirectiveQueued {
            input: prompt.input_text.clone(),
        });
        prompt.input_text.clear();
        response.request_focus();
    }
}
