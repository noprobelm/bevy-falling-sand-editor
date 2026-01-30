use std::{fmt::Debug, hash::Hash, marker::PhantomData};

use bevy::prelude::*;

#[derive(States, Reflect, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum PopupState<T: Send + Sync + Default + Debug + Clone + Eq + Hash + 'static> {
    #[default]
    Closed,
    Open,
    #[doc(hidden)]
    _Marker(PhantomData<T>),
}

impl<T: Send + Sync + Default + Debug + Clone + Eq + Hash + 'static> PopupState<T> {
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
