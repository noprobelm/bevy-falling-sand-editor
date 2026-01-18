mod states;

use bevy::prelude::*;

use states::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(StatesPlugin);
    }
}
