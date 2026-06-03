use crate::ui::{ParticleEditorWindowState, PopupState};
use bevy::prelude::*;

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_toggle_particle_editor);
    }
}

#[derive(Event)]
pub struct UiToggleParticleEditorEvent;

fn on_toggle_particle_editor(
    _trigger: On<UiToggleParticleEditorEvent>,
    current_partical_editor_state: Res<State<PopupState<ParticleEditorWindowState>>>,
    mut next_partical_editor_state: ResMut<NextState<PopupState<ParticleEditorWindowState>>>,
) {
    next_partical_editor_state.set(current_partical_editor_state.get_next());
}
