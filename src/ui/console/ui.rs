use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
use leafwing_input_manager::prelude::ActionState;
use shlex::Shlex;

use crate::{
    directive::{DirectiveQueued, DirectiveRegistry},
    ui::{ConsoleAction, ConsoleConfiguration, ConsoleState, InformationAreaState, PromptState},
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
    registry: Res<DirectiveRegistry>,
) -> Result {
    let ctx = contexts.ctx_mut()?;
    let information_area_toggled = action_state.just_pressed(&ConsoleAction::ToggleInformationArea);
    if information_area_toggled {
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
            msgw_directive_queued,
            &mut console_state.prompt,
            information_area_toggled,
            &action_state,
            &registry,
        );
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
                for msg in information_area_state.log_history.iter().rev() {
                    ui.label(msg);
                }
            });
        });
}

fn prompt_ui(
    ui: &mut egui::Ui,
    msgw_directive_queued: MessageWriter<DirectiveQueued>,
    prompt_state: &mut PromptState,
    information_area_toggled: bool,
    action_state: &ActionState<ConsoleAction>,
    registry: &DirectiveRegistry,
) {
    // We don't want the hotkey for toggling the information area to add actual text to the input.
    let response = if information_area_toggled {
        ui.add(
            egui::TextEdit::singleline(&mut prompt_state.input_text.clone())
                .desired_width(ui.available_width()),
        )
    } else {
        ui.add(
            egui::TextEdit::singleline(&mut prompt_state.input_text)
                .desired_width(ui.available_width()),
        )
    };

    if prompt_state.request_focus {
        response.request_focus();
        prompt_state.request_focus = false;
    }

    if action_state.just_pressed(&ConsoleAction::SubmitInputText) {
        let command = prompt_state.input_text.clone();
        execute_command(command, registry);
        prompt_state.input_text.clear();
    }
}

fn execute_command(command: String, registry: &DirectiveRegistry) {
    if command.is_empty() {
        return;
    }
    let args = Shlex::new(&command).collect::<Vec<_>>();
    if let Some(directive) = registry.find_command(&args[0]) {
        let node = directive.build_directive_node();
        let args = node.get_args(&args);
        println!("{:?}", directive.name());
    }
}
