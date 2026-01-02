use bevy::{prelude::*, render::render_resource::AsBindGroup};
use bevy_shader::ShaderRef;
use bevy_sprite_render::{AlphaMode2d, Material2d, Material2dPlugin};

use crate::cursor::CursorPosition;

const CURSOR_GRID_SHADER_PATH: &str = "shaders/cursor_guide.wgsl";

pub struct OverlaysPlugin;

impl Plugin for OverlaysPlugin {
    fn build(&self, app: &mut App) {
        app.init_gizmo_group::<OverlaysGizmos>()
            .add_plugins(Material2dPlugin::<CursorGuideMaterial>::default())
            .add_systems(Startup, setup_cursor_grid)
            .add_systems(Update, update_cursor_grid);
    }
}

#[derive(Default, Reflect, GizmoConfigGroup)]
pub struct OverlaysGizmos;

#[derive(Resource, Clone, Default, Debug)]
pub struct DrawCursorGuide;

#[derive(Asset, TypePath, AsBindGroup, Debug, Clone)]
pub struct CursorGuideMaterial {
    #[uniform(0)]
    pub cursor_world_pos: Vec2,
    #[uniform(0)]
    pub grid_size: Vec2,
    #[uniform(0)]
    pub line_color: LinearRgba,
    #[uniform(0)]
    pub line_width: f32,
    #[uniform(0)]
    pub fade_power: f32,
    #[uniform(0)]
    pub fade_end: f32,
}

impl Default for CursorGuideMaterial {
    fn default() -> Self {
        Self {
            cursor_world_pos: Vec2::ZERO,
            grid_size: Vec2::new(1024.0, 1024.0),
            line_color: LinearRgba::new(36. / 255., 49. / 255., 60. / 255., 0.9),
            line_width: 0.08,
            fade_power: 2.0,
            fade_end: 12.0,
        }
    }
}

impl Material2d for CursorGuideMaterial {
    fn fragment_shader() -> ShaderRef {
        CURSOR_GRID_SHADER_PATH.into()
    }

    fn alpha_mode(&self) -> AlphaMode2d {
        AlphaMode2d::Blend
    }
}

#[derive(Component)]
pub struct CursorGridOverlay;

/// Size of the cursor grid overlay in world units.
const CURSOR_GRID_SIZE: f32 = 50.0;

fn setup_cursor_grid(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<CursorGuideMaterial>>,
    draw_cursor_guide: Option<Res<DrawCursorGuide>>,
) {
    let mesh_handle = meshes.add(Rectangle::new(CURSOR_GRID_SIZE, CURSOR_GRID_SIZE));

    let material = CursorGuideMaterial {
        grid_size: Vec2::splat(CURSOR_GRID_SIZE),
        ..default()
    };
    let material_handle = materials.add(material);

    let entity = commands
        .spawn((
            CursorGridOverlay,
            Mesh2d(mesh_handle),
            MeshMaterial2d(material_handle),
            Transform::from_translation(Vec3::new(0.0, 0.0, 100.0)),
        ))
        .id();
    if draw_cursor_guide.is_none() {
        commands.entity(entity).insert(Visibility::Hidden);
    }
}

fn update_cursor_grid(
    cursor_position: Res<CursorPosition>,
    draw_guide: Option<Res<DrawCursorGuide>>,
    mut materials: ResMut<Assets<CursorGuideMaterial>>,
    mut query: Query<
        (
            &MeshMaterial2d<CursorGuideMaterial>,
            &mut Transform,
            &mut Visibility,
        ),
        With<CursorGridOverlay>,
    >,
) {
    for (material_handle, mut transform, mut visibility) in &mut query {
        if draw_guide.is_some() {
            *visibility = Visibility::Visible;
            // Move the grid to follow the cursor
            transform.translation.x = cursor_position.current.x;
            transform.translation.y = cursor_position.current.y;
            if let Some(material) = materials.get_mut(&material_handle.0) {
                material.cursor_world_pos = cursor_position.current;
            }
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}
