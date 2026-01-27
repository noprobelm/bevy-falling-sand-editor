use bevy::prelude::*;

pub struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<BrushTypeState>()
            .init_state::<BrushModeState>()
            .add_sub_state::<BrushModeSpawnState>();
    }
}

#[derive(States, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BrushTypeState {
    Line,
    #[default]
    Circle,
    Cursor,
}

#[derive(States, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum BrushModeState {
    #[default]
    Spawn,
    Despawn,
}

#[derive(SubStates, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[source(BrushModeState = BrushModeState::Spawn)]
pub enum BrushModeSpawnState {
    #[default]
    Particles,
    DynamicRigidBodies,
}
