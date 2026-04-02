use bevy::prelude::*;
use bevy_falling_sand::{
    core::{ParticleSimulationRun, SimulationStepSignal},
    debug::{DebugDirtyRects, DebugParticleMap},
};
use leafwing_input_manager::common_conditions::action_just_pressed;

use crate::{
    brush::SelectedParticle,
    particles::HoveredParticle,
    ui::{QuickAction, ShowUi, UiState},
};

pub(super) struct SystemsPlugin;

impl Plugin for SystemsPlugin {
    fn build(&self, app: &mut App) {
        app.configure_sets(
            Update,
            CanvasQuickActionSystems.run_if(in_state(UiState::Canvas)),
        );
        app.add_systems(
            Update,
            (
                handle_toggle_ui.run_if(action_just_pressed(QuickAction::ToggleUi)),
                handle_toggle_map.run_if(action_just_pressed(QuickAction::ToggleMapOverlay)),
                handle_toggle_dirty_chunks
                    .run_if(action_just_pressed(QuickAction::ToggleDirtyChunksOverlay)),
                handle_toggle_simulation_run
                    .run_if(action_just_pressed(QuickAction::ToggleSimulationRun)),
                handle_toggle_simulation_step
                    .run_if(action_just_pressed(QuickAction::ToggleSimulationStep)),
                sample_hovered_particle
                    .run_if(action_just_pressed(QuickAction::SampleHoveredParticle)),
            )
                .in_set(CanvasQuickActionSystems),
        );
    }
}

/// System set for application initialization systems.
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct CanvasQuickActionSystems;

fn toggle_resource<T: Resource + Default>(commands: &mut Commands, resource: &Option<Res<T>>) {
    if resource.is_some() {
        commands.remove_resource::<T>();
    } else {
        commands.init_resource::<T>();
    }
}

fn handle_toggle_ui(mut commands: Commands, show_ui: Option<Res<ShowUi>>) {
    toggle_resource(&mut commands, &show_ui);
}

fn handle_toggle_map(mut commands: Commands, debug_map: Option<Res<DebugParticleMap>>) {
    toggle_resource(&mut commands, &debug_map);
}

fn handle_toggle_dirty_chunks(mut commands: Commands, debug_chunks: Option<Res<DebugDirtyRects>>) {
    toggle_resource(&mut commands, &debug_chunks);
}

fn handle_toggle_simulation_run(
    mut commands: Commands,
    simulation_run: Option<Res<ParticleSimulationRun>>,
) {
    toggle_resource(&mut commands, &simulation_run);
}

fn handle_toggle_simulation_step(mut msgw_simulation_step: MessageWriter<SimulationStepSignal>) {
    msgw_simulation_step.write(SimulationStepSignal);
}

fn sample_hovered_particle(
    hovered_particle: Res<HoveredParticle>,
    mut selected_particle: Single<&mut SelectedParticle>,
) {
    if let Some(particle) = hovered_particle.particle.clone() {
        selected_particle.0 = particle;
    }
}
