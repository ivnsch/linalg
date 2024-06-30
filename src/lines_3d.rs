use bevy::prelude::*;

use crate::functions::draw_line_fn;

#[allow(dead_code)]
pub fn add_lines_3d_system(app: &mut App) {
    app.add_systems(Update, draw_lines);
}

fn draw_lines(gizmos: Gizmos) {
    let half_range = 20;

    draw_line_fn(gizmos, -half_range, half_range, 10, |x| x * 2.0);
}
