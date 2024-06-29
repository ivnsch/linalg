use bevy::prelude::*;

#[allow(dead_code)]
pub fn add_curves_3d_system(app: &mut App) {
    // app.add_systems(Update, draw_square_fn);
    app.add_systems(Update, draw_sin_as_vert_vecs);
}

#[allow(dead_code)]
fn draw_square_fn(gizmos: Gizmos) {
    draw_fn(gizmos, -10, 10, |x| x * x);
}

#[allow(dead_code)]
fn draw_sin_fn(gizmos: Gizmos, _time: Res<Time>) {
    const VERT_SCALING: f32 = 5.0;
    draw_fn(gizmos, -10, 10, |x| x.sin() * VERT_SCALING);
    // animate
    // let t = time.elapsed_seconds();
    // draw_fn(gizmos, -10 + t as i32, 10 + t as i32, |x| x.sin());
}

fn draw_sin_as_vert_vecs(gizmos: Gizmos, _time: Res<Time>) {
    const VERT_SCALING: f32 = 5.0;
    draw_fn_as_vert_vecs(gizmos, -10, 10, |x| x.sin() * VERT_SCALING);
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

fn draw_fn_as_vert_vecs(
    mut gizmos: Gizmos,
    range_start: i32,
    range_end: i32,
    function: fn(f32) -> f32,
) {
    let x_scaling = 0.1;
    let z_scaling = 0.2;

    let mut last_point = None;

    for i in range_start..range_end {
        let x = i as f32;
        let z = function(x);

        if let Some((last_x, last_z)) = last_point {
            vert_x_arrow_out(last_x * x_scaling, last_z * z_scaling, &mut gizmos);
            vert_x_arrow_out(x * x_scaling, z * z_scaling, &mut gizmos);
        }

        last_point = Some((x, z));
    }
}

fn vert_x_arrow_out(x: f32, y: f32, gizmos: &mut Gizmos) {
    gizmos.arrow(Vec3::new(x, 0.0, 0.0), Vec3::new(x, y, 0.0), Color::WHITE);
}
