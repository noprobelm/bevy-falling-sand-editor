use bevy::prelude::*;
use bevy_falling_sand::prelude::ParticleType;
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
    Copy,
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
pub struct SelectedParticle(pub ParticleType);

/// Tracks which [`ParticleType`] entity the brush's [`SelectedParticle`] corresponds to.
#[derive(Component, Copy, Clone, PartialEq, Debug, Reflect)]
pub struct SelectedParticleType(pub Entity);
