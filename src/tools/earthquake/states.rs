use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<EarthquakeRegionState>();
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
