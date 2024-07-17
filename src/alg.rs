#![cfg(test)]
use approx::assert_relative_eq;
// use ndarray::{array, Array1, Array2};
// use ndarray_linalg::Solve;
use faer::{assert_matrix_eq, linalg::matmul::matmul, mat, Mat, Parallelism};
use nalgebra::{Matrix3, Matrix3x1, Vector3};
use faer::linalg::triangular_solve::solve_lower_triangular_in_place;
use peroxide::fuga::{matrix, LinearAlgebra, Shape::Col, SolveKind};

#[test]
fn multiply_vector_matrix() {
    let v = Vector3::new(1, 2, 3);

    #[rustfmt::skip] 
    let m = Matrix3::new(
        11, 12, 13, 
        21, 22, 23,
        31, 32, 33
    );

    let res = m * v;

    assert_eq!(res, Vector3::new(74, 134, 194));
}

#[test]
fn multiply_vector_matrix_faer() {
    let matrix = mat![
        [11.0, 12.0, 13.0], 
        [21.0, 22.0, 23.0],
        [31.0, 32.0, 33.0]
    ];

    let v = mat![
        [1.0], 
        [2.0],
        [3.0]
    ];
    let mut c = Mat::<f64>::zeros(3, 1);

    matmul(c.as_mut(), matrix.as_ref(), v.as_ref(), None, 1.0, Parallelism::None);
    // println!("{:?}", c);

    assert_matrix_eq!(
        c, 
        mat![[74.0], [134.0], [194.0]],
        comp = abs,
        tol = 1e-10
    );
}


#[test]
fn solve_no_solutions_equations_system() {
    #[rustfmt::skip] 
    let a = Matrix3::new(
        0.0, 1.0, 4.0,
        1.0, 3.0, 5.0,
        3.0, 7.0, 7.0,
    );

    let b = Vector3::new(-5.0, -2.0, 6.0);

    // wrong result with lu decomposition
    // when there are no solutions, sometimes qr decomposition returns wrong results too
    // let solution = a.lu().solve(&b); // Some([[5254199565265581.0, -3002399751580329.0, 750599937895081.0]])
    let solution = a.qr().solve(&b);
    println!("solution: {:?}", solution);

    assert_eq!(solution, None);
}

#[test]
fn solve_no_solutions_equations_system_faer() {
    let matrix = mat![
        [0.0, 1.0, 4.0], 
        [1.0, 3.0, 5.0],
        [3.0, 7.0, 7.0]
    ];

    let v = mat![
        [-5.0], 
        [-2.0],
        [6.0]
    ];
    
    let mut x = Mat::<f64>::zeros(4, 2);
    x.copy_from(&v);
    solve_lower_triangular_in_place(matrix.as_ref(), x.as_mut(), Parallelism::None);
    
// no solutions - returns a matrix with inf and NaN values
// [
// [-inf],
// [inf],
// [NaN],
// ]
    println!("{:?}", x);
}

#[test]
fn solve_single_solution_equations_system() {
    #[rustfmt::skip] 
    let a = Matrix3::new(
        1.0, 0.0, -3.0,
        2.0, 2.0, 9.0,
        0.0, 1.0, 5.0,
    );

    let b = Vector3::new(8.0, 7.0, -2.0);

    let expected_solution = Matrix3x1::new(5.0, 3.0, -1.0);

    let lu_solution = a.lu().solve(&b); 
    assert_eq!(lu_solution.unwrap(), expected_solution);
    let qr_solution = a.qr().solve(&b); 
    assert_eq!(qr_solution.unwrap(), expected_solution);
    let least_squares_solution = a.svd(true, true).solve(&b, 0.0); 
    // not sure about background of specific epsilon here, leaving smallest that passes (> f64::EPSILON)
    assert_relative_eq!(least_squares_solution.unwrap(), expected_solution, epsilon = 0.00000000000001);
}

#[test]
fn echelon_form() {
        let a = matrix(
            vec![
                1.0, 2.0, -1.0, //
                1.0, -1.0, -2.0, //
                1.0, 2.0, 3.0, //
                2.0, 7.0, 7.0,
            ],
            3,
            4,
            Col,
        );

        // Calculate reduced row echelon form
        let rref_a = a.rref();
        // println!("rref_a: {}", rref_a);

        // the solution: x1 = 1, x2 = -1, x3 = 2
        let expected= matrix(
            vec![
                1.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, //
                0.0, 0.0, 1.0, //
                1.0, -1.0, 2.0,
            ],
            3,
            4,
            Col,
        );

        assert_eq!(rref_a, expected);
}

#[test]
fn echelon_form_with_free_variables() {
        let a = matrix(
            vec![
                3.0, 8.0, 0.0, //
                0.0, 0.0, 2.0, //
                1.0, 0.0, 2.0, //
                0.0, 2.0, 1.0, //
                0.0, 0.0, 0.0
            ],
            3,
            5,
            Col,
        );

        // Calculate reduced row echelon form
        let rref_a = a.rref();
        println!("rref_a: {}", rref_a);

        // // the solution: x1 = 1, x2 = -1, x3 = 2
        let expected= matrix(
            vec![
                1.0, 0.0, 0.0, //
                0.0, 1.0, 0.0, //
                0.0, 0.0, 1.0, //
                0.25, 1.25, -0.75, //
                0.0, 0.0, 0.0, //
            ],
            3,
            5,
            Col,
        );

        assert_eq!(rref_a, expected);
}

#[test]
fn lu_decomposition() {
        let a = matrix(
            vec![
                2.0, 1.0, 3.0, //
                1.0, -1.0, 2.0, //
                3.0, 2.0, 1.0, //
            ],
            3,
            3,
            Col,
        );

        let e = 0.0001;

        let b  = &vec![5.0, 4.0, 7.0];
        let res = a.solve(b, SolveKind::LU);
        println!("res: {:?}", res);
        assert_relative_eq!(res[0], 3.0, epsilon = e);
        assert_relative_eq!(res[1], -1.0, epsilon = e);
        assert_relative_eq!(res[2], 0.0, epsilon = e);

        let lu= a.lu();
        // println!("lu: {:?}", lu);

        // l: lower triangular matrix
        let l = lu.l;
        // l:        c[0]   c[1]   c[2]
        // r[0]      1      0      0
        // r[1] 0.6667      1      0
        // r[2] 0.3333 0.7143      1
        println!("l: {}", l);
        // row 1
        assert_relative_eq!(l[(0,0)], 1.0, epsilon = e);
        assert_relative_eq!(l[(0,1)], 0.0, epsilon = e);
        assert_relative_eq!(l[(0,2)], 0.0, epsilon = e);

        // row 2
        assert_relative_eq!(l[(1,0)], 0.666666, epsilon = e);
        assert_relative_eq!(l[(1,1)], 1.0, epsilon = e);
        assert_relative_eq!(l[(1,2)], 0.0, epsilon = e);
        
        // row 3
        assert_relative_eq!(l[(2,0)], 0.333333, epsilon = e);
        assert_relative_eq!(l[(2,1)], 0.714285, epsilon = e);
        assert_relative_eq!(l[(2,2)], 1.0, epsilon = e);

        // u: echelon form of A
        let u = lu.u;
        // u:          c[0]    c[1]    c[2]
        // r[0]       3       1       2
        // r[1]       0  2.3333 -0.3333
        // r[2]       0       0 -1.4286
        println!("u: {}", u);
        // row 1
        assert_relative_eq!(u[(0,0)], 3.0, epsilon = e);
        assert_relative_eq!(u[(0,1)], 1.0, epsilon = e);
        assert_relative_eq!(u[(0,2)], 2.0, epsilon = e);

        // row 2
        assert_relative_eq!(u[(1,0)], 0.0, epsilon = e);
        assert_relative_eq!(u[(1,1)], 2.333333, epsilon = e);
        assert_relative_eq!(u[(1,2)], -0.333333, epsilon = e);
        
        // row 3
        assert_relative_eq!(u[(2,0)], 0.0, epsilon = e);
        assert_relative_eq!(u[(2,1)], 0.0, epsilon = e);
        assert_relative_eq!(u[(2,2)], -1.428571, epsilon = e);

        // check whether solving LU "by hand" renders the same result as solving the original matrix
        // incidentally using LU internally to solve these as well
        let y = l.solve(b, SolveKind::LU);
        let x = u.solve(&y, SolveKind::LU);

        // this gives [4.0, -0.2, -3.4], 
        // which is different to solution [3.0, -1.0, 0.0]
        // what's wrong? TODO
        println!("x = {:?}", x);

        assert_relative_eq!(res[0], x[0], epsilon = e);
        assert_relative_eq!(res[1], x[1], epsilon = e);
        assert_relative_eq!(res[2], x[2], epsilon = e);

}


// #[test]
// fn solve_equations_system2() {
//     let a: Array2<f64> = array![[3., 2., -1.], [2., -2., 4.], [-2., 1., -2.]];
//     let b: Array1<f64> = array![1., -2., 0.];
//     let x = a.solve_into(b).unwrap();
//     assert!(x.abs_diff_eq(&array![1., -2., -2.], 1e-9));

//     // // Define your 4x3 matrix A and 4-dimensional vector b
//     // #[rustfmt::skip] 
//     // let a = Array2::from_shape_vec((4, 3), vec![
//     //     1.0, 2.0, 3.0,
//     //     4.0, 5.0, 6.0,
//     //     7.0, 8.0, 9.0,
//     //     10.0, 11.0, 12.0,
//     // ]).unwrap();

//     // let b = Array1::from(vec![10.0, 20.0, 30.0, 40.0]);

//     // // Solve the equation Ax = b
//     // // a.solv
//     // match a.solve_into(b) {
//     //     Ok(x) => println!("Solution x: {:?}", x),
//     //     Err(e) => println!("Error: {}", e),
//     // }
// }
    
// }
