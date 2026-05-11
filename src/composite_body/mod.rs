use std::collections::HashMap;

use bevy::prelude::*;
use bevy_falling_sand::physics::dynamic::composite::{
    CompositeBodyPicker, CompositeImageLoader, DamageCompositePixel, PixelData,
    spawn_composite_body,
};
use bevy_falling_sand::prelude::ParticleType;
use bevy_falling_sand::reactions::Corrodible;

use crate::cursor::Cursor;

pub struct CompositeBodyDemoPlugin;

impl Plugin for CompositeBodyDemoPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_demo_composite_body)
            .add_systems(Update, (try_spawn_wood_square, handle_damage_input));
    }
}

#[derive(Resource)]
struct PendingWoodSquare(Handle<Image>);

/// Build a [`HashMap<IVec2, PixelData>`] from positions sharing one color —
/// convenience for monochrome demo bodies.
fn uniform_pixels(
    positions: impl IntoIterator<Item = IVec2>,
    color: Color,
) -> HashMap<IVec2, PixelData> {
    positions
        .into_iter()
        .map(|p| (p, PixelData::new(color)))
        .collect()
}

fn spawn_demo_composite_body(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    // 9x1 horizontal bar anchored at one end
    let bar = uniform_pixels(
        (-4..=4).map(|x| IVec2::new(x, 0)),
        Color::srgb(0.3, 0.7, 0.9),
    );
    spawn_composite_body(
        &mut commands,
        &mut images,
        Vec2::new(0.0, 60.0),
        Quat::IDENTITY,
        &bar,
        Vec2::ZERO,
        0.0,
        Some(IVec2::new(-4, 0)),
    );

    // Wood square: pixels are tagged with the "Wood Wall" ParticleType so the
    // body inherits its Flammable + Corrodible reactions via
    // derive_composite_body_reactions. Spawn deferred until the image decodes.
    commands.insert_resource(PendingWoodSquare(
        asset_server.load("textures/created/wood_grain.png"),
    ));

    // Asymmetric "rust" boulder: pixels eaten by adjacent Corrosive particles.
    let boulder_rows: [(i32, std::ops::RangeInclusive<i32>); 5] = [
        (2, -1..=1),
        (1, -2..=6),
        (0, -2..=4),
        (-1, -1..=4),
        (-2, 0..=3),
    ];
    let boulder = uniform_pixels(
        boulder_rows
            .into_iter()
            .flat_map(|(y, xs)| xs.map(move |x| IVec2::new(x, y))),
        Color::srgb(0.55, 0.5, 0.45),
    );
    let boulder_body = spawn_composite_body(
        &mut commands,
        &mut images,
        Vec2::new(-30.0, 110.0),
        Quat::IDENTITY,
        &boulder,
        Vec2::ZERO,
        0.4,
        None,
    );
    commands.entity(boulder_body).insert(Corrodible);

    // 5x3 horizontal-stripe flag
    let flag: HashMap<IVec2, PixelData> = (-1..=1_i32)
        .flat_map(|y| {
            let color = match y {
                1 => Color::srgb(0.9, 0.1, 0.1),
                0 => Color::srgb(0.95, 0.95, 0.95),
                -1 => Color::srgb(0.1, 0.2, 0.8),
                _ => Color::WHITE,
            };
            (-2..=2_i32).map(move |x| (IVec2::new(x, y), PixelData::new(color)))
        })
        .collect();
    spawn_composite_body(
        &mut commands,
        &mut images,
        Vec2::new(60.0, 90.0),
        Quat::IDENTITY,
        &flag,
        Vec2::ZERO,
        0.0,
        None,
    );
}

fn try_spawn_wood_square(
    mut commands: Commands,
    pending: Option<Res<PendingWoodSquare>>,
    mut loader: CompositeImageLoader,
) {
    let Some(pending) = pending else {
        return;
    };
    let spawned = loader.try_spawn(
        &mut commands,
        &pending.0,
        Vec2::new(20.0, 80.0),
        Quat::IDENTITY,
        Vec2::ZERO,
        0.0,
        None,
        Some(ParticleType::from("Wood Wall")),
    );
    if spawned.is_some() {
        commands.remove_resource::<PendingWoodSquare>();
    }
}

#[allow(clippy::needless_pass_by_value)]
fn handle_damage_input(
    cursor: Res<Cursor>,
    keys: Res<ButtonInput<KeyCode>>,
    picker: CompositeBodyPicker,
    mut damages: MessageWriter<DamageCompositePixel>,
) {
    if !keys.pressed(KeyCode::KeyX) {
        return;
    }
    let Some(hit) = picker.pick_at(cursor.current) else {
        return;
    };
    damages.write(DamageCompositePixel {
        body: hit.entity,
        local_pixel: hit.local_pixel,
    });
}
