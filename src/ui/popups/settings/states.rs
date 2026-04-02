use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<SettingsApplicationState>()
            .add_sub_state::<SettingsCategory>()
            .add_sub_state::<KeybindsListeningState>();
    }
}

#[derive(
    States,
    Reflect,
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
pub enum SettingsApplicationState {
    #[default]
    Closed,
    Open,
}

#[derive(
    SubStates,
    Reflect,
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[source(SettingsApplicationState = SettingsApplicationState::Open)]
pub enum SettingsCategory {
    #[default]
    Brush,
    Debug,
    Keybinds,
}

#[derive(
    SubStates,
    Reflect,
    Default,
    Debug,
    Copy,
    Clone,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Serialize,
    Deserialize,
)]
#[source(SettingsCategory = SettingsCategory::Keybinds)]
pub enum KeybindsListeningState {
    #[default]
    Deafened,
    Listening,
}

#[derive(Resource)]
pub struct ListeningForKeybind {
    pub binding_id: &'static str,
}
