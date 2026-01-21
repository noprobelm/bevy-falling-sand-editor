mod console;
mod states;

use bevy::prelude::*;

use bevy_egui::{EguiContexts, EguiPlugin, EguiPrimaryContextPass, egui};
pub use console::*;
pub use states::*;

pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((EguiPlugin::default(), StatesPlugin, ConsolePlugin))
            .add_systems(EguiPrimaryContextPass, render);
    }
}

fn render(mut contexts: EguiContexts) -> Result {
    let ctx = contexts.ctx_mut()?;
    egui::TopBottomPanel::top("Top panel").show(ctx, |ui| {});
    Ok(())
}
