use bevy::{math::Vec3, prelude::Gizmos, render::color::Color};

/// draws function as a line
pub fn draw_line_fn<F>(
    mut gizmos: Gizmos,
    range_start: i32,
    range_end: i32,
    step_size: usize,
    function: F,
) where
    F: Fn(f32) -> f32,
{
    let scaling = 0.2;
    let x_scaling = scaling;
    let z_scaling = scaling;

    let mut last_point = None;

    for x_int in (range_start..range_end).step_by(step_size) {
        let x = x_int as f32;
        let z = function(x);
        let y = 0.0;

        if let Some((last_x, last_z)) = last_point {
            gizmos.line(
                Vec3::new(last_x * x_scaling, last_z * z_scaling, y),
                Vec3::new(x * x_scaling, z * z_scaling, y),
                Color::WHITE,
            );
        }

        last_point = Some((x, z));
    }
}