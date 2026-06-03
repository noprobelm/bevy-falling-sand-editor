mod components;
pub mod debug;
mod fracture;
mod gizmos;
mod region;
mod resources;
mod setup;
mod signals;
mod states;
mod systems;
mod voronoi;

use bevy::ecs::system::SystemParam;
use bevy::prelude::*;
pub use signals::*;

pub use components::*;
pub use debug::DebugEarthquake;
pub use region::EarthquakeRegion;
pub use resources::EarthquakeConfiguration;
pub use states::*;

pub(super) struct EarthquakePlugin;

impl Plugin for EarthquakePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            debug::DebugPlugin,
            gizmos::GizmosPlugin,
            setup::SetupPlugin,
            signals::SignalsPlugin,
            states::StatesPlugin,
            systems::SystemsPlugin,
        ));
    }
}

#[derive(SystemParam)]
pub struct EarthquakeOptions<'w, 's> {
    pub size: Single<
        'w,
        's,
        &'static mut crate::tools::brush::ToolBrushSize,
        (
            With<crate::tools::earthquake::EarthquakeBrush>,
            Without<crate::tools::painter::PainterBrush>,
        ),
    >,
    pub color: Single<
        'w,
        's,
        &'static mut crate::tools::brush::ToolBrushColor,
        (
            With<crate::tools::earthquake::EarthquakeBrush>,
            Without<crate::tools::painter::PainterBrush>,
        ),
    >,
    pub configuration: ResMut<'w, EarthquakeConfiguration>,
    pub current_region_state: Res<'w, State<EarthquakeShape>>,
    pub next_region_state: ResMut<'w, NextState<EarthquakeShape>>,
    pub current_fracture_shape_state: Res<'w, State<EarthquakeFractureShape>>,
    pub next_fracture_shape_state: ResMut<'w, NextState<EarthquakeFractureShape>>,
    pub debug: Option<Res<'w, DebugEarthquake>>,
}
