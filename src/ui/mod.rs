mod action_panel;
mod console;
mod helpers;
mod popups;
mod quick_actions;
mod setup;
mod states;
pub mod widgets;

use bevy::prelude::*;

pub use action_panel::*;
use bevy_egui::{EguiPlugin, EguiPrimaryContextPass};
pub use console::*;
pub use helpers::*;
pub use popups::*;
pub use quick_actions::*;
pub use setup::*;
pub use states::*;

#[derive(Event)]
pub struct UiToggleSignal;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            EguiPlugin::default(),
            SetupPlugin,
            QuickActionsPlugin,
            ActionPanelPlugin,
            PopupsPlugin,
            ConsolePlugin,
            UiStatePlugin,
        ))
        .configure_sets(
            EguiPrimaryContextPass,
            (
                // The order in which these execute matter so that expanding the console can push
                // other windows out of the way.
                UiSystems::Console,
                UiSystems::ActionPanel,
                UiSystems::ParticleEditor,
                UiSystems::Settings,
                UiSystems::CursorInfoOverlay,
            )
                .chain(),
        )
        .init_resource::<ShowUi>()
        .add_observer(on_ui_toggle);
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
    CursorInfoOverlay,
}

fn on_ui_toggle(
    _trigger: On<UiToggleSignal>,
    mut commands: Commands,
    enabled: Option<Res<ShowUi>>,
) {
    if enabled.is_some() {
        commands.remove_resource::<ShowUi>();
    } else {
        commands.insert_resource(ShowUi);
    }
}
