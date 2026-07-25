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
    config::{InputButton, SettingsConfig},
    particles::DefaultParticleIds,
    setup::SetupSystems,
    tools::{
        brush::{ToolBrushColor, ToolBrushSize},
        painter::{
            PainterConfiguration, PainterShape, PainterSpawnState,
            components::{PainterBrush, SelectedParticle, SelectedParticleType},
            gizmos::PainterBrushGizmos,
            resources::PAINTER_BRUSH_DEFAULT_SIZE,
        },
    },
};

pub(super) struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<PainterAction>::default())
            .init_resource::<PainterConfiguration>()
            .insert_gizmo_config(
                PainterBrushGizmos,
                GizmoConfig {
                    enabled: true,
                    ..default()
                },
            )
            .add_systems(
                Startup,
                (spawn_brush, load_settings)
                    .chain()
                    .in_set(SetupSystems::Tools),
            )
            .add_systems(
                Update,
                insert_selected_particle.run_if(condition_setup_brush_particle_ready),
            );
    }
}

#[derive(Resource, Clone, Debug, Serialize, Deserialize)]
pub struct PainterKeyBindings {
    pub draw: InputButton,
    #[serde(alias = "toggle_brush_mode")]
    pub toggle_mode: InputButton,
}

impl Default for PainterKeyBindings {
    fn default() -> Self {
        Self {
            draw: MouseButton::Left.into(),
            toggle_mode: MouseButton::Right.into(),
        }
    }
}

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum PainterAction {
    ToggleMode,
    ToggleType,
    #[actionlike(Axis)]
    ChangeSize,
}

fn spawn_brush(mut commands: Commands, config: Res<PainterConfiguration>) {
    commands.spawn((
        PainterBrush,
        ToolBrushSize(PAINTER_BRUSH_DEFAULT_SIZE),
        ToolBrushColor(config.brush.color),
    ));
}

fn insert_selected_particle(
    mut commands: Commands,
    registry: Res<ParticleTypeRegistry>,
    default_ids: Res<DefaultParticleIds>,
    particle_types: Query<&ParticleType>,
    brush: Single<Entity, With<PainterBrush>>,
) {
    let (particle_type_id, pt_entity) = if let Some(entity) = registry.get(default_ids.dirt_wall) {
        (default_ids.dirt_wall, *entity)
    } else {
        let entity = *registry
            .entities()
            .next()
            .expect("No particle types found in the world");
        let particle_type = particle_types
            .get(entity)
            .expect("Failed to find particle type in query");
        (particle_type.id(), entity)
    };

    commands.entity(brush.entity()).insert((
        SelectedParticle(particle_type_id),
        SelectedParticleType(pt_entity),
    ));
}

fn load_settings(
    mut commands: Commands,
    mut next_brush_type_state: ResMut<NextState<PainterShape>>,
    mut next_brush_mode_state: ResMut<NextState<PainterSpawnState>>,
    brush: Single<Entity, With<PainterBrush>>,
    settings_config: Res<Persistent<SettingsConfig>>,
) {
    let keys = &settings_config.keys.painter;
    let mut input_map =
        InputMap::default().with_axis(PainterAction::ChangeSize, MouseScrollAxis::Y);
    keys.toggle_mode
        .insert_into_input_map(&mut input_map, PainterAction::ToggleMode);

    commands.entity(brush.entity()).insert((
        input_map,
        settings_config.painter.size,
        ToolBrushColor(settings_config.painter.configuration.brush.color),
    ));
    commands.insert_resource(settings_config.painter.configuration.clone());
    commands.insert_resource(settings_config.keys.painter.clone());
    next_brush_type_state.set(settings_config.painter.shape);
    next_brush_mode_state.set(settings_config.painter.mode);
}

fn condition_setup_brush_particle_ready(
    particle_types: Query<Entity, Added<ParticleType>>,
    brush_without_particle: Query<(), (With<PainterBrush>, Without<SelectedParticle>)>,
) -> bool {
    !particle_types.is_empty() && !brush_without_particle.is_empty()
}
