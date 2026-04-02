use bevy::prelude::*;
use bevy_falling_sand::core::SpawnParticleSignal;
use fontdue::{Font, FontSettings};

use crate::brush::SelectedParticle;

const DEFAULT_FONT: &[u8] = include_bytes!("../../../assets/fonts/JetBrainsMono-Regular.ttf");

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum TextAlignment {
    #[default]
    Left,
    Center,
    Right,
}

#[derive(Event, Clone, Debug)]
pub struct SpawnTextEvent {
    pub center: IVec2,
    pub text: String,
    pub font_size: f32,
    pub alignment: TextAlignment,
}

pub fn spawn_text(
    trigger: On<SpawnTextEvent>,
    mut msgw_spawn_particles: MessageWriter<SpawnParticleSignal>,
    selected_particle: Single<&SelectedParticle>,
) {
    let event = trigger.event();
    let positions = rasterize_text(&event.text, event.font_size, &event.alignment);

    let (min, max) = bounding_box(&positions);
    let text_center = (min + max) / 2;
    let offset = event.center - text_center;

    for pos in positions {
        msgw_spawn_particles.write(SpawnParticleSignal::new(
            selected_particle.0.clone(),
            pos + offset,
        ));
    }
}

fn rasterize_text(text: &str, font_size: f32, alignment: &TextAlignment) -> Vec<IVec2> {
    let font = Font::from_bytes(DEFAULT_FONT, FontSettings::default())
        .expect("Failed to load embedded font");

    let lines: Vec<&str> = text.lines().collect();

    // Rasterize each line and track its width
    let mut line_results: Vec<(Vec<IVec2>, i32)> = Vec::new();
    let line_height = font_size.ceil() as i32;
    let mut max_width: i32 = 0;

    for line in &lines {
        let (positions, width) = rasterize_line(&font, line, font_size);
        max_width = max_width.max(width);
        line_results.push((positions, width));
    }

    // Combine lines with vertical offset and horizontal alignment
    let mut all_positions = Vec::new();

    for (line_idx, (positions, width)) in line_results.into_iter().enumerate() {
        let x_offset = match alignment {
            TextAlignment::Left => 0,
            TextAlignment::Center => (max_width - width) / 2,
            TextAlignment::Right => max_width - width,
        };
        let y_offset = -(line_idx as i32) * line_height;

        for pos in positions {
            all_positions.push(IVec2::new(pos.x + x_offset, pos.y + y_offset));
        }
    }

    all_positions
}

fn rasterize_line(font: &Font, text: &str, font_size: f32) -> (Vec<IVec2>, i32) {
    let mut positions = Vec::new();
    let mut cursor_x: i32 = 0;

    for ch in text.chars() {
        let (metrics, bitmap) = font.rasterize(ch, font_size);

        for row in 0..metrics.height {
            for col in 0..metrics.width {
                let alpha = bitmap[row * metrics.width + col];
                if alpha > 128 {
                    let x = cursor_x + col as i32 + metrics.xmin;
                    let y = -(row as i32) + metrics.height as i32 - 1 + metrics.ymin;
                    positions.push(IVec2::new(x, y));
                }
            }
        }

        cursor_x += metrics.advance_width as i32;
    }

    (positions, cursor_x)
}

fn bounding_box(positions: &[IVec2]) -> (IVec2, IVec2) {
    if positions.is_empty() {
        return (IVec2::ZERO, IVec2::ZERO);
    }
    let mut min = positions[0];
    let mut max = positions[0];
    for &p in &positions[1..] {
        min = min.min(p);
        max = max.max(p);
    }
    (min, max)
}
