use bevy::prelude::*;

mod manifest;

pub(super) struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, manifest::write_particle_manifest);
    }
}
