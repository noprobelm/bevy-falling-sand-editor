use crate::{
    Cursor,
    particles::HoveredParticle,
    ui::{ShowCursorOverlay, ShowUi, UiToggleCursorOverlaySignal},
};
use bevy::prelude::*;
use bevy_egui::{EguiContexts, EguiPrimaryContextPass, egui};
pub(super) struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            EguiPrimaryContextPass,
            show.run_if(resource_exists::<ShowUi>)
                .run_if(resource_exists::<ShowCursorOverlay>),
        )
        .init_resource::<ShowCursorOverlay>()
        .add_observer(on_toggle_cursor_overlay);
    }
}

fn on_toggle_cursor_overlay(
    _trigger: On<UiToggleCursorOverlaySignal>,
    mut commands: Commands,
    enabled: Option<Res<ShowCursorOverlay>>,
) {
    if enabled.is_some() {
        commands.remove_resource::<ShowCursorOverlay>();
    } else {
        commands.insert_resource(ShowCursorOverlay);
    }
}

pub fn show(
    mut contexts: EguiContexts,
    hovered_particle: Res<HoveredParticle>,
    cursor_position: Res<Cursor>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("brush overlay")
        .title_bar(false)
        .resizable(false)
        .constrain_to(ctx.available_rect())
        .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(format!("x: {:8.3}", cursor_position.current.x)).monospace(),
            );
            ui.label(
                egui::RichText::new(format!("y: {:8.3}", cursor_position.current.y)).monospace(),
            );
            if let Some(particle) = hovered_particle.particle.clone() {
                ui.label(particle.name);
            }
        });

    Ok(())
}
