mod console;
mod quick_actions;
mod setup;
mod states;

use bevy::prelude::*;

use bevy_egui::EguiPlugin;
pub use console::*;
pub use quick_actions::*;
pub use setup::*;
pub use states::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            SetupPlugin,
            QuickActionsPlugin,
            ConsolePlugin,
            UiStatePlugin,
        ))
        .init_resource::<ShowUi>();
    }
}

#[derive(Resource, Default)]
pub struct ShowUi;
