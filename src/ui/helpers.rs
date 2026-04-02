use std::ops::RangeInclusive;

use bevy_egui::egui::{self, emath::Numeric};

pub fn add_label_with_drag_value<Num>(
    ui: &mut egui::Ui,
    fill: usize,
    label: impl Into<egui::WidgetText>,
    value: Num,
    range: RangeInclusive<Num>,
    speed: f64,
) -> Num
where
    Num: Numeric,
{
    ui.label(label);
    for _ in 0..fill {
        skip_grid_column(ui);
    }
    let mut drag_value = value;
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.add(
            egui::DragValue::new(&mut drag_value)
                .range(range)
                .speed(speed),
        );
    });
    ui.end_row();
    drag_value
}

pub fn add_label_with_toggle_switch(
    ui: &mut egui::Ui,
    fill: usize,
    label: impl Into<egui::WidgetText>,
    mut is_on: bool,
) -> bool {
    ui.label(label);
    for _ in 0..fill {
        skip_grid_column(ui);
    }
    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
        ui.add(crate::ui::widgets::toggle_switch::toggle(&mut is_on));
    });
    ui.end_row();
    is_on
}

pub fn skip_grid_column(ui: &mut egui::Ui) {
    ui.label("");
}

pub fn add_major_grid_separator(ui: &mut egui::Ui) {
    let row_spacing = ui.spacing().item_spacing.y;
    let padding = 8.0;
    let rect = ui.max_rect();
    // Line is drawn at: current position + padding above
    // Total space needed: padding above + padding below - row_spacing (which end_row adds)
    let y = ui.cursor().top() + padding;
    let mut stroke = ui.visuals().widgets.noninteractive.bg_stroke;
    stroke.width = 2.0;
    ui.painter().hline(rect.left()..=rect.right(), y, stroke);
    ui.allocate_space(egui::vec2(0.0, (padding * 2.0 - row_spacing).max(0.0)));
    ui.end_row();
}

pub fn add_minor_grid_separator(ui: &mut egui::Ui) {
    let row_spacing = ui.spacing().item_spacing.y;
    let rect = ui.max_rect();
    let y = ui.cursor().top() + row_spacing / 2.0;
    let stroke = ui.visuals().widgets.noninteractive.bg_stroke;
    ui.painter().hline(rect.left()..=rect.right(), y, stroke);
}
