use bevy::prelude::*;
use leafwing_input_manager::prelude::ActionState;
use serde::{Deserialize, Serialize};

use crate::tools::{SelectedTool, ToolStateActions};

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EarthquakeShape>()
            .init_state::<EarthquakeFractureShape>()
            .add_sub_state::<EarthquakeBrushState>()
            .add_systems(
                Update,
                handle_brush_state.run_if(in_state(SelectedTool::Earthquake)),
            )
            .add_observer(on_set_earthquake_shape)
            .add_observer(on_set_earthquake_fracture_shape);
    }
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
#[source(SelectedTool = SelectedTool::Earthquake)]
pub enum EarthquakeBrushState {
    #[default]
    Draw,
    Resize,
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
pub enum EarthquakeShape {
    #[default]
    Circle,
    Rect,
    Polygon,
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
pub enum EarthquakeFractureShape {
    Convex,
    #[default]
    Concave,
}

#[derive(Event, Message, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct SetEarthquakeShape(pub EarthquakeShape);

#[derive(Event, Message, Clone, Copy, Eq, PartialEq, Hash, Debug)]
pub struct SetEarthquakeFractureShape(pub EarthquakeFractureShape);

fn handle_brush_state(
    actions: Single<&ActionState<ToolStateActions>>,
    mut state: ResMut<NextState<EarthquakeBrushState>>,
) -> Result {
    if actions.just_pressed(&ToolStateActions::Resize) {
        state.set(EarthquakeBrushState::Resize);
    }
    if actions.just_released(&ToolStateActions::Resize) {
        state.set(EarthquakeBrushState::Draw);
    }

    Ok(())
}

fn on_set_earthquake_shape(
    trigger: On<SetEarthquakeShape>,
    mut next_shape: ResMut<NextState<EarthquakeShape>>,
) {
    next_shape.set(trigger.event().0);
}

fn on_set_earthquake_fracture_shape(
    trigger: On<SetEarthquakeFractureShape>,
    mut next_shape: ResMut<NextState<EarthquakeFractureShape>>,
) {
    next_shape.set(trigger.event().0);
}
