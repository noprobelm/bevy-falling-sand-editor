mod components;
mod gizmos;
mod resources;
mod setup;
mod states;
pub mod systems;

use bevy::{ecs::system::SystemParam, prelude::*};

use crate::tools::painter::{gizmos::GizmosPlugin, setup::SetupPlugin, systems::SystemsPlugin};
pub use components::*;
pub use resources::*;
pub use setup::*;
pub use states::*;

pub struct PainterPlugin;

impl Plugin for PainterPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, StatesPlugin, SystemsPlugin, GizmosPlugin));
    }
}

#[derive(SystemParam)]
pub struct PainterOptions<'w, 's> {
    pub size: Single<
        'w,
        's,
        &'static mut crate::tools::brush::ToolBrushSize,
        (
            With<crate::tools::painter::PainterBrush>,
            Without<crate::tools::earthquake::EarthquakeBrush>,
        ),
    >,
    pub color: Single<
        'w,
        's,
        &'static mut crate::tools::brush::ToolBrushColor,
        (
            With<crate::tools::painter::PainterBrush>,
            Without<crate::tools::earthquake::EarthquakeBrush>,
        ),
    >,
    pub configuration: ResMut<'w, PainterConfiguration>,
    pub current_type_state: Res<'w, State<PainterShape>>,
    pub next_type_state: ResMut<'w, NextState<PainterShape>>,
    pub current_mode_state: Res<'w, State<PainterSpawnState>>,
    pub next_mode_state: ResMut<'w, NextState<PainterSpawnState>>,
}
