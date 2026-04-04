use bevy::prelude::*;

use crate::canvas::brush::{BrushModeSpawnState, BrushSize, BrushTypeState};

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_set_brush_type)
            .add_observer(on_set_brush_size)
            .add_observer(on_set_brush_mode);
    }
}

#[derive(Event, Message, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BrushSetSizeSignal(pub usize);

#[derive(Event, Message, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BrushSetTypeSignal(pub BrushTypeState);

#[derive(Event, Message, Clone, Eq, PartialEq, Hash, Debug)]
pub struct BrushSetModeSignal(pub BrushModeSpawnState);

fn on_set_brush_size(
    trigger: On<BrushSetSizeSignal>,
    mut brush_size_query: Query<&mut BrushSize>,
) -> Result {
    let mut brush_size = brush_size_query.single_mut()?;
    let size = trigger.event().0;
    brush_size.0 = size;

    Ok(())
}

fn on_set_brush_type(
    trigger: On<BrushSetTypeSignal>,
    mut next_brush_type_state: ResMut<NextState<BrushTypeState>>,
) {
    next_brush_type_state.set(trigger.event().0);
}

fn on_set_brush_mode(
    trigger: On<BrushSetModeSignal>,
    mut next_spawn_state: ResMut<NextState<BrushModeSpawnState>>,
) {
    next_spawn_state.set(trigger.event().0);
}
