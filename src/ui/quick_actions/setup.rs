use bevy::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{Actionlike, plugin::InputManagerPlugin, prelude::InputMap};
use serde::{Deserialize, Serialize};

use crate::{
    config::{InputButton, SettingsConfig},
    setup::SetupSystems,
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<QuickAction>::default())
            .add_systems(Startup, load_settings.in_set(SetupSystems::Ui));
    }
}

#[derive(Actionlike, Resource, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
#[allow(clippy::enum_variant_names)]
pub enum QuickAction {
    ToggleUi,
    ToggleMapOverlay,
    ToggleDirtyChunksOverlay,
    ToggleSimulationRun,
    ToggleSimulationStep,
    SampleHoveredParticle,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct QuickActionsKeyBindings {
    pub toggle_ui: InputButton,
    pub toggle_map_overlay: InputButton,
    pub toggle_dirty_chunks_overlay: InputButton,
    pub toggle_simulation_run: InputButton,
    pub toggle_simulation_step: InputButton,
    pub sample_hovered_particle: InputButton,
}

impl Default for QuickActionsKeyBindings {
    fn default() -> Self {
        Self {
            toggle_ui: KeyCode::KeyH.into(),
            toggle_map_overlay: KeyCode::F1.into(),
            toggle_dirty_chunks_overlay: KeyCode::F2.into(),
            toggle_simulation_run: KeyCode::Space.into(),
            toggle_simulation_step: KeyCode::Enter.into(),
            sample_hovered_particle: MouseButton::Middle.into(),
        }
    }
}

fn load_settings(mut commands: Commands, settings_config: Res<Persistent<SettingsConfig>>) {
    let keys = &settings_config.get().keys.ui.quick_actions;
    let mut input_map = InputMap::default();
    keys.toggle_ui
        .insert_into_input_map(&mut input_map, QuickAction::ToggleUi);
    keys.toggle_map_overlay
        .insert_into_input_map(&mut input_map, QuickAction::ToggleMapOverlay);
    keys.toggle_dirty_chunks_overlay
        .insert_into_input_map(&mut input_map, QuickAction::ToggleDirtyChunksOverlay);
    keys.toggle_simulation_run
        .insert_into_input_map(&mut input_map, QuickAction::ToggleSimulationRun);
    keys.toggle_simulation_step
        .insert_into_input_map(&mut input_map, QuickAction::ToggleSimulationStep);
    keys.sample_hovered_particle
        .insert_into_input_map(&mut input_map, QuickAction::SampleHoveredParticle);
    commands.spawn(input_map);
}
