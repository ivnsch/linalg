use bevy::prelude::*;
use nalgebra::{Matrix2, Vector2};

use crate::functions::draw_line_fn;

#[allow(dead_code)]
pub fn add_lines_2d_system(app: &mut App) {
    app.add_systems(Update, draw_lines);
}

fn draw_lines(mut gizmos: Gizmos) {
    let half_range = 800;

    let line1 = |x| (x + 200.0) * 2.0;
    let line2 = |x| x * -1.0 + 1.5;

    draw_line_fn(&mut gizmos, -half_range, half_range, 10, 1.0, line1);
    draw_line_fn(&mut gizmos, -half_range, half_range, 10, 1.0, line2);

    let intersection = intersection(line1, line2);

    gizmos.circle_2d(
        Vec2 {
            x: intersection.x,
            y: intersection.y,
        },
        10.0,
        Color::WHITE,
    );
}

fn intersection<F1, F2>(line1: F1, line2: F2) -> Intersection
where
    F1: Fn(f32) -> f32,
    F2: Fn(f32) -> f32,
{
    let entry1 = to_matrix_entry(line1);
    let entry2 = to_matrix_entry(line2);

    let a: nalgebra::Matrix<
        f32,
        nalgebra::Const<2>,
        nalgebra::Const<2>,
        nalgebra::ArrayStorage<f32, 2, 2>,
    > = Matrix2::new(
        entry1.x, entry1.y, //
        entry2.x, entry2.y, //
    );

    let b = Vector2::new(entry1.res, entry2.res);

    // TODO no unwrap
    let solution = a.lu().solve(&b).unwrap();
    // println!("a: {:?}, b: {:?}, solution: {:?}", a, b, solution);

    Intersection {
        x: solution.x,
        y: solution.y,
    }
}

#[derive(Debug)]
struct MatrixEntry {
    x: f32,
    y: f32,
    res: f32,
}

#[derive(Debug)]
struct Line {
    m: f32,
    b: f32,
}

#[derive(Debug)]
struct Intersection {
    x: f32,
    y: f32,
}

fn to_matrix_entry<F>(line_closure: F) -> MatrixEntry
where
    F: Fn(f32) -> f32,
{
    let coefficients = to_line_coefficients(line_closure);
    MatrixEntry {
        x: -coefficients.m,
        y: 1.0,
        res: coefficients.b,
    }
}

fn to_line_coefficients<F>(line_closure: F) -> Line
where
    F: Fn(f32) -> f32,
{
    let x1 = 1.0;
    let y1 = line_closure(x1);
    let x2 = 2.0;
    let y2 = line_closure(x2);

    let a: nalgebra::Matrix<
        f32,
        nalgebra::Const<2>,
        nalgebra::Const<2>,
        nalgebra::ArrayStorage<f32, 2, 2>,
    > = Matrix2::new(
        x1, 1.0, //
        x2, 1.0, //
    );

    let b = Vector2::new(y1, y2);

    // TODO no unwrap
    let solution = a.lu().solve(&b).unwrap();

    Line {
        m: solution.x,
        b: solution.y,
    }
}
