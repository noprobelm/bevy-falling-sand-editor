use bevy::prelude::*;

use crate::ui::ConsoleSetupPlugin;

pub struct UiSetupPlugin;

impl Plugin for UiSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsoleSetupPlugin);
    }
}
