mod action_panel;
mod console;
mod particle_editor;
mod quick_actions;
mod setup;
mod states;

use bevy::prelude::*;

pub use action_panel::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
pub use console::*;
use particle_editor::*;
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
            ActionPanelPlugin,
            ParticleEditorPlugin,
            ConsolePlugin,
            UiStatePlugin,
        ))
        .configure_sets(
            EguiPrimaryContextPass,
            (UiSystems::ActionPanel, UiSystems::Console).chain(),
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
}
