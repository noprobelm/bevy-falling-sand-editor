use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::CanvasState;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SelectState>();
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
#[source(CanvasState = CanvasState::Select)]
pub enum SelectState {
    #[default]
    Idle,
    ExpandSelection,
    DragParticles,
}
