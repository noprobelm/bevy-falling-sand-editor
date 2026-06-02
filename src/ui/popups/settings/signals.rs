use bevy::prelude::*;
use crate::ui::SettingsApplicationState;

pub(super) struct SignalsPlugin;

impl Plugin for SignalsPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_toggle_settings);
    }
}

#[derive(Event)]
pub struct UiToggleSettingsEvent;

fn on_toggle_settings(
    _trigger: On<UiToggleSettingsEvent>,
    current_settings_app_state: Res<State<SettingsApplicationState>>,
    mut next_settings_app_state: ResMut<NextState<SettingsApplicationState>>,
) {
    let next = match current_settings_app_state.get() {
        SettingsApplicationState::Open => SettingsApplicationState::Closed,
        SettingsApplicationState::Closed => SettingsApplicationState::Open,
    };
    next_settings_app_state.set(next);
}
