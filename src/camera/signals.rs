use bevy::prelude::*;

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SaveCameraSignal>();
    }
}

#[derive(Event, Message, Default, Eq, PartialEq, Hash, Debug, Reflect)]
pub struct SaveCameraSignal;
