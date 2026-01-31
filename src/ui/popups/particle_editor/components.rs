use bevy::prelude::*;
use bevy_falling_sand::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Resource, Clone, Default, PartialEq, Debug, Reflect, Serialize, Deserialize)]
pub struct SelectedEditorParticle(pub Particle);
