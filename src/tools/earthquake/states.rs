use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EarthquakeRegionState>()
            .init_state::<EarthquakeFractureShapeState>();
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
pub enum EarthquakeRegionState {
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
pub enum EarthquakeFractureShapeState {
    SimplifiedConvexHulls,
    #[default]
    ExactVoronoiCells,
}
