use bevy::prelude::*;

/// Find all horizontal lines interpolated between a start and end position.
pub fn get_interpolated_line_points(start: Vec2, end: Vec2, min_x: i32, max_x: i32) -> Vec<IVec2> {
    let mut positions = vec![];

    let direction = (end - start).normalize();
    let length = (end - start).length();
    let num_samples = (length.ceil() as usize).max(1);

    for i in 0..=num_samples {
        let t = i as f32 / num_samples as f32;
        let sample_point = start + direction * length * t;

        for x_offset in min_x..=max_x {
            let position = IVec2::new(
                (sample_point.x + x_offset as f32).floor() as i32,
                sample_point.y.floor() as i32,
            );
            positions.push(position);
        }
    }

    positions
}

pub fn get_interpolated_cursor_points(start: Vec2, end: Vec2) -> Vec<IVec2> {
    if start == end {
        return vec![start.as_ivec2()];
    }

    let mut positions = vec![];
    let direction = (end - start).normalize();
    let length = (end - start).length();
    let num_samples = (length.ceil() as usize).max(1);

    for i in 0..=num_samples {
        let t = i as f32 / num_samples as f32;
        positions.push((start + direction * length * t).as_ivec2());
    }
    positions
}
