use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::{
    ConsoleKeyBindings, ConsoleSetupPlugin, QuickActionsKeyBindings, QuickActionsSetupPlugin,
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ConsoleSetupPlugin, QuickActionsSetupPlugin));
    }
}

#[derive(Resource, Default, Clone, Debug, Serialize, Deserialize)]
pub struct UiKeyBindings {
    pub console: ConsoleKeyBindings,
    pub quick_actions: QuickActionsKeyBindings,
}
