use crate::functions::draw_line2d_fn;
use bevy::prelude::*;
use nalgebra::{ArrayStorage, Const, Matrix2, Vector2};

#[allow(dead_code)]
pub fn add_lines_2d_system(app: &mut App) {
    app.add_systems(Update, draw_lines);
}

fn draw_lines(mut gizmos: Gizmos) {
    let half_range = 800;

    let line1 = |x| (-1.0 - x) / -2.0;
    let line2 = |x| 3.0 - (x * 2.0);

    let scaling = 100.0;
    draw_line2d_fn(&mut gizmos, -half_range, half_range, 10, scaling, line1);
    draw_line2d_fn(&mut gizmos, -half_range, half_range, 10, scaling, line2);

    let matrix: MatrixWithResults = to_matrix(line1, line2);
    let intersection = intersection(&matrix);
    // println!("matrix: {:?} intersection: {:?}", matrix, intersection);

    draw_intersection(&mut gizmos, intersection, scaling);

    // just for convenience, draw column space on same plot
    // note that column space looks different depending on coefficient multipliers and row ordering,
    // and that we derive these from closures,
    // so entering here equations from somewhere else can render different column space vectors
    draw_column_space(&mut gizmos, &matrix, scaling);
}

fn draw_intersection(gizmos: &mut Gizmos, intersection: Intersection, scaling: f32) {
    gizmos.circle_2d(
        Vec2 {
            x: intersection.x * scaling,
            y: intersection.y * scaling,
        },
        10.0,
        Color::WHITE,
    );
}

fn intersection(matrix: &MatrixWithResults) -> Intersection {
    // TODO no unwrap
    let solution = matrix.m.lu().solve(&matrix.res).unwrap();
    Intersection {
        x: solution.x,
        y: solution.y,
    }
}

fn to_matrix<F1, F2>(line1: F1, line2: F2) -> MatrixWithResults
where
    F1: Fn(f32) -> f32,
    F2: Fn(f32) -> f32,
{
    let entry1 = to_matrix_entry(line1);
    let entry2 = to_matrix_entry(line2);

    MatrixWithResults {
        m: Matrix2::new(
            entry1.x, entry1.y, //
            entry2.x, entry2.y, //
        ),
        res: Vector2::new(entry1.res, entry2.res),
    }
}

#[derive(Debug)]
struct MatrixEntry {
    x: f32,
    y: f32,
    res: f32,
}

#[derive(Debug)]
struct MatrixWithResults {
    m: Matrix2<f32>,
    res: Vector2<f32>,
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
    // println!("coefficients: {:?}", coefficients);
    MatrixEntry {
        x: -coefficients.m,
        y: 1.0,
        res: coefficients.b,
    }
}

fn draw_column_space(gizmos: &mut Gizmos, matrix: &MatrixWithResults, scaling: f32) {
    let col1 = matrix.m.column(0);
    let col2 = matrix.m.column(1);

    let v1 = Vec2 {
        x: col1[0] * scaling,
        y: col1[1] * scaling,
    };
    let v2 = Vec2 {
        x: col2[0] * scaling,
        y: col2[1] * scaling,
    };
    let v_sum = Vec2 {
        x: matrix.res[0] * scaling,
        y: matrix.res[1] * scaling,
    };

    let origin = Vec2 { x: 0.0, y: 0.0 };
    gizmos.arrow_2d(origin, v1, Color::BLUE);
    gizmos.arrow_2d(origin, v2, Color::BLUE);
    gizmos.arrow_2d(origin, v_sum, Color::YELLOW);

    // finish parallelogram (visual guide)
    gizmos.arrow_2d(v1, v_sum, Color::BLACK);
    gizmos.arrow_2d(v2, v_sum, Color::BLACK);
}

fn to_line_coefficients<F>(line_closure: F) -> Line
where
    F: Fn(f32) -> f32,
{
    let x1 = 1.0;
    let y1 = line_closure(x1);
    let x2 = 2.0;
    let y2 = line_closure(x2);

    let a: nalgebra::Matrix<f32, Const<2>, Const<2>, ArrayStorage<f32, 2, 2>> = Matrix2::new(
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
