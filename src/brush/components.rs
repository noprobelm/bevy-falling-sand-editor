use bevy::prelude::*;
use bevy_falling_sand::prelude::Particle;
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
pub struct Brush;

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
pub struct BrushSize(pub usize);

#[derive(Component, Clone, Default, PartialEq, Debug, Reflect, Serialize, Deserialize)]
pub struct BrushColor(pub Color);

#[derive(Component, Clone, Default, PartialEq, Debug, Reflect, Serialize, Deserialize)]
pub struct BrushParticle(pub Particle);
