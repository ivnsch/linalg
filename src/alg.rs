#![cfg(test)]

use nalgebra::{Matrix3, Vector3};
use ndarray::{array, Array1, Array2};
use ndarray_linalg::Solve;

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
fn solve_equations_system() {
    let a: Array2<f64> = array![[3., 2., -1.], [2., -2., 4.], [-2., 1., -2.]];
    let b: Array1<f64> = array![1., -2., 0.];
    let x = a.solve_into(b).unwrap();
    assert!(x.abs_diff_eq(&array![1., -2., -2.], 1e-9));

    // // Define your 4x3 matrix A and 4-dimensional vector b
    // #[rustfmt::skip] 
    // let a = Array2::from_shape_vec((4, 3), vec![
    //     1.0, 2.0, 3.0,
    //     4.0, 5.0, 6.0,
    //     7.0, 8.0, 9.0,
    //     10.0, 11.0, 12.0,
    // ]).unwrap();

    // let b = Array1::from(vec![10.0, 20.0, 30.0, 40.0]);

    // // Solve the equation Ax = b
    // // a.solv
    // match a.solve_into(b) {
    //     Ok(x) => println!("Solution x: {:?}", x),
    //     Err(e) => println!("Error: {}", e),
    // }
}
    
// }
