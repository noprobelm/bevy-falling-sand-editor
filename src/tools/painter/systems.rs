use bevy::prelude::*;
use bevy_falling_sand::{
    core::{DespawnParticleSignal, ParticleMap, ParticleTypeRegistry, SpawnParticleSignal},
    render::textures::WorldTextureOrigin,
};
use leafwing_input_manager::{common_conditions::action_pressed, prelude::ActionState};

use crate::{
    Cursor,
    game_of_life::{GolSpawnBuffer, GolTextures},
    tools::{
        ToolAction,
        brush::ToolBrushSize,
        earthquake::RemoveFractureBodyCellsAtWorldPositions,
        painter::{
            PainterAction, PainterBrushState, PainterConfiguration, PainterModeState, PainterShape,
            PainterSpawnState,
            components::{PainterBrush, SelectedParticle, SelectedParticleType},
            resources::PAINTER_BRUSH_MIN_SIZE,
        },
    },
};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sync_selected_particle_type)
            .add_systems(
                Update,
                resize_brush.run_if(in_state(PainterBrushState::Resize)),
            )
            .add_systems(
                Update,
                (
                    brush_action_spawn_particles
                        .run_if(action_pressed(ToolAction::Primary))
                        .run_if(in_state(PainterBrushState::Draw))
                        .run_if(in_state(PainterModeState::Particles)),
                    brush_action_despawn_particles
                        .run_if(action_pressed(ToolAction::Primary))
                        .run_if(in_state(PainterBrushState::Draw))
                        .run_if(in_state(PainterSpawnState::Despawn)),
                    brush_action_spawn_conway
                        .run_if(resource_exists::<GolTextures>)
                        .run_if(action_pressed(ToolAction::Primary))
                        .run_if(in_state(PainterBrushState::Draw))
                        .run_if(in_state(PainterModeState::Conway)),
                ),
            );
    }
}

/// Keeps [`SelectedParticleType`] in sync when [`SelectedParticle`] changes
/// (e.g. user picks a different particle in the editor or samples one from the canvas).
fn sync_selected_particle_type(
    mut brush_query: Query<
        (&SelectedParticle, &mut SelectedParticleType),
        Changed<SelectedParticle>,
    >,
    registry: Res<ParticleTypeRegistry>,
) {
    for (selected, mut tracked) in &mut brush_query {
        if let Some(&entity) = registry.get(selected.0)
            && tracked.0 != entity
        {
            tracked.0 = entity;
        }
    }
}

fn resize_brush(
    config: Res<PainterConfiguration>,
    mut single: Single<(&ActionState<PainterAction>, &mut ToolBrushSize)>,
) {
    let (action_state, brush_size) = (single.0, &mut single.1);
    let delta = action_state.value(&PainterAction::ChangeSize);
    config
        .brush
        .resize(brush_size, PAINTER_BRUSH_MIN_SIZE, delta);
}

fn brush_action_spawn_particles(
    mut msgw_spawn: MessageWriter<SpawnParticleSignal>,
    brush: Single<(&ToolBrushSize, &SelectedParticle)>,
    cursor: Res<Cursor>,
    brush_type: Res<State<PainterShape>>,
) {
    alg::get_positions(
        cursor.current,
        cursor.previous,
        cursor.previous_previous,
        brush.0.0,
        &brush_type,
    )
    .iter()
    .for_each(|pos| {
        msgw_spawn.write(SpawnParticleSignal::new(brush.1.0, *pos));
    });
}

fn brush_action_despawn_particles(
    mut commands: Commands,
    mut msgw_despawn: MessageWriter<DespawnParticleSignal>,
    brush_size: Single<&ToolBrushSize, With<PainterBrush>>,
    cursor: Res<Cursor>,
    brush_type: Res<State<PainterShape>>,
) {
    let positions = alg::get_positions(
        cursor.current,
        cursor.previous,
        cursor.previous_previous,
        brush_size.0,
        &brush_type,
    );

    for pos in &positions {
        msgw_despawn.write(DespawnParticleSignal::from_position(*pos));
    }

    if !positions.is_empty() {
        commands.trigger(RemoveFractureBodyCellsAtWorldPositions { positions });
    }
}

fn brush_action_spawn_conway(
    cursor: Res<Cursor>,
    map: Res<ParticleMap>,
    tex_origin: Res<WorldTextureOrigin>,
    brush: Single<&ToolBrushSize, With<PainterBrush>>,
    brush_type: Res<State<PainterShape>>,
    mut spawn_buf: ResMut<GolSpawnBuffer>,
) {
    let w = map.width() as i32;
    let h = map.height() as i32;

    let positions = crate::tools::painter::systems::alg::get_positions(
        cursor.current,
        cursor.previous,
        cursor.previous_previous,
        brush.0,
        &brush_type,
    );

    for pos in &positions {
        let tx = (pos.x - tex_origin.0.x).rem_euclid(w) as u32;
        let ty = (tex_origin.0.y + h - 1 - pos.y).rem_euclid(h) as u32;
        spawn_buf.positions.push(tx | (ty << 16));
    }
}

pub mod alg {
    use bevy::prelude::*;

    use crate::tools::painter::PainterShape;

    pub fn get_positions(
        p1: Vec2,
        p2: Vec2,
        p3: Vec2,
        brush_size: f32,
        brush_type: &PainterShape,
    ) -> Vec<IVec2> {
        let cursor_pairs = [(p1, p2), (p2, p3)];

        cursor_pairs
            .iter()
            .flat_map(|(start, end)| match brush_type {
                PainterShape::Circle => get_interpolated_circle_points(*start, *end, brush_size),
                PainterShape::Line => get_interpolated_line_points(*start, *end, brush_size),
                PainterShape::Cursor => get_interpolated_cursor_points(*start, *end),
            })
            .collect()
    }

    /// Find all horizontal lines interpolated between a start and end position.
    fn get_interpolated_line_points(start: Vec2, end: Vec2, line_length: f32) -> Vec<IVec2> {
        let mut positions = vec![];

        let min_x = -((line_length as i32) / 2) * 3;
        let max_x = (line_length as i32 / 2) * 3;

        let direction = (end - start).normalize();
        let length = (end - start).length();
        let num_samples = (length.ceil() as usize).max(1);

        for i in 0..=num_samples {
            let t = i as f32 / num_samples as f32;
            let sample_point = start + direction * length * t;

            for x_offset in min_x..=max_x {
                let position = IVec2::new(
                    (sample_point.x + x_offset as f32).floor() as i32,
                    sample_point.y.floor() as i32,
                );
                positions.push(position);
            }
        }

        positions
    }

    /// Find all cursor points interpolated between a start and end position.
    fn get_interpolated_cursor_points(start: Vec2, end: Vec2) -> Vec<IVec2> {
        if start == end {
            return vec![start.floor().as_ivec2()];
        }

        let mut positions = vec![];
        let direction = (end - start).normalize();
        let length = (end - start).length();
        let num_samples = (length.ceil() as usize).max(1);

        for i in 0..=num_samples {
            let t = i as f32 / num_samples as f32;
            positions.push((start + direction * length * t).floor().as_ivec2());
        }
        positions
    }

    /// Find all circles interpolated between a start and end position.
    fn get_interpolated_circle_points(start: Vec2, end: Vec2, radius: f32) -> Vec<IVec2> {
        let mut positions = vec![];
        if start == end {
            let min_x = (start.x - radius).floor() as i32;
            let max_x = (start.x + radius).ceil() as i32;
            let min_y = (start.y - radius).floor() as i32;
            let max_y = (start.y + radius).ceil() as i32;
            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let pos = Vec2::new(x as f32, y as f32);
                    if (pos - start).length() <= radius {
                        positions.push(pos.as_ivec2());
                    }
                }
            }
            return positions;
        } else {
            let length = (end - start).length();
            let direction = (end - start).normalize();

            let min_x = (start.x.min(end.x) - radius).floor() as i32;
            let max_x = (start.x.max(end.x) + radius).ceil() as i32;
            let min_y = (start.y.min(end.y) - radius).floor() as i32;
            let max_y = (start.y.max(end.y) + radius).ceil() as i32;

            for x in min_x..=max_x {
                for y in min_y..=max_y {
                    let point = Vec2::new(x as f32, y as f32);

                    let to_point = point - start;
                    let projected_length = to_point.dot(direction);
                    // Sometimes projected length will exceed the actual length of the capsule, so
                    // we need to clamp it.
                    let clamped_length = projected_length.clamp(0.0, length);

                    let closest_point = start + direction * clamped_length;
                    let distance_to_line = (point - closest_point).length();

                    if distance_to_line <= radius {
                        positions.push(IVec2::new(x, y));
                    }
                }
            }
        }

        positions
    }
}
