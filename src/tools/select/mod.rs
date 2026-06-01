mod gizmos;
mod resources;
mod setup;
pub mod states;
mod systems;

use bevy::{ecs::system::SystemParam, prelude::*};
use gizmos::*;
use resources::*;
use setup::*;
use states::*;
use systems::*;

pub struct SelectToolPlugin;

impl Plugin for SelectToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ResourcesPlugin,
            SetupPlugin,
            StatesPlugin,
            SystemsPlugin,
            GizmosPlugin,
        ));
    }
}

#[derive(SystemParam)]
pub struct SelectOptions<'w> {
    pub current_mode_state: Res<'w, State<SelectModeState>>,
    pub next_mode_state: ResMut<'w, NextState<SelectModeState>>,
}
