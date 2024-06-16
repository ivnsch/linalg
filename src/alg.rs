#[cfg(test)]
use nalgebra::{Matrix3, Vector3};

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
