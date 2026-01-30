mod particle_editor;
mod settings;

use bevy::prelude::*;
use std::{fmt::Debug, hash::Hash, marker::PhantomData};

pub use particle_editor::*;
pub use settings::*;

pub struct PopupsPlugin;

impl Plugin for PopupsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ParticleEditorPlugin, SettingsPlugin));
    }
}

#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum ActionPanelApplicationState<T: Send + Sync + Default + Debug + Clone + Eq + Hash + 'static>
{
    #[default]
    Closed,
    Open,
    #[doc(hidden)]
    _Marker(PhantomData<T>),
}

impl<T: Send + Sync + Default + Debug + Clone + Eq + Hash + 'static>
    ActionPanelApplicationState<T>
{
    pub fn is_open(&self) -> bool {
        self == &Self::Open
    }

    pub fn get_next(&self) -> Self {
        match self {
            Self::Open => Self::Closed,
            Self::Closed => Self::Open,
            _ => unreachable!(),
        }
    }
}
