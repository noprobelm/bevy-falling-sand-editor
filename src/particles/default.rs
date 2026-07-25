use std::time::Duration;

use crate::chunk_effects::{BurnEffect, GasEffect, GlowEffect, LiquidEffect};
use crate::particles::{ParticleCategory, ParticleName};
use bevy::prelude::*;
use bevy_falling_sand::prelude::*;

#[derive(Resource, Copy, Clone, Debug)]
pub struct DefaultParticleIds {
    pub rock_wall: ParticleTypeId,
    pub dirt_wall: ParticleTypeId,
    pub ice_wall: ParticleTypeId,
    pub wood_wall: ParticleTypeId,
    pub grass_wall: ParticleTypeId,
    pub dense_rock_wall: ParticleTypeId,
    pub obsidian: ParticleTypeId,
    pub custom_wall: ParticleTypeId,
    pub sand: ParticleTypeId,
    pub snow: ParticleTypeId,
    pub dirt: ParticleTypeId,
    pub custom: ParticleTypeId,
    pub colorful: ParticleTypeId,
    pub rock: ParticleTypeId,
    pub water: ParticleTypeId,
    pub acid: ParticleTypeId,
    pub slime: ParticleTypeId,
    pub congealed_slime: ParticleTypeId,
    pub sparkly_slime: ParticleTypeId,
    pub blood: ParticleTypeId,
    pub whiskey: ParticleTypeId,
    pub oil: ParticleTypeId,
    pub lava: ParticleTypeId,
    pub steam: ParticleTypeId,
    pub smoke: ParticleTypeId,
    pub fire: ParticleTypeId,
    pub flammable_gas: ParticleTypeId,
}

impl Default for DefaultParticleIds {
    fn default() -> Self {
        Self {
            rock_wall: ParticleTypeId::from_raw(0),
            dirt_wall: ParticleTypeId::from_raw(1),
            ice_wall: ParticleTypeId::from_raw(2),
            wood_wall: ParticleTypeId::from_raw(3),
            grass_wall: ParticleTypeId::from_raw(4),
            dense_rock_wall: ParticleTypeId::from_raw(5),
            obsidian: ParticleTypeId::from_raw(6),
            custom_wall: ParticleTypeId::from_raw(7),
            sand: ParticleTypeId::from_raw(8),
            snow: ParticleTypeId::from_raw(9),
            dirt: ParticleTypeId::from_raw(10),
            custom: ParticleTypeId::from_raw(11),
            colorful: ParticleTypeId::from_raw(12),
            rock: ParticleTypeId::from_raw(13),
            water: ParticleTypeId::from_raw(14),
            acid: ParticleTypeId::from_raw(15),
            slime: ParticleTypeId::from_raw(16),
            congealed_slime: ParticleTypeId::from_raw(17),
            sparkly_slime: ParticleTypeId::from_raw(18),
            blood: ParticleTypeId::from_raw(19),
            whiskey: ParticleTypeId::from_raw(20),
            oil: ParticleTypeId::from_raw(21),
            lava: ParticleTypeId::from_raw(22),
            steam: ParticleTypeId::from_raw(23),
            smoke: ParticleTypeId::from_raw(24),
            fire: ParticleTypeId::from_raw(25),
            flammable_gas: ParticleTypeId::from_raw(26),
        }
    }
}

fn palette(colors: Vec<Color>) -> ColorProfile {
    ColorProfile {
        source: ColorSource::Palette(Palette { index: 0, colors }),
        assignment: ColorAssignment::Random,
    }
}

fn texture(path: &str) -> ColorProfile {
    ColorProfile::texture(path)
}

fn particle_type(id: ParticleTypeId, name: &str) -> (ParticleType, ParticleName) {
    (ParticleType::from_id(id), ParticleName(name.to_string()))
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

pub(super) fn spawn_default_particles(commands: &mut Commands) {
    let ids = DefaultParticleIds::default();

    // ── Walls ──

    commands.spawn((
        particle_type(ids.rock_wall, "Rock Wall"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.23137255, 0.2, 0.2, 1.0),
            Color::srgba(0.2901961, 0.23921569, 0.23921569, 1.0),
            Color::srgba(0.36078432, 0.2901961, 0.2901961, 1.0),
            Color::srgba(0.4, 0.32941177, 0.32941177, 1.0),
        ]),
        StaticRigidBodyParticle,
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.dirt_wall, "Dirt Wall"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.5686275, 0.41960785, 0.29803923, 1.0),
            Color::srgba(0.4509804, 0.34117648, 0.23921569, 1.0),
        ]),
        StaticRigidBodyParticle,
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.ice_wall, "Ice Wall"),
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
                produces: ids.water,
                chance_to_produce: 0.2,
            }),
            chance_to_ignite: 0.0,
            spreads_fire: false,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.wood_wall, "Wood Wall"),
        ParticleCategory("Wall".into()),
        texture("textures/created/wood_grain.png"),
        StaticRigidBodyParticle,
        Flammable {
            duration: Duration::from_secs(10),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.015,
            reaction: Some(BurnProduct {
                produces: ids.smoke,
                chance_to_produce: 0.035,
            }),
            chance_to_ignite: 0.2,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: true,
            ignites_on_spawn: false,
        },
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.grass_wall, "Grass Wall"),
        ParticleCategory("Wall".into()),
        texture("textures/created/flowered_grass.png"),
        StaticRigidBodyParticle,
        Flammable {
            duration: Duration::from_secs(1),
            tick_rate: Duration::from_millis(100),
            chance_despawn_per_tick: 0.5,
            reaction: Some(BurnProduct {
                produces: ids.fire,
                chance_to_produce: 1.0,
            }),
            chance_to_ignite: 0.36,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.dense_rock_wall, "Dense Rock Wall"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.41960785, 0.4509804, 0.54901963, 1.0),
            Color::srgba(0.54901963, 0.5882353, 0.67058825, 1.0),
            Color::srgba(0.69803923, 0.76862746, 0.8392157, 1.0),
        ]),
        StaticRigidBodyParticle,
    ));

    commands.spawn((
        particle_type(ids.obsidian, "Obsidian"),
        ParticleCategory("Wall".into()),
        palette(vec![
            Color::srgba(0.2666666, 0.3137254, 0.3333333, 1.0),
            Color::srgba(0.2, 0.2352941, 0.2509803, 1.0),
        ]),
        StaticRigidBodyParticle,
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.custom_wall, "My Custom Wall Particle"),
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
        particle_type(ids.sand, "Sand"),
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
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.snow, "Snow"),
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
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.dirt, "Dirt"),
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
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.custom, "My Custom Particle"),
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
        particle_type(ids.colorful, "Colorful"),
        ParticleCategory("Movable Solid".into()),
        ColorProfile {
            source: ColorSource::Gradient(ColorGradient {
                colors: vec![
                    Color::hsla(0.0, 1.0, 0.5, 1.0),
                    Color::hsla(360.0, 1.0, 0.5, 1.0),
                ],
                steps: vec![5000],
                index: 0,
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
        Corrodible,
    ));

    // ── Solid ──

    commands.spawn((
        particle_type(ids.rock, "Rock"),
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
        particle_type(ids.water, "Water"),
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
                    target: ids.slime,
                    becomes: ids.water,
                    chance: 0.005,
                    radius: 1.0,
                    consumes: Consumes::Target,
                },
                ContactRule {
                    target: ids.lava,
                    becomes: ids.obsidian,
                    chance: 0.45,
                    radius: 1.0,
                    consumes: Consumes::Source,
                },
                ContactRule {
                    target: ids.acid,
                    becomes: ids.steam,
                    chance: 1.0,
                    radius: 1.0,
                    consumes: Consumes::Source,
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
        particle_type(ids.acid, "Acid"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(0.25490198, 0.6862745, 0.0, 1.)]),
        Density(750),
        Momentum(IVec2::ZERO),
        liquid_movement(6),
        ParticleResistor(0.75),
        Speed::new(0, 3),
        Corrosive::new(0.01, Duration::from_millis(100)),
        ContactReaction {
            rules: vec![
                ContactRule {
                    target: ids.water,
                    becomes: ids.steam,
                    chance: 1.0,
                    radius: 1.0,
                    consumes: Consumes::Target,
                },
                ContactRule {
                    target: ids.slime,
                    becomes: ids.congealed_slime,
                    chance: 1.0,
                    radius: 1.0,
                    consumes: Consumes::Target,
                },
            ],
        },
    ));

    commands.spawn((
        particle_type(ids.slime, "Slime"),
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
        ContactReaction {
            rules: vec![ContactRule {
                target: ids.acid,
                becomes: ids.congealed_slime,
                chance: 1.0,
                radius: 1.0,
                consumes: Consumes::Source,
            }],
        },
    ));

    commands.spawn((
        particle_type(ids.congealed_slime, "Congealed Slime"),
        ParticleCategory("Liquid".into()),
        palette(vec![
            Color::srgba(0.50980395, 0.59607846, 0.20392157, 0.5019608),
            Color::srgba(0.56078434, 0.654902, 0.22352941, 0.5019608),
        ]),
        LiquidEffect,
        Density(850),
        Momentum(IVec2::ZERO),
        liquid_movement(1),
        ParticleResistor(0.8),
        Speed::new(0, 2),
    ));

    commands.spawn((
        particle_type(ids.sparkly_slime, "Sparkly Slime"),
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
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.blood, "Blood"),
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
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.whiskey, "Whiskey"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(0.8392157, 0.6, 0.4392157, 0.5019608)]),
        Density(850),
        Momentum(IVec2::ZERO),
        liquid_movement(6),
        ParticleResistor(0.4),
        Speed::new(0, 3),
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.oil, "Oil"),
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
                produces: ids.smoke,
                chance_to_produce: 0.035,
            }),
            chance_to_ignite: 0.2,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.lava, "Lava"),
        ParticleCategory("Liquid".into()),
        palette(vec![Color::srgba(0.9, 0.4, 0.05, 1.0)]),
        GlowEffect,
        Density(750),
        Momentum(IVec2::ZERO),
        liquid_movement(2),
        ParticleResistor(0.7),
        Speed::new(0, 2),
        Fire { radius: 1.0 },
        ContactReaction {
            rules: vec![ContactRule {
                target: ids.acid,
                becomes: ids.flammable_gas,
                chance: 1.0,
                radius: 1.0,
                consumes: Consumes::Target,
            }],
        },
    ));

    // ── Gases ──

    commands.spawn((
        particle_type(ids.steam, "Steam"),
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
                produces: ids.water,
                chance_to_produce: 1.0,
            }),
            chance_to_ignite: 0.0,
            spreads_fire: false,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
        Corrodible,
    ));

    commands.spawn((
        particle_type(ids.smoke, "Smoke"),
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
        particle_type(ids.fire, "FIRE"),
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
            chance_to_ignite: 0.0,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: true,
            ignites_on_spawn: true,
        },
    ));

    commands.spawn((
        particle_type(ids.flammable_gas, "Flammable Gas"),
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
            chance_to_ignite: 0.35,
            spreads_fire: true,
            spread_radius: 1.0,
            despawn_on_extinguish: false,
            ignites_on_spawn: false,
        },
    ));
}
