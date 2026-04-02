mod setup;

use avian2d::prelude::PhysicsDebugPlugin;
use bevy::prelude::*;
use bevy_falling_sand::debug::FallingSandDebugPlugin;

use setup::*;

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FallingSandDebugPlugin, PhysicsDebugPlugin, SetupPlugin));
    }
}
