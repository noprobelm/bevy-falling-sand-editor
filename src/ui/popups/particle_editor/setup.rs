use bevy::prelude::*;

use crate::ui::ParticleMaterialLabels;

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ParticleMaterialLabels>();
    }
}
