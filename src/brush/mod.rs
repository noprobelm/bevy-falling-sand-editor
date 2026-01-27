mod components;
mod gizmos;
mod setup;
mod states;

use states::*;

use bevy::prelude::*;

use crate::brush::{gizmos::GizmosPlugin, setup::SetupPlugin};

pub struct BrushPlugin;

impl Plugin for BrushPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SetupPlugin, StatesPlugin, GizmosPlugin));
    }
}
