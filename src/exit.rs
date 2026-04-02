use bevy::prelude::*;
use bevy_falling_sand::prelude::PersistChunksSignal;

use crate::config::{PrepareSaveSettingsEvent, PrepareSaveWorldConfigEvent};

pub struct ExitPlugin;

impl Plugin for ExitPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_exit_application).add_systems(
            PostUpdate,
            wait_for_saves_then_exit.run_if(resource_exists::<PendingExit>),
        );
    }
}

#[derive(Resource)]
struct PendingExit;

#[derive(Event)]
pub struct ExitApplicationEvent;

fn on_exit_application(_trigger: On<ExitApplicationEvent>, mut commands: Commands) {
    commands.trigger(PrepareSaveWorldConfigEvent);
    commands.trigger(PrepareSaveSettingsEvent);

    commands.write_message(PersistChunksSignal);

    commands.insert_resource(PendingExit);
}

fn wait_for_saves_then_exit(
    pending_bfs_saves: Res<bevy_falling_sand::persistence::PendingSaveTasks>,
    mut app_exit: MessageWriter<AppExit>,
) {
    if pending_bfs_saves.is_empty() {
        app_exit.write(AppExit::Success);
    }
}
