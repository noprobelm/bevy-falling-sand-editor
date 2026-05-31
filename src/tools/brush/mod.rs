mod components;
mod gizmos;
mod setup;
mod signals;
mod states;
pub mod systems;

use bevy::{ecs::system::SystemParam, prelude::*};
pub use signals::*;

use crate::tools::brush::{gizmos::GizmosPlugin, setup::SetupPlugin, systems::SystemsPlugin};
pub use components::*;
pub use setup::*;
pub use states::*;

pub struct BrushToolPlugin;

impl Plugin for BrushToolPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            SetupPlugin,
            SignalsPlugin,
            StatesPlugin,
            SystemsPlugin,
            GizmosPlugin,
        ));
    }
}

#[derive(SystemParam)]
pub struct BrushOptions<'w, 's> {
    pub size: Single<'w, 's, &'static mut crate::brush::BrushSize>,
    pub current_type_state: Res<'w, State<BrushTypeState>>,
    pub next_type_state: ResMut<'w, NextState<BrushTypeState>>,
    pub current_mode_state: Res<'w, State<BrushSpawnState>>,
    pub next_mode_state: ResMut<'w, NextState<BrushSpawnState>>,
}
