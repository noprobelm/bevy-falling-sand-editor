#[cfg(feature = "dev")]
use bevy::prelude::*;

#[cfg(feature = "dev")]
pub(super) struct RecordPlugin;

#[cfg(feature = "dev")]
impl Plugin for RecordPlugin {
    fn build(&self, _app: &mut App) {}
}
