pub const DEFAULT_PARTICLES_RON: &str = r#"(
  resources: {},
  entities: {
    4294966371: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.23137255,
                green: 0.2,
                blue: 0.2,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.2901961,
                green: 0.23921569,
                blue: 0.23921569,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.36078432,
                green: 0.2901961,
                blue: 0.2901961,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.4,
                green: 0.32941177,
                blue: 0.32941177,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Rock Wall",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966372: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.5803922,
                green: 0.70980394,
                blue: 0.78039217,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.87058824,
                green: 0.92941177,
                blue: 0.67058825,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.9411765,
                green: 0.8117647,
                blue: 0.4,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.8392157,
                green: 0.50980395,
                blue: 0.41960785,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.7411765,
                green: 0.30980393,
                blue: 0.41960785,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.9411765,
                green: 0.36078432,
                blue: 0.36862746,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Sparkly Slime",
        ),
        "bfs_editor::particle::ParticleCategory": ("Liquid"),
        "bfs_editor::chunk_effects::LiquidEffect": (),
        "bfs_movement::particle::Density": (850),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
            (
              neighbor_group: [
                (1, 0),
                (-1, 0),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::ParticleResistor": (0.5),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 2,
        ),
      },
    ),
    4294966373: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.54901963,
                green: 0.85882354,
                blue: 0.972549,
                alpha: 0.5019608,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Ice Wall",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
        "bfs_reactions::particle::Flammable": (
          duration: (
            secs: 2,
            nanos: 0,
          ),
          tick_rate: (
            secs: 0,
            nanos: 100000000,
          ),
          chance_despawn_per_tick: 0.01,
          reaction: Some((
            produces: (
              name: "Water",
            ),
            chance_to_produce: 0.2,
          )),
          color: None,
          chance_to_ignite: 0.0,
          spreads_fire: false,
          spread_radius: 1.0,
          despawn_on_extinguish: false,
          ignites_on_spawn: false,
        ),

      },
    ),
    4294966374: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Texture((
            path: "textures/created/wood_grain.png",
          )),
          assignment: Sequential,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Wood Wall",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
        "bfs_reactions::particle::Flammable": (
          duration: (
            secs: 10,
            nanos: 0,
          ),
          tick_rate: (
            secs: 0,
            nanos: 100000000,
          ),
          chance_despawn_per_tick: 0.0,
          reaction: Some((
            produces: (
              name: "Smoke",
            ),
            chance_to_produce: 0.035,
          )),
          color: Some((
            source: Palette((
              index: 0,
              colors: [
                Srgba((
                  red: 1.0,
                  green: 0.34901962,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.0,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.6,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.8117647,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.9098039,
                  blue: 0.03137255,
                  alpha: 1.0,
                )),
              ],
            )),
            assignment: Sequential,
          )),
          chance_to_ignite: 0.05,
          spreads_fire: true,
          spread_radius: 1.0,
          despawn_on_extinguish: true,
          ignites_on_spawn: false,
        ),

      },
    ),
    4294966375: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.47058824,
                green: 0.023529412,
                blue: 0.023529412,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Blood",
        ),
        "bfs_editor::particle::ParticleCategory": ("Liquid"),
        "bfs_movement::particle::Density": (800),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
            (
              neighbor_group: [
                (1, 0),
                (-1, 0),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
            (
              neighbor_group: [
                (3, 0),
                (-3, 0),
              ],
            ),
            (
              neighbor_group: [
                (4, 0),
                (-4, 0),
              ],
            ),
            (
              neighbor_group: [
                (5, 0),
                (-5, 0),
              ],
            ),
            (
              neighbor_group: [
                (6, 0),
                (-6, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::ParticleResistor": (0.5),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 3,
        ),
      },
    ),
    4294966376: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.8392157,
                green: 0.6,
                blue: 0.4392157,
                alpha: 0.5019608,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Whiskey",
        ),
        "bfs_editor::particle::ParticleCategory": ("Liquid"),
        "bfs_movement::particle::Density": (850),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
            (
              neighbor_group: [
                (1, 0),
                (-1, 0),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
            (
              neighbor_group: [
                (3, 0),
                (-3, 0),
              ],
            ),
            (
              neighbor_group: [
                (4, 0),
                (-4, 0),
              ],
            ),
            (
              neighbor_group: [
                (5, 0),
                (-5, 0),
              ],
            ),
            (
              neighbor_group: [
                (6, 0),
                (-6, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::ParticleResistor": (0.4),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 3,
        ),
      },
    ),
    4294966377: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Texture((
            path: "textures/created/flowered_grass.png",
          )),
          assignment: Sequential,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Grass Wall",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
        "bfs_reactions::particle::Flammable": (
          duration: (
            secs: 1,
            nanos: 0,
          ),
          tick_rate: (
            secs: 0,
            nanos: 100000000,
          ),
          chance_despawn_per_tick: 0.5,
          reaction: Some((
            produces: (
              name: "FIRE",
            ),
            chance_to_produce: 1.0,
          )),
          color: Some((
            source: Palette((
              index: 0,
              colors: [
                Srgba((
                  red: 1.0,
                  green: 0.34901962,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.0,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.6,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.8117647,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.9098039,
                  blue: 0.03137255,
                  alpha: 1.0,
                )),
              ],
            )),
            assignment: Sequential,
          )),
          chance_to_ignite: 0.36,
          spreads_fire: true,
          spread_radius: 1.0,
          despawn_on_extinguish: false,
          ignites_on_spawn: false,
        ),

      },
    ),
    4294966378: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.21960784,
                green: 0.10980392,
                blue: 0.15686275,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.23921569,
                green: 0.40784314,
                blue: 0.5568628,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.6666667,
                green: 0.7372549,
                blue: 0.54901963,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.9098039,
                green: 0.8862745,
                blue: 0.70980394,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.9490196,
                green: 0.60784316,
                blue: 0.42745098,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "My Custom Particle",
        ),
        "bfs_editor::particle::ParticleCategory": ("Movable Solid"),
        "bfs_movement::particle::Density": (1250),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::AirResistance": (resistances: [0.0, 0.4]),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 5,
          max: 10,
        ),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966379: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.16862746,
                green: 0.07058824,
                blue: 0.16078432,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Oil",
        ),
        "bfs_editor::particle::ParticleCategory": ("Liquid"),
        "bfs_movement::particle::Density": (730),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
            (
              neighbor_group: [
                (1, 0),
                (-1, 0),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
            (
              neighbor_group: [
                (3, 0),
                (-3, 0),
              ],
            ),
            (
              neighbor_group: [
                (4, 0),
                (-4, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::ParticleResistor": (0.5),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 3,
        ),
        "bfs_reactions::particle::Flammable": (
          duration: (
            secs: 5,
            nanos: 0,
          ),
          tick_rate: (
            secs: 0,
            nanos: 100000000,
          ),
          chance_despawn_per_tick: 0.1,
          reaction: Some((
            produces: (
              name: "Smoke",
            ),
            chance_to_produce: 0.035,
          )),
          color: Some((
            source: Palette((
              index: 0,
              colors: [
                Srgba((
                  red: 1.0,
                  green: 0.34901962,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.0,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.6,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.8117647,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.9098039,
                  blue: 0.03137255,
                  alpha: 1.0,
                )),
              ],
            )),
            assignment: Sequential,
          )),
          chance_to_ignite: 0.2,
          spreads_fire: true,
          spread_radius: 1.0,
          despawn_on_extinguish: false,
          ignites_on_spawn: false,
        ),

      },
    ),
    4294966380: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.4392157,
                green: 0.4117647,
                blue: 0.4,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.52156866,
                green: 0.5019608,
                blue: 0.4509804,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Smoke",
        ),
        "bfs_editor::particle::ParticleCategory": ("Gas"),
        "bfs_editor::chunk_effects::GasEffect": (),
        "bfs_movement::particle::Density": (275),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, 1),
                (1, 1),
                (-1, 1),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 1,
        ),
      },
    ),
    4294966381: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.5686275,
                green: 0.41960785,
                blue: 0.29803923,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.4509804,
                green: 0.34117648,
                blue: 0.23921569,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Dirt Wall",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966394: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.2666666,
                green: 0.3137254,
                blue: 0.3333333,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.2,
                green: 0.2352941,
                blue: 0.2509803,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Obsidian",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),

    4294966382: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.5686275,
                green: 0.41960785,
                blue: 0.29803923,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.4509804,
                green: 0.34117648,
                blue: 0.23921569,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Dirt",
        ),
        "bfs_editor::particle::ParticleCategory": ("Movable Solid"),
        "bfs_movement::particle::Density": (1250),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::AirResistance": (resistances: [0.0, 0.6]),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 5,
          max: 10,
        ),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966383: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.21960784,
                green: 0.10980392,
                blue: 0.15686275,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.23921569,
                green: 0.40784314,
                blue: 0.5568628,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.6666667,
                green: 0.7372549,
                blue: 0.54901963,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.9098039,
                green: 0.8862745,
                blue: 0.70980394,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.9490196,
                green: 0.60784316,
                blue: 0.42745098,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "My Custom Wall Particle",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966384: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.2509804,
                green: 0.38431373,
                blue: 0.09411765,
                alpha: 0.5019608,
              )),
              Srgba((
                red: 0.2901961,
                green: 0.4509804,
                blue: 0.10980392,
                alpha: 0.5019608,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Flammable Gas",
        ),
        "bfs_editor::particle::ParticleCategory": ("Gas"),
        "bfs_editor::chunk_effects::GasEffect": (),
        "bfs_movement::particle::Density": (200),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, 1),
                (1, 1),
                (-1, 1),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 1,
        ),
        "bfs_reactions::particle::Flammable": (
          duration: (
            secs: 1,
            nanos: 0,
          ),
          tick_rate: (
            secs: 0,
            nanos: 100000000,
          ),
          chance_despawn_per_tick: 0.5,
          reaction: None,
          color: Some((
            source: Palette((
              index: 0,
              colors: [
                Srgba((
                  red: 1.0,
                  green: 0.34901962,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.0,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.6,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.8117647,
                  blue: 0.0,
                  alpha: 1.0,
                )),
                Srgba((
                  red: 1.0,
                  green: 0.9098039,
                  blue: 0.03137255,
                  alpha: 1.0,
                )),
              ],
            )),
            assignment: Sequential,
          )),
          chance_to_ignite: 0.35,
          spreads_fire: true,
          spread_radius: 1.0,
          despawn_on_extinguish: false,
          ignites_on_spawn: false,
        ),

      },
    ),
    4294966385: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 1.0,
                green: 0.92156863,
                blue: 0.5411765,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.9490196,
                green: 0.8784314,
                blue: 0.41960785,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Sand",
        ),
        "bfs_editor::particle::ParticleCategory": ("Movable Solid"),
        "bfs_movement::particle::Density": (1250),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::AirResistance": (resistances: [0.0, 0.9]),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 5,
          max: 10,
        ),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966386: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 1.0,
                green: 0.34901962,
                blue: 0.0,
                alpha: 1.0,
              )),
              Srgba((
                red: 1.0,
                green: 0.5686275,
                blue: 0.0,
                alpha: 1.0,
              )),
              Srgba((
                red: 1.0,
                green: 0.8117647,
                blue: 0.0,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.78039217,
                green: 0.2901961,
                blue: 0.019607844,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "FIRE",
        ),
        "bfs_editor::particle::ParticleCategory": ("Gas"),
        "bfs_editor::chunk_effects::GasEffect": (),
        "bfs_editor::chunk_effects::BurnEffect": (),
        "bfs_movement::particle::Density": (450),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, 1),
                (1, 1),
                (-1, 1),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
            (
              neighbor_group: [
                (3, 0),
                (-3, 0),
              ],
            ),
            (
              neighbor_group: [
                (4, 0),
                (-4, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 3,
        ),
        "bfs_reactions::particle::Flammable": (
          duration: (
            secs: 1,
            nanos: 0,
          ),
          tick_rate: (
            secs: 0,
            nanos: 100000000,
          ),
          chance_despawn_per_tick: 0.5,
          reaction: None,
          color: None,
          chance_to_ignite: 0.0,
          spreads_fire: true,
          spread_radius: 1.0,
          despawn_on_extinguish: true,
          ignites_on_spawn: true,
        ),

      },
    ),
    4294966387: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.41960785,
                green: 0.4509804,
                blue: 0.54901963,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.54901963,
                green: 0.5882353,
                blue: 0.67058825,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.69803923,
                green: 0.76862746,
                blue: 0.8392157,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Rock",
        ),
        "bfs_editor::particle::ParticleCategory": ("Solid"),
        "bfs_movement::particle::Density": (1250),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 3,
        ),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966388: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.043137256,
                green: 0.5019608,
                blue: 0.67058825,
                alpha: 0.5019608,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_reactions::contact::ContactReaction": (
          rules: [
            (
              target: (name: "Slime"),
              becomes: (name: "Water"),
              chance: 0.005,
              radius: 1.0,
              consumes: Target,
            ),
            (
              target: (name: "Lava"),
              becomes: (name: "Obsidian"),
              chance: 0.45,
              radius: 1.0,
            ),
          ],
        ),
        "bfs_core::particle::ParticleType": (
          name: "Water",
        ),
        "bfs_editor::particle::ParticleCategory": ("Liquid"),
        "bfs_movement::particle::Density": (750),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
            (
              neighbor_group: [
                (1, 0),
                (-1, 0),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
            (
              neighbor_group: [
                (3, 0),
                (-3, 0),
              ],
            ),
            (
              neighbor_group: [
                (4, 0),
                (-4, 0),
              ],
            ),
            (
              neighbor_group: [
                (5, 0),
                (-5, 0),
              ],
            ),
            (
              neighbor_group: [
                (6, 0),
                (-6, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::ParticleResistor": (0.75),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 3,
        ),
      },
    ),
    4294966389: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.50980395,
                green: 0.59607846,
                blue: 0.20392157,
                alpha: 0.5019608,
              )),
              Srgba((
                red: 0.56078434,
                green: 0.654902,
                blue: 0.22352941,
                alpha: 0.5019608,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Slime",
        ),
        "bfs_editor::particle::ParticleCategory": ("Liquid"),
        "bfs_editor::chunk_effects::LiquidEffect": (),
        "bfs_movement::particle::Density": (850),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
            (
              neighbor_group: [
                (1, 0),
                (-1, 0),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::ParticleResistor": (0.6),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 2,
        ),
      },
    ),
    4294966390: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.93333334,
                green: 0.9490196,
                blue: 0.95686275,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.78039217,
                green: 0.8392157,
                blue: 0.8784314,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Steam",
        ),
        "bfs_editor::particle::ParticleCategory": ("Gas"),
        "bfs_editor::chunk_effects::GasEffect": (),
        "bfs_movement::particle::Density": (250),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, 1),
                (1, 1),
                (-1, 1),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
            (
              neighbor_group: [
                (3, 0),
                (-3, 0),
              ],
            ),
            (
              neighbor_group: [
                (4, 0),
                (-4, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 1,
        ),
        "bfs_reactions::particle::Flammable": (
          duration: (
            secs: 0,
            nanos: 200000000,
          ),
          tick_rate: (
            secs: 0,
            nanos: 100000000,
          ),
          chance_despawn_per_tick: 1.0,
          reaction: Some((
            produces: (
              name: "Water",
            ),
            chance_to_produce: 1.0,
          )),
          color: None,
          chance_to_ignite: 0.0,
          spreads_fire: false,
          spread_radius: 1.0,
          despawn_on_extinguish: false,
          ignites_on_spawn: false,
        ),

      },
    ),
    4294966391: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.91764706,
                green: 0.99215686,
                blue: 0.972549,
                alpha: 1.0,
              )),
              Srgba((
                red: 1.0,
                green: 1.0,
                blue: 1.0,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Snow",
        ),
        "bfs_editor::particle::ParticleCategory": ("Movable Solid"),
        "bfs_movement::particle::Density": (1250),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::AirResistance": (resistances: [0.0, 0.2]),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 5,
          max: 10,
        ),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966392: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.41960785,
                green: 0.4509804,
                blue: 0.54901963,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.54901963,
                green: 0.5882353,
                blue: 0.67058825,
                alpha: 1.0,
              )),
              Srgba((
                red: 0.69803923,
                green: 0.76862746,
                blue: 0.8392157,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Dense Rock Wall",
        ),
        "bfs_editor::particle::ParticleCategory": ("Wall"),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966393: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Gradient((
            start: Hsla((
              hue: 0.0,
              saturation: 1.0,
              lightness: 0.5,
              alpha: 1.0,
            )),
            end: Hsla((
              hue: 360.0,
              saturation: 1.0,
              lightness: 0.5,
              alpha: 1.0,
            )),
            index: 0,
            steps: 5000,
            hsv_interpolation: true,
          )),
          assignment: Sequential,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Colorful",
        ),
        "bfs_editor::particle::ParticleCategory": ("Movable Solid"),
        "bfs_movement::particle::Density": (1250),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::AirResistance": (resistances: [0.0, 0.4]),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 5,
          max: 10,
        ),
        "bfs_physics::StaticRigidBodyParticle": (),
      },
    ),
    4294966370: (
      components: {
        "bfs_color::particle::ColorProfile": (
          source: Palette((
            index: 0,
            colors: [
              Srgba((
                red: 0.9,
                green: 0.4,
                blue: 0.05,
                alpha: 1.0,
              )),
            ],
          )),
          assignment: Random,
        ),
        "bfs_core::particle::ParticleType": (
          name: "Lava",
        ),
        "bfs_editor::particle::ParticleCategory": ("Liquid"),
        "bfs_editor::chunk_effects::GlowEffect": (),
        "bfs_movement::particle::Density": (750),
        "bfs_movement::particle::Momentum": ((0, 0)),
        "bfs_movement::particle::Movement": (
          neighbor_groups: [
            (
              neighbor_group: [
                (0, -1),
              ],
            ),
            (
              neighbor_group: [
                (-1, -1),
                (1, -1),
              ],
            ),
            (
              neighbor_group: [
                (1, 0),
                (-1, 0),
              ],
            ),
            (
              neighbor_group: [
                (2, 0),
                (-2, 0),
              ],
            ),
          ],
        ),
        "bfs_movement::particle::ParticleResistor": (0.7),
        "bfs_movement::particle::Speed": (
          current: 1,
          threshold: 0,
          max: 2,
        ),
        "bfs_reactions::particle::Fire": (
          radius: 1.0,
        ),
      },
    ),
  },
)
"#;
