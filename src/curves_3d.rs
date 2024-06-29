use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_curves_3d_system(app: &mut App) {
    // app.add_systems(Update, draw_square_fn);
    app.add_systems(Update, draw_sin_fn);
}

#[allow(dead_code)]
fn draw_square_fn(gizmos: Gizmos) {
    draw_fn(gizmos, -10, 10, |x| x * x);
}

fn draw_sin_fn(gizmos: Gizmos, _time: Res<Time>) {
    draw_fn(gizmos, -10, 10, |x| x.sin());
    // animate
    // let t = time.elapsed_seconds();
    // draw_fn(gizmos, -10 + t as i32, 10 + t as i32, |x| x.sin());
}

fn draw_fn(mut gizmos: Gizmos, range_start: i32, range_end: i32, function: fn(f32) -> f32) {
    let scaling = 0.2;
    let x_scaling = scaling;
    let y_scaling = scaling;

    let mut last_point = None;

    for i in range_start..range_end {
        let x = i as f32;
        let z = 0.0;
        let y = function(x);

        if let Some((last_x, last_y)) = last_point {
            gizmos.line(
                Vec3::new(last_x * x_scaling, z, last_y * y_scaling),
                Vec3::new(x * x_scaling, z, y * y_scaling),
                Color::WHITE,
            );
        }

        last_point = Some((x, y));
    }
}
