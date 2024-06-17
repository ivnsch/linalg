#![cfg(test)]

use nalgebra::{Matrix1x3, Matrix3, Matrix3x1, Matrix4, Vector3, Vector4};
// use ndarray::{array, Array1, Array2};
// use ndarray_linalg::Solve;

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
