mod help;

use bevy::prelude::*;

pub use help::HelpDirective;

use crate::ui::directives::help::HelpDirectivePlugin;

pub struct DirectivesPlugin;

impl Plugin for DirectivesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(HelpDirectivePlugin);
    }
}
