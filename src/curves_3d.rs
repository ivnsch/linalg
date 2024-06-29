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
    const VERT_SCALING: f32 = 5.0;
    draw_fn(gizmos, -10, 10, |x| x.sin() * VERT_SCALING);
    // animate
    // let t = time.elapsed_seconds();
    // draw_fn(gizmos, -10 + t as i32, 10 + t as i32, |x| x.sin());
}

fn draw_fn(mut gizmos: Gizmos, range_start: i32, range_end: i32, function: fn(f32) -> f32) {
    let scaling = 0.2;
    let x_scaling = scaling;
    let z_scaling = scaling;

    let mut last_point = None;

    for i in range_start..range_end {
        let x = i as f32;
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
