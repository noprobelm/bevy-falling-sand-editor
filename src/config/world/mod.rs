mod persistence;
mod save;

use bevy::prelude::*;

pub use persistence::*;
pub use save::PrepareSaveWorldConfigEvent;

pub(super) struct WorldPersistencePlugin;

impl Plugin for WorldPersistencePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(save::SavePlugin);
    }
}
