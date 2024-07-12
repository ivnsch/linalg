use std::f32::consts::PI;

use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_wave_2d_system(app: &mut App) {
    app.add_systems(Update, draw_wave);
}

fn draw_wave(mut gizmos: Gizmos, time: Res<Time>) {
    let range = 20;

    let t = time.elapsed_seconds() as f32;
    // let t = 0.0; // not animated

    // equation of travelling wave: u(x,t)=Acos(kx−ωt)
    // nice explanation https://physics.stackexchange.com/a/259007
    let function = |x: f32| {
        let amplitude = 1.0;
        let wave_length = 3.0;
        let k = 2.0 * PI / wave_length; // wave cycles per unit distance
        let frequency = 0.5;
        let angular_frequency = 2.0 * PI * frequency;
        let phase = 0.0;
        let scalar = ((k * x) - angular_frequency * t + phase).cos();

        amplitude * scalar
    };

    draw_planar_fn_as_vert_vecs(&mut gizmos, -range, range, Color::WHITE, function);
}

/// draws planar function as a sequence of vectors,
fn draw_planar_fn_as_vert_vecs<F>(
    gizmos: &mut Gizmos,
    range_start: i32,
    range_end: i32,
    color: Color,
    function: F,
) where
    F: Fn(f32) -> f32,
{
    let scaling = 50.0;
    let x_scaling = scaling;
    let y_scaling = scaling;

    let mut last_point = None;

    let mut value = range_start as f32;
    while value < range_end as f32 {
        let x = value as f32;
        let y = function(x);

        if let Some((last_x, last_y)) = last_point {
            vert_x_arrow_out(last_x * x_scaling, last_y * y_scaling, gizmos, color);
            vert_x_arrow_out(x * x_scaling, y * y_scaling, gizmos, color);
        }

        last_point = Some((x, y));
        value += 0.1;
    }
}

fn vert_x_arrow_out(x: f32, y: f32, gizmos: &mut Gizmos, color: Color) {
    gizmos.arrow_2d(Vec2::new(x, 0.0), Vec2::new(x, y), color);
}
