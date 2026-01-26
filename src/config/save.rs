use bevy::prelude::*;

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_prepare_world_save_complete);
    }
}

/// Trigger this event to begin the world save process.
#[derive(Event, Default, Debug)]
pub struct PrepareWorldSaveEvent;

/// Triggered automatically after PrepareWorldSaveEvent handlers complete.
#[derive(Event, Default, Debug)]
pub struct WorldSaveEvent;

/// Event to trigger saving settings data
#[derive(Event, Default, Debug)]
pub struct SaveSettingsEvent;

fn on_prepare_world_save_complete(_trigger: On<PrepareWorldSaveEvent>, mut commands: Commands) {
    commands.trigger(WorldSaveEvent);
}
