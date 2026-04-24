use bevy::prelude::*;
use bevy_falling_sand::prelude::ParticleTypesPersistedSignal;
use std::path::PathBuf;

use crate::ui::{ParticleTypesSavedMessageConfiguration, PopupState};

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PopupState<ParticleEditorWindowState>>()
            .init_state::<PopupState<LoadParticlesWindowState>>()
            .init_state::<SynchronizeBrushState>()
            .add_systems(
                Update,
                (
                    handle_particle_types_recently_saved,
                    tick_particle_types_recently_saved_timer
                        .run_if(resource_exists::<ParticleTypesRecentlySaved>),
                ),
            );
    }
}

#[derive(Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ParticleEditorWindowState {
    #[default]
    Closed,
    Open,
}

#[derive(Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum LoadParticlesWindowState {
    #[default]
    Closed,
    Open,
}

#[derive(States, Reflect, Default, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum SynchronizeBrushState {
    #[default]
    Enabled,
    Disabled,
}

#[derive(Resource, Reflect, Default, Debug, Clone, Eq, PartialEq)]
pub struct ParticleTypesRecentlySaved {
    pub path: PathBuf,
    pub timer: Timer,
}

fn handle_particle_types_recently_saved(
    mut commands: Commands,
    mut msgr_particle_types_saved: MessageReader<ParticleTypesPersistedSignal>,
    config: Res<ParticleTypesSavedMessageConfiguration>,
) {
    msgr_particle_types_saved.read().for_each(|msg| {
        commands.insert_resource(ParticleTypesRecentlySaved {
            path: msg.0.clone(),
            timer: Timer::new(config.fade_duration, TimerMode::Once),
        })
    });
}

fn tick_particle_types_recently_saved_timer(
    mut commands: Commands,
    mut particle_types_recently_saved: ResMut<ParticleTypesRecentlySaved>,
    time: Res<Time>,
) {
    particle_types_recently_saved.timer.tick(time.delta());
    if particle_types_recently_saved.timer.just_finished() {
        commands.remove_resource::<ParticleTypesRecentlySaved>();
    }
}
