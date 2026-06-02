use bevy::prelude::*;

use crate::tools::earthquake::EarthquakeRegion;

pub(super) struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<EarthquakeGizmos>()
            .add_systems(Update, debug_earthquake);
    }
}

#[derive(GizmoConfigGroup, Copy, Clone, Default, Debug, Reflect)]
pub(super) struct EarthquakeGizmos;

#[derive(Component)]
pub(super) struct DebugEarthquake {
    pub(super) region: EarthquakeRegion,
    pub(super) fracture_edges: Vec<(Vec2, Vec2)>,
    pub(super) timer: Timer,
}

fn debug_earthquake(
    mut commands: Commands,
    mut debug_earthquake: Query<(Entity, &mut DebugEarthquake)>,
    time: Res<Time>,
    mut earthquake_gizmos: Gizmos<EarthquakeGizmos>,
) {
    debug_earthquake
        .iter_mut()
        .for_each(|(entity, mut debug_earthquake)| {
            debug_earthquake.timer.tick(time.delta());
            let alpha = 1. - debug_earthquake.timer.fraction();
            draw_earthquake_region_gizmo(
                &mut earthquake_gizmos,
                &debug_earthquake.region,
                Color::srgba(1., 1., 1., alpha),
            );
            let fracture_color = Color::srgba(1., 0.4, 0.2, alpha);
            for &(start, end) in &debug_earthquake.fracture_edges {
                earthquake_gizmos.line_2d(start, end, fracture_color);
            }
            if debug_earthquake.timer.is_finished() {
                commands.entity(entity).despawn();
            }
        });
}

fn draw_earthquake_region_gizmo(
    gizmos: &mut Gizmos<EarthquakeGizmos>,
    region: &EarthquakeRegion,
    color: Color,
) {
    match region {
        EarthquakeRegion::Circle { center, radius } => {
            gizmos.circle_2d(Isometry2d::from_translation(*center), *radius, color);
        }
        EarthquakeRegion::Rect {
            center,
            half_extents,
            rotation,
        } => {
            gizmos.rect_2d(
                Isometry2d::new(*center, Rot2::radians(*rotation)),
                *half_extents * 2.0,
                color,
            );
        }
        EarthquakeRegion::Polygon { vertices } => {
            if vertices.len() < 2 {
                return;
            }

            for (&start, &end) in vertices.iter().zip(vertices.iter().cycle().skip(1)) {
                gizmos.line_2d(start, end, color);
            }
        }
    }
}
