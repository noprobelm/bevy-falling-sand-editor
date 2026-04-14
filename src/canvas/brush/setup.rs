use bevy::prelude::*;
use bevy_falling_sand::prelude::*;
use bevy_persistent::Persistent;
use leafwing_input_manager::{
    Actionlike,
    plugin::InputManagerPlugin,
    prelude::{InputMap, MouseScrollAxis},
};
use serde::{Deserialize, Serialize};

use crate::{
    canvas::brush::{
        BrushSpawnState, BrushTypeState,
        components::{Brush, BrushColor, BrushSize, SelectedParticle, SelectedParticleType},
        gizmos::BrushGizmos,
    },
    config::{InputButton, SettingsConfig},
    setup::SetupSystems,
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<BrushAction>::default())
            .insert_gizmo_config(
                BrushGizmos,
                GizmoConfig {
                    enabled: true,
                    ..default()
                },
            )
            .add_systems(
                Startup,
                (spawn_brush, load_settings)
                    .chain()
                    .in_set(SetupSystems::Canvas),
            )
            .add_systems(
                Update,
                insert_selected_particle.run_if(condition_setup_brush_particle_ready),
            );
    }
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct BrushKeyBindings {
    pub draw: InputButton,
    pub toggle_brush_mode: InputButton,
}

impl Default for BrushKeyBindings {
    fn default() -> Self {
        Self {
            draw: MouseButton::Left.into(),
            toggle_brush_mode: MouseButton::Right.into(),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum BrushAction {
    ToggleMode,
    ToggleType,
    #[actionlike(Axis)]
    ChangeSize,
}

fn spawn_brush(mut commands: Commands) {
    commands.spawn((
        Brush,
        BrushSize(2),
        BrushColor(Color::Srgba(Srgba::new(1., 1., 1., 0.3))),
    ));
}

fn insert_selected_particle(
    mut commands: Commands,
    registry: Res<ParticleTypeRegistry>,
    particle_types: Query<&ParticleType>,
    brush: Single<Entity, With<Brush>>,
) {
    const DEFAULT_PARTICLE_NAME: &str = "Dirt Wall";
    let pt_entity = if let Some(entity) = registry.get(DEFAULT_PARTICLE_NAME) {
        *entity
    } else {
        *registry
            .entities()
            .next()
            .expect("No particle types found in the world")
    };

    let particle = Particle::from(
        particle_types
            .get(pt_entity)
            .expect("Failed to find particle type in query")
            .clone(),
    );

    commands
        .entity(brush.entity())
        .insert((SelectedParticle(particle), SelectedParticleType(pt_entity)));
}

fn load_settings(
    mut commands: Commands,
    mut next_brush_type_state: ResMut<NextState<BrushTypeState>>,
    mut next_brush_mode_state: ResMut<NextState<BrushSpawnState>>,
    brush: Single<Entity, With<Brush>>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let keys = &settings_config.keys.brush;
    let mut input_map = InputMap::default().with_axis(BrushAction::ChangeSize, MouseScrollAxis::Y);
    keys.toggle_brush_mode
        .insert_into_input_map(&mut input_map, BrushAction::ToggleMode);

    commands
        .entity(brush.entity())
        .insert((input_map, settings_config.brush.size));
    commands.insert_resource(settings_config.keys.brush.clone());
    next_brush_type_state.set(settings_config.brush.btype);
    next_brush_mode_state.set(settings_config.brush.mode);
}

fn condition_setup_brush_particle_ready(
    particle_types: Query<Entity, Added<ParticleType>>,
    brush_without_particle: Query<(), (With<Brush>, Without<SelectedParticle>)>,
) -> bool {
    !particle_types.is_empty() && !brush_without_particle.is_empty()
}
