use bevy::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BrushTypeState>();
    }
}

#[derive(Default, Copy, Clone, Hash, Eq, PartialEq, Ord, PartialOrd, Debug, States)]
pub enum BrushTypeState {
    Line,
    #[default]
    Circle,
    Cursor,
}
