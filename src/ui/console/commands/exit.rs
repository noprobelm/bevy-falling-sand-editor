use bevy::prelude::*;
use bevy_falling_sand::prelude::SaveAllChunks;

use crate::{
    config::{PrepareWorldSaveEvent, SaveSettingsEvent},
    directive::Directive,
};

pub struct ExitDirectivePlugin;

impl Plugin for ExitDirectivePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_exit_application).add_systems(
            PostUpdate,
            wait_for_saves_then_exit.run_if(resource_exists::<PendingExit>),
        );
    }
}

/// Resource that tracks whether the application is waiting to exit after saves complete.
#[derive(Resource)]
struct PendingExit;

#[derive(Event)]
pub struct ExitApplicationEvent;

#[derive(Default)]
pub struct ExitDirective;

impl Directive for ExitDirective {
    fn name(&self) -> &'static str {
        "exit"
    }

    fn description(&self) -> &'static str {
        "Exit the application"
    }

    fn run(&self, _args: &[String], commands: &mut Commands) {
        commands.trigger(ExitApplicationEvent);
    }
}

fn on_exit_application(_trigger: On<ExitApplicationEvent>, mut commands: Commands) {
    // Save world and settings via observers
    commands.trigger(PrepareWorldSaveEvent);
    commands.trigger(SaveSettingsEvent);

    // Save chunks (may be async)
    commands.write_message(SaveAllChunks);

    // Mark that we're waiting to exit
    commands.insert_resource(PendingExit);
}

/// System that exits the application once all save tasks have completed.
fn wait_for_saves_then_exit(
    pending_bfs_saves: Res<bevy_falling_sand::persistence::PendingSaveTasks>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if pending_bfs_saves.is_empty() {
        app_exit.write(AppExit::Success);
    }
}
