mod barnsley;
mod sierpinski;
mod text;

use bevy::prelude::*;

pub use barnsley::*;
pub use sierpinski::*;
pub use text::*;

use crate::particles::{carpet::spawn_sierpinski_carpet, triangle::spawn_sierpinski_triangle};

pub(super) struct PatternsPlugin;

impl Plugin for PatternsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_sierpinski_carpet)
            .add_observer(spawn_sierpinski_triangle)
            .add_observer(spawn_barnsley)
            .add_observer(spawn_text);
    }
}
