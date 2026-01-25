use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::{ConsoleKeyBindings, ConsoleSetupPlugin};

pub struct UiSetupPlugin;

impl Plugin for UiSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ConsoleSetupPlugin);
    }
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct UiKeyBindings {
    pub console: ConsoleKeyBindings,
}
