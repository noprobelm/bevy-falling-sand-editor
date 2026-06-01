mod render;

use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat, TextureUsages};
use bevy_falling_sand::prelude::*;

const SHADER_ASSET_PATH: &str = "shaders/game_of_life.wgsl";

/// Triggered to toggle the Conway simulation on or off.
#[derive(Event)]
pub struct GolToggleEvent;

/// When present, the Conway simulation is allowed to set up and run.
#[derive(Resource)]
struct GolEnabled;

pub struct GameOfLifePlugin;

impl Plugin for GameOfLifePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<GolSpawnBuffer>()
            .add_observer(on_gol_toggle)
            .add_systems(
                Update,
                setup_gol_textures.run_if(
                    resource_exists::<GolEnabled>
                        .and(resource_exists::<WorldColorTexture>)
                        .and(not(resource_exists::<GolTextures>)),
                ),
            )
            .add_systems(
                PreUpdate,
                clear_gol_spawn_buffer.run_if(resource_exists::<GolTextures>),
            )
            .add_systems(
                PostUpdate,
                flip_gol_buffers.run_if(resource_exists::<GolTextures>),
            );

        render::build_render_app(app);
    }

    fn finish(&self, app: &mut App) {
        render::finish_render_app(app);
    }
}

/// Double-buffered GoL textures (ping-pong each frame).
#[derive(Resource)]
pub struct GolTextures {
    pub a: Handle<Image>,
    pub b: Handle<Image>,
    /// When true, `a` is the input (read) and `b` is the output (write).
    pub current_is_a: bool,
}

/// Packed texel positions to stamp as alive this frame.
#[derive(Resource, Default)]
pub struct GolSpawnBuffer {
    pub positions: Vec<u32>,
}

/// Entity for the GoL overlay sprite.
#[derive(Resource)]
struct GolOverlayEntity(Entity);

fn setup_gol_textures(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    map: Res<ParticleMap>,
) {
    let width = map.width();
    let height = map.height();
    let origin = map.origin();

    let make_image = || {
        let data = vec![0u8; (width * height * 4) as usize];
        let mut img = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8Unorm,
            default(),
        );
        img.texture_descriptor.usage = TextureUsages::STORAGE_BINDING
            | TextureUsages::TEXTURE_BINDING
            | TextureUsages::COPY_DST;
        img
    };

    let handle_a = images.add(make_image());
    let handle_b = images.add(make_image());

    let center_x = origin.x as f32 + width as f32 / 2.0;
    let center_y = origin.y as f32 + height as f32 / 2.0;

    let entity = commands
        .spawn((
            Sprite {
                image: handle_a.clone(),
                custom_size: Some(Vec2::new(width as f32, height as f32)),
                ..default()
            },
            // Z above the effect overlay (which is at ~0.1)
            Transform::from_xyz(center_x, center_y, 0.2),
        ))
        .id();

    commands.insert_resource(GolTextures {
        a: handle_a,
        b: handle_b,
        current_is_a: true,
    });
    commands.insert_resource(GolOverlayEntity(entity));
}

fn clear_gol_spawn_buffer(mut spawn_buf: ResMut<GolSpawnBuffer>) {
    spawn_buf.positions.clear();
}

fn flip_gol_buffers(
    mut textures: ResMut<GolTextures>,
    mut sprite_query: Query<&mut Sprite>,
    overlay: Res<GolOverlayEntity>,
) {
    textures.current_is_a = !textures.current_is_a;

    // Point the sprite at the buffer that was just written (the output).
    // After flipping, the NEW input is what was the old output, so the
    // texture we want to display is the one that is NOW the input.
    let display_handle = if textures.current_is_a {
        textures.a.clone()
    } else {
        textures.b.clone()
    };

    if let Ok(mut sprite) = sprite_query.get_mut(overlay.0) {
        sprite.image = display_handle;
    }
}

fn on_gol_toggle(
    _trigger: On<GolToggleEvent>,
    mut commands: Commands,
    enabled: Option<Res<GolEnabled>>,
    overlay: Option<Res<GolOverlayEntity>>,
) {
    if enabled.is_some() {
        if let Some(overlay) = overlay {
            commands.entity(overlay.0).despawn();
            commands.remove_resource::<GolOverlayEntity>();
        }
        commands.remove_resource::<GolTextures>();
        commands.remove_resource::<GolEnabled>();
        info!("Conway simulation disabled");
    } else {
        commands.insert_resource(GolEnabled);
        info!("Conway simulation enabled");
    }
}
