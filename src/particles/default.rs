use std::time::Duration;

use crate::chunk_effects::{BurnEffect, GasEffect, GlowEffect, LiquidEffect};
use crate::particles::ParticleCategory;
use bevy::prelude::*;
use bevy_falling_sand::prelude::*;

fn palette(colors: Vec<Color>) -> ColorProfile {
    ColorProfile {
        source: ColorSource::Palette(Palette { index: 0, colors }),
        assignment: ColorAssignment::Random,
    }
}

fn texture(path: &str) -> ColorProfile {
    ColorProfile::texture(path)
}

fn movable_solid_movement() -> Movement {
    Movement::new(
        vec![
            NeighborGroup::new(vec![IVec2::new(0, -1)].into()),
            NeighborGroup::new(vec![IVec2::new(-1, -1), IVec2::new(1, -1)].into()),
        ]
        .into(),
    )
}

fn liquid_movement(spread: i32) -> Movement {
    let mut groups = vec![
        NeighborGroup::new(vec![IVec2::new(0, -1)].into()),
        NeighborGroup::new(vec![IVec2::new(-1, -1), IVec2::new(1, -1)].into()),
    ];
    for i in 1..=spread {
        groups.push(NeighborGroup::new(
            vec![IVec2::new(i, 0), IVec2::new(-i, 0)].into(),
        ));
    }
    Movement::new(groups.into())
}

fn gas_movement(horizontal_spread: i32) -> Movement {
    let mut groups = vec![NeighborGroup::new(
        vec![IVec2::new(0, 1), IVec2::new(1, 1), IVec2::new(-1, 1)].into(),
    )];
    for i in 0..horizontal_spread {
        let dist = i + 2;
        groups.push(NeighborGroup::new(
            vec![IVec2::new(dist, 0), IVec2::new(-dist, 0)].into(),
        ));
    }
    Movement::new(groups.into())
}

fn fire_color_profile() -> ColorProfile {
    ColorProfile {
        source: ColorSource::Palette(Palette {
            index: 0,
            colors: vec![
                Color::srgba(1.0, 0.34901962, 0.0, 1.0),
                Color::srgba(1.0, 0.0, 0.0, 1.0),
                Color::srgba(1.0, 0.6, 0.0, 1.0),
                Color::srgba(1.0, 0.8117647, 0.0, 1.0),
                Color::srgba(1.0, 0.9098039, 0.03137255, 1.0),
            ],
        }),
        assignment: ColorAssignment::Sequential,
    }
}

pub(super) fn spawn_default_particles(commands: &mut Commands) {
    // ── Walls ──

    commands.spawn((
        ParticleType::from("Rock Wall"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.23137255, 0.2, 0.2, 1.0),
            Color::srgba(0.2901961, 0.23921569, 0.23921569, 1.0),
            Color::srgba(0.36078432, 0.2901961, 0.2901961, 1.0),
            Color::srgba(0.4, 0.32941177, 0.32941177, 1.0),
        ]),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("Dirt Wall"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.5686275, 0.41960785, 0.29803923, 1.0),
            Color::srgba(0.4509804, 0.34117648, 0.23921569, 1.0),
        ]),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("Ice Wall"),
        ParticleCategory("Wall".into()),
        palette(vec![Color::srgba(
            0.54901963, 0.85882354, 0.972549, 0.5019608,
        )]),
        StaticRigidBodyParticle,
        Flammable {
            duration: Duration::from_secs(2),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.01,
            reaction: Some(BurnProduct {
                produces: Particle::from("Water"),
                chance_to_produce: 0.2,
            }),
            color: None,
            chance_to_ignite: 0.0,
            spreads_fire: false,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
    ));

    commands.spawn((
        ParticleType::from("Wood Wall"),
        ParticleCategory("Wall".into()),
        texture("textures/created/wood_grain.png"),
        StaticRigidBodyParticle,
        Flammable {
            duration: Duration::from_secs(10),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.015,
            reaction: Some(BurnProduct {
                produces: Particle::from("Smoke"),
                chance_to_produce: 0.035,
            }),
            color: Some(fire_color_profile()),
            chance_to_ignite: 0.2,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: true,
            ignites_on_spawn: false,
        },
    ));

    commands.spawn((
        ParticleType::from("Grass Wall"),
        ParticleCategory("Wall".into()),
        texture("textures/created/flowered_grass.png"),
        StaticRigidBodyParticle,
        Flammable {
            duration: Duration::from_secs(1),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.5,
            reaction: Some(BurnProduct {
                produces: Particle::from("FIRE"),
                chance_to_produce: 1.0,
            }),
            color: Some(fire_color_profile()),
            chance_to_ignite: 0.36,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
    ));

    commands.spawn((
        ParticleType::from("Dense Rock Wall"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.41960785, 0.4509804, 0.54901963, 1.0),
            Color::srgba(0.54901963, 0.5882353, 0.67058825, 1.0),
            Color::srgba(0.69803923, 0.76862746, 0.8392157, 1.0),
        ]),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("Obsidian"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.2666666, 0.3137254, 0.3333333, 1.0),
            Color::srgba(0.2, 0.2352941, 0.2509803, 1.0),
        ]),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("My Custom Wall Particle"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.21960784, 0.10980392, 0.15686275, 1.0),
            Color::srgba(0.23921569, 0.40784314, 0.5568628, 1.0),
            Color::srgba(0.6666667, 0.7372549, 0.54901963, 1.0),
            Color::srgba(0.9098039, 0.8862745, 0.70980394, 1.0),
            Color::srgba(0.9490196, 0.60784316, 0.42745098, 1.0),
        ]),
        StaticRigidBodyParticle,
    ));

    // ── Movable Solids ──

    commands.spawn((
        ParticleType::from("Sand"),
        ParticleCategory("Movable Solid".into()),
        palette(vec![
            Color::srgba(1.0, 0.92156863, 0.5411765, 1.0),
            Color::srgba(0.9490196, 0.8784314, 0.41960785, 1.0),
        ]),
        Density(1250),
        Momentum(IVec2::ZERO),
        movable_solid_movement(),
        AirResistance::new([0.0, 0.9]),
        Speed::new(5, 10),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("Snow"),
        ParticleCategory("Movable Solid".into()),
        palette(vec![
            Color::srgba(0.91764706, 0.99215686, 0.972549, 1.0),
            Color::srgba(1.0, 1.0, 1.0, 1.0),
        ]),
        Density(1250),
        Momentum(IVec2::ZERO),
        movable_solid_movement(),
        AirResistance::new([0.0, 0.2]),
        Speed::new(5, 10),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("Dirt"),
        ParticleCategory("Movable Solid".into()),
        palette(vec![
            Color::srgba(0.5686275, 0.41960785, 0.29803923, 1.0),
            Color::srgba(0.4509804, 0.34117648, 0.23921569, 1.0),
        ]),
        Density(1250),
        Momentum(IVec2::ZERO),
        movable_solid_movement(),
        AirResistance::new([0.0, 0.6]),
        Speed::new(5, 10),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("My Custom Particle"),
        ParticleCategory("Movable Solid".into()),
        palette(vec![
            Color::srgba(0.21960784, 0.10980392, 0.15686275, 1.0),
            Color::srgba(0.23921569, 0.40784314, 0.5568628, 1.0),
            Color::srgba(0.6666667, 0.7372549, 0.54901963, 1.0),
            Color::srgba(0.9098039, 0.8862745, 0.70980394, 1.0),
            Color::srgba(0.9490196, 0.60784316, 0.42745098, 1.0),
        ]),
        Density(1250),
        Momentum(IVec2::ZERO),
        movable_solid_movement(),
        AirResistance::new([0.0, 0.4]),
        Speed::new(5, 10),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        ParticleType::from("Colorful"),
        ParticleCategory("Movable Solid".into()),
        ColorProfile {
            source: ColorSource::Gradient(ColorGradient {
                start: Color::hsla(0.0, 1.0, 0.5, 1.0),
                end: Color::hsla(360.0, 1.0, 0.5, 1.0),
                index: 0,
                steps: 5000,
                hsv_interpolation: true,
            }),
            assignment: ColorAssignment::Sequential,
        },
        Density(1250),
        Momentum(IVec2::ZERO),
        movable_solid_movement(),
        AirResistance::new([0.0, 0.4]),
        Speed::new(5, 10),
        StaticRigidBodyParticle,
    ));

    // ── Solid ──

    commands.spawn((
        ParticleType::from("Rock"),
        ParticleCategory("Solid".into()),
        palette(vec![
            Color::srgba(0.41960785, 0.4509804, 0.54901963, 1.0),
            Color::srgba(0.54901963, 0.5882353, 0.67058825, 1.0),
            Color::srgba(0.69803923, 0.76862746, 0.8392157, 1.0),
        ]),
        Density(1250),
        Movement::new(vec![NeighborGroup::new(vec![IVec2::new(0, -1)].into())].into()),
        Speed::new(0, 3),
        StaticRigidBodyParticle,
    ));

    // ── Liquids ──

    commands.spawn((
        ParticleType::from("Water"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(
            0.043137256,
            0.5019608,
            0.67058825,
            0.5019608,
        )]),
        ContactReaction {
            rules: vec![
                ContactRule {
                    target: Particle::from("Slime"),
                    becomes: Particle::from("Water"),
                    chance: 0.005,
                    radius: 1.0,
                    consumes: Consumes::Target,
                },
                ContactRule {
                    target: Particle::from("Lava"),
                    becomes: Particle::from("Obsidian"),
                    chance: 0.45,
                    radius: 1.0,
                    consumes: Consumes::default(),
                },
            ],
        },
        Density(750),
        Momentum(IVec2::ZERO),
        liquid_movement(6),
        ParticleResistor(0.75),
        Speed::new(0, 3),
    ));

    commands.spawn((
        ParticleType::from("Slime"),
        ParticleCategory("Liquid".into()),
        palette(vec![
            Color::srgba(0.50980395, 0.59607846, 0.20392157, 0.5019608),
            Color::srgba(0.56078434, 0.654902, 0.22352941, 0.5019608),
        ]),
        LiquidEffect,
        Density(850),
        Momentum(IVec2::ZERO),
        liquid_movement(2),
        ParticleResistor(0.6),
        Speed::new(0, 2),
    ));

    commands.spawn((
        ParticleType::from("Sparkly Slime"),
        ParticleCategory("Liquid".into()),
        palette(vec![
            Color::srgba(0.5803922, 0.70980394, 0.78039217, 1.0),
            Color::srgba(0.87058824, 0.92941177, 0.67058825, 1.0),
            Color::srgba(0.9411765, 0.8117647, 0.4, 1.0),
            Color::srgba(0.8392157, 0.50980395, 0.41960785, 1.0),
            Color::srgba(0.7411765, 0.30980393, 0.41960785, 1.0),
            Color::srgba(0.9411765, 0.36078432, 0.36862746, 1.0),
        ]),
        LiquidEffect,
        Density(850),
        Momentum(IVec2::ZERO),
        liquid_movement(2),
        ParticleResistor(0.5),
        Speed::new(0, 2),
    ));

    commands.spawn((
        ParticleType::from("Blood"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(
            0.47058824,
            0.023529412,
            0.023529412,
            1.0,
        )]),
        Density(800),
        Momentum(IVec2::ZERO),
        liquid_movement(6),
        ParticleResistor(0.5),
        Speed::new(0, 3),
    ));

    commands.spawn((
        ParticleType::from("Whiskey"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(0.8392157, 0.6, 0.4392157, 0.5019608)]),
        Density(850),
        Momentum(IVec2::ZERO),
        liquid_movement(6),
        ParticleResistor(0.4),
        Speed::new(0, 3),
    ));

    commands.spawn((
        ParticleType::from("Oil"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(0.16862746, 0.07058824, 0.16078432, 1.0)]),
        Density(730),
        Momentum(IVec2::ZERO),
        liquid_movement(4),
        ParticleResistor(0.5),
        Speed::new(0, 3),
        Flammable {
            duration: Duration::from_secs(5),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.1,
            reaction: Some(BurnProduct {
                produces: Particle::from("Smoke"),
                chance_to_produce: 0.035,
            }),
            color: Some(fire_color_profile()),
            chance_to_ignite: 0.2,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
    ));

    commands.spawn((
        ParticleType::from("Lava"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(0.9, 0.4, 0.05, 1.0)]),
        GlowEffect,
        Density(750),
        Momentum(IVec2::ZERO),
        liquid_movement(2),
        ParticleResistor(0.7),
        Speed::new(0, 2),
        Fire { radius: 1.0 },
    ));

    // ── Gases ──

    commands.spawn((
        ParticleType::from("Steam"),
        ParticleCategory("Gas".into()),
        palette(vec![
            Color::srgba(0.93333334, 0.9490196, 0.95686275, 1.0),
            Color::srgba(0.78039217, 0.8392157, 0.8784314, 1.0),
        ]),
        GasEffect,
        Density(250),
        gas_movement(3),
        Speed::new(0, 1),
        Flammable {
            duration: Duration::from_millis(200),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 1.0,
            reaction: Some(BurnProduct {
                produces: Particle::from("Water"),
                chance_to_produce: 1.0,
            }),
            color: None,
            chance_to_ignite: 0.0,
            spreads_fire: false,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
    ));

    commands.spawn((
        ParticleType::from("Smoke"),
        ParticleCategory("Gas".into()),
        palette(vec![
            Color::srgba(0.36862746, 0.34117648, 0.32941177, 1.0),
            Color::srgba(0.4392157, 0.4117647, 0.4, 1.0),
            Color::srgba(0.52156866, 0.5019608, 0.4509804, 1.0),
        ]),
        GasEffect,
        Density(275),
        gas_movement(1),
        Speed::new(0, 1),
    ));

    commands.spawn((
        ParticleType::from("FIRE"),
        ParticleCategory("Gas".into()),
        palette(vec![
            Color::srgba(1.0, 0.34901962, 0.0, 1.0),
            Color::srgba(1.0, 0.5686275, 0.0, 1.0),
            Color::srgba(1.0, 0.8117647, 0.0, 1.0),
            Color::srgba(0.78039217, 0.2901961, 0.019607844, 1.0),
        ]),
        GasEffect,
        BurnEffect,
        Density(450),
        gas_movement(3),
        Speed::new(0, 3),
        Flammable {
            duration: Duration::from_secs(1),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.5,
            reaction: None,
            color: None,
            chance_to_ignite: 0.0,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: true,
            ignites_on_spawn: true,
        },
    ));

    commands.spawn((
        ParticleType::from("Flammable Gas"),
        ParticleCategory("Gas".into()),
        palette(vec![
            Color::srgba(0.2509804, 0.38431373, 0.09411765, 0.5019608),
            Color::srgba(0.2901961, 0.4509804, 0.10980392, 0.5019608),
        ]),
        GasEffect,
        Density(200),
        gas_movement(1),
        Speed::new(0, 1),
        Flammable {
            duration: Duration::from_secs(1),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.5,
            reaction: None,
            color: Some(fire_color_profile()),
            chance_to_ignite: 0.35,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
    ));
}
