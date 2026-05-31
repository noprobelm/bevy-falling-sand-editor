use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use crate::ui::PopupState;

pub(super) struct StatesPlugin;

impl Plugin for StatesPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<PopupState<ToolOptionsWindowState>>()
            .add_systems(
                Update,
                toggle_tool_options.run_if(input_just_pressed(KeyCode::KeyT)),
            );
    }
}

#[derive(Reflect, Default, Debug, Clone, Eq, PartialEq, Hash)]
pub enum ToolOptionsWindowState {
    #[default]
    Closed,
    Open,
}

fn toggle_tool_options(
    current: Res<State<PopupState<ToolOptionsWindowState>>>,
    mut next: ResMut<NextState<PopupState<ToolOptionsWindowState>>>,
) {
    next.set(current.get().get_next());
}
