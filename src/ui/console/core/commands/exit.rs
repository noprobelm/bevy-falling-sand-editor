use bevy::prelude::*;
use bevy_falling_sand::prelude::{PendingSaveTasks, SaveAllChunks};

use super::super::core::{ConsoleCommand, PrintConsoleLine};

pub struct ExitCommandPlugin;

impl Plugin for ExitCommandPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingExit>()
            .add_observer(on_exit_application)
            .add_systems(PostUpdate, wait_for_saves_then_exit);
    }
}

/// Resource that tracks whether the application is waiting to exit after saves complete.
#[derive(Resource, Default)]
struct PendingExit(bool);

#[derive(Event)]
pub struct ExitApplicationEvent;

#[derive(Default)]
pub struct ExitCommand;

impl ConsoleCommand for ExitCommand {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "Exit the application"
    }

    fn execute_action(
        &self,
        _args: &[String],
        _console_writer: &mut MessageWriter<PrintConsoleLine>,
        commands: &mut Commands,
    ) {
        commands.trigger(ExitApplicationEvent);
    }
}

fn on_exit_application(
    _trigger: On<ExitApplicationEvent>,
    mut commands: Commands,
    mut pending_exit: ResMut<PendingExit>,
) {
    // Save all chunks and mark that we're waiting to exit
    commands.trigger(SaveAllChunks);
    pending_exit.0 = true;
}

/// System that exits the application once all save tasks have completed.
fn wait_for_saves_then_exit(
    pending_exit: Res<PendingExit>,
    pending_saves: Res<PendingSaveTasks>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if pending_exit.0 && pending_saves.is_empty() {
        app_exit.write(AppExit::Success);
    }
}

