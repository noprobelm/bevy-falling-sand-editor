use bevy::prelude::*;

use crate::tools::earthquake::{EarthquakeConfiguration, EarthquakeRegion};

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<EarthquakeGizmos>()
            .add_systems(Update, debug_earthquake);
    }
}

#[derive(Resource, Default)]
pub struct DebugEarthquake;

#[derive(GizmoConfigGroup, Copy, Clone, Default, Debug, Reflect)]
pub(super) struct EarthquakeGizmos;

#[derive(Component)]
pub(super) struct DebugEarthquakeInfo {
    pub(super) region: EarthquakeRegion,
    pub(super) fracture_edges: Vec<(Vec2, Vec2)>,
    pub(super) timer: Timer,
}

fn debug_earthquake(
    mut commands: Commands,
    mut debug_earthquake: Query<(Entity, &mut DebugEarthquakeInfo)>,
    config: Res<EarthquakeConfiguration>,
    time: Res<Time>,
    mut earthquake_gizmos: Gizmos<EarthquakeGizmos>,
) {
    debug_earthquake
        .iter_mut()
        .for_each(|(entity, mut debug_earthquake)| {
            debug_earthquake.timer.tick(time.delta());
            let alpha = 1. - debug_earthquake.timer.fraction();
            debug_earthquake.region.draw_gizmo(
                &mut earthquake_gizmos,
                config.debug_region_color_with_alpha(alpha),
            );
            let fracture_color = config.debug_fracture_color_with_alpha(alpha);
            for &(start, end) in &debug_earthquake.fracture_edges {
                earthquake_gizmos.line_2d(start, end, fracture_color);
            }
            if debug_earthquake.timer.is_finished() {
                commands.entity(entity).despawn();
            }
        });
}
