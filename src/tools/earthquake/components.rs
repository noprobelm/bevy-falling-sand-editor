use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(
    Component,
    Clone,
    Default,
    Eq,
    PartialEq,
    Ord,
    PartialOrd,
    Hash,
    Debug,
    Reflect,
    Serialize,
    Deserialize,
)]
pub struct EarthquakeBrush;

#[derive(
    Component, Copy, Clone, Default, PartialEq, PartialOrd, Debug, Reflect, Serialize, Deserialize,
)]
pub struct EarthquakeBrushSize(pub f32);

#[derive(Component, Clone, Default, PartialEq, Debug, Reflect, Serialize, Deserialize)]
pub struct EarthquakeBrushColor(pub Color);
