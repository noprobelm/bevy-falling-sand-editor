mod config;
mod ui;

use bevy::prelude::*;

pub use config::*;
pub use ui::*;

pub struct AppStatePlugin;

impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((ConfigStatePlugin, UiStatePlugin));
    }
}
