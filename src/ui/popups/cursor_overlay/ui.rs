use crate::{
    Cursor,
    particles::HoveredParticle,
    ui::{ParticleCategoryLabels, ShowCursorOverlay, ShowUi},
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
        .init_resource::<ShowCursorOverlay>();
    }
}

pub fn show(
    mut contexts: EguiContexts,
    hovered_particle: Res<HoveredParticle>,
    labels: Res<ParticleCategoryLabels>,
    cursor_position: Res<Cursor>,
) -> Result {
    let ctx = contexts.ctx_mut()?;

    egui::Window::new("brush overlay")
        .title_bar(false)
        .resizable(false)
        .constrain_to(ctx.content_rect())
        .anchor(egui::Align2::RIGHT_TOP, [-10.0, 10.0])
        .show(ctx, |ui| {
            ui.label(
                egui::RichText::new(format!("x: {:8.3}", cursor_position.current.x)).monospace(),
            );
            ui.label(
                egui::RichText::new(format!("y: {:8.3}", cursor_position.current.y)).monospace(),
            );
            let particle = hovered_particle.particle.map_or_else(String::new, |id| {
                labels
                    .categories()
                    .flat_map(|(_, labels)| labels.iter())
                    .find_map(|label| (label.id == id).then(|| label.name.clone()))
                    .unwrap_or_else(|| format!("Particle {}", id.get()))
            });
            ui.label(particle);
        });

    Ok(())
}
