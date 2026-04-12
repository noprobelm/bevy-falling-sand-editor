use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::ui::CanvasState;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<SelectState>()
            .init_state::<SelectModeState>()
            .add_observer(on_set_select_mode);
    }
}

#[derive(Event)]
pub struct SetSelectModeEvent(pub SelectModeState);

fn on_set_select_mode(
    trigger: On<SetSelectModeEvent>,
    mut state: ResMut<NextState<SelectModeState>>,
) {
    state.set(trigger.event().0);
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
pub enum SelectModeState {
    Drag,
    #[default]
    Throw,
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
