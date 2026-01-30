mod action_panel;
mod console;
mod quick_actions;
mod setup;
mod states;
mod widgets;

use bevy::prelude::*;

pub use action_panel::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
pub use console::*;
pub use quick_actions::*;
pub use setup::*;
pub use states::*;
pub use widgets::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            SetupPlugin,
            QuickActionsPlugin,
            ActionPanelPlugin,
            ConsolePlugin,
            UiStatePlugin,
        ))
        .configure_sets(
            EguiPrimaryContextPass,
            (
                UiSystems::ActionPanel,
                UiSystems::Console,
                UiSystems::ParticleEditor,
                UiSystems::Settings,
            )
                .chain(),
        )
        .init_resource::<ShowUi>();
    }
}

#[derive(Resource, Default)]
pub struct ShowUi;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum UiSystems {
    ActionPanel,
    Console,
    ParticleEditor,
    Settings,
}
