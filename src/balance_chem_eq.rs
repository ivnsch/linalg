use nalgebra::{DMatrix, Vector3};
use peroxide::fuga::{matrix, LinearAlgebra, Shape::Col};

pub fn balance(candidate: ReactionMoleculeUnits) -> Reaction {
    // let matrix = Matrix3::from_columns(&[
    //     candidate.input[0].0.clone().map(|e| e as f64),
    //     candidate.input[1].0.clone().map(|e| e as f64),
    //     candidate.output[0].0.clone().map(|e| e as f64),
    //     candidate.output[1].0.clone().map(|e| e as f64),
    // ]);

    // let a = DMatrix::from_row_slice(2, 3, &[
    //     1.0, 2.0, 3.0,
    //     4.0, 5.0, 6.0
    // ]);

    let v1: Vec<f64> = candidate.input[0]
        .0
        .into_iter()
        .map(|e| *e as f64)
        .collect();

    let v2: Vec<f64> = candidate.input[1]
        .0
        .into_iter()
        .map(|e| *e as f64)
        .collect();

    let v3: Vec<f64> = candidate.output[0]
        .0
        .into_iter()
        .map(|e| *e as f64 * -1.0)
        .collect();

    let v4: Vec<f64> = candidate.output[1]
        .0
        .into_iter()
        .map(|e| *e as f64 * -1.0)
        .collect();

    let result: Vec<f64> = vec![0.0, 0.0, 0.0];

    let all_cols = [v1, v2, v3, v4, result].concat().to_owned();

    let mut a = matrix(
        all_cols,
        // vec![
        //     // 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0,
        //     //
        //     // 2.0, -1.0, 3.0, 0.0, 0.0, 0.0, 1.0, 2.0,
        //     // -2.0,
        //     //
        //     1.0, 2.0, -1.0, //
        //     1.0, -1.0, -2.0, //
        //     1.0, 2.0, 3.0, //
        //     2.0, 7.0, 7.0,
        // ],
        3, 4, Col,
    );

    // Calculate the reduced row echelon form
    let rref_a = a.rref();
    println!("rref_a: {:?}", rref_a);

    // variable x4 (4th column) is free, that is, the molecule count of the last variable,
    // the count of the other molecules depends on what we choose for x4

    // remember that coefficients are the number of atoms of each element in the respective molecule:
    let c4 = rref_a.col(3);
    println!("c4 (water) coefficients (atom count) {:?}", c4);

    // so if the molecule is e.g. water (H20), we'd have a vector (0, 2, 1) (corresponding to C, H, O),
    // and a scalar multiplier (the "x") corresponding to how many of these molecules go into the balanced equation.
    // in our example equation, the water molecule (by virtue of it incidentally appearing last in the equation) is "free"
    // so we derive the count of the other molecules depending on the water we want to get

    // so let's set 4 water molecules and see how many of the others we need for the equation to be balanced:
    // bit clunky way to calculate it, for now ok
    let free_count = 4.0;
    // move the c4 vector to right to define the other variables in terms of it
    let c4_right: Vec<f64> = c4.into_iter().map(|value| value * -1.0).collect();
    let first_mol_count = c4_right[0] * free_count;
    assert_eq!(first_mol_count, 1.0);
    let second_mol_count = c4_right[1] * free_count;
    assert_eq!(second_mol_count, 5.0);
    let third_mol_count = c4_right[2] * free_count;
    assert_eq!(third_mol_count, 3.0);

    let c1 = rref_a.col(0);
    let c2 = rref_a.col(1);
    let c3 = rref_a.col(2);
    let c4 = rref_a.col(3);

    let m1 = Molecule(Vector3::new(c1[0], c1[1], c1[2]));
    let m2 = Molecule(Vector3::new(c2[0], c2[1], c2[2]));
    let m3 = Molecule(Vector3::new(c3[0], c3[1], c3[2]));
    let m4 = Molecule(Vector3::new(c4[0], c4[1], c4[2]));

    // let matrix = vectors_to_dmatrix(&[
    //     candidate.input[0].0.clone().map(|e| e as f64),
    //     candidate.input[1].0.clone().map(|e| e as f64),
    //     // move output to left side to form matrix == 0: multiply coefficients with -1
    //     candidate.output[0].0.clone().map(|e| e as f64 * -1.0),
    //     candidate.output[1].0.clone().map(|e| e as f64 * -1.0),
    // ]);
    // // let matrix = DMatrix::from_columns(&[
    // //     candidate.input[0].0.clone().map(|e| e as f64),
    // //     candidate.input[1].0.clone().map(|e| e as f64),
    // //     candidate.output[0].0.clone().map(|e| e as f64),
    // //     candidate.output[1].0.clone().map(|e| e as f64),
    // // ]);
    // println!("input matrix: {:?}", matrix);

    // // let a = DMatrix::from_row_slice(2, 3, &[
    // //     1.0, 2.0, 3.0,
    // //     4.0, 5.0, 6.0
    // // ]);
    // // let b = DVector::from_row_slice(&[7.0, 8.0]);
    // let b = DVector::from_row_slice(&[0.0, 0.0, 0.0]);

    // // Solve using the least-squares method
    // let least_squares_solution = matrix.svd(true, true).solve(&b, 0.0).unwrap();
    // println!("least_squares_solution: {:?}", least_squares_solution);

    // // let b = Vector3::new(0.0, 0.0, 0.0);
    // // let solution = matrix.qr().solve(&b);
    // // println!("solution: {:?}", solution);

    Reaction {
        input: vec![m1, m2],
        output: vec![m3, m4],
    }
}

fn vectors_to_dmatrix(cols: &[Vector3<f64>]) -> DMatrix<f64> {
    let ncols = cols.len();
    let nrows = cols[0].len();

    let mut matrix_data = Vec::with_capacity(ncols * nrows);
    for i in 0..nrows {
        for j in 0..ncols {
            matrix_data.push(cols[j][i]);
        }
    }

    DMatrix::from_row_slice(nrows, ncols, &matrix_data)
}

#[derive(Debug)]
struct ReactionMoleculeUnits {
    input: Vec<Molecule>,
    output: Vec<Molecule>,
}

#[derive(Debug)]
struct Reaction {
    input: Vec<Molecule>,
    output: Vec<Molecule>,
}

#[derive(Debug, Clone)]
struct Molecule(Vector3<f64>);

struct Atoms {
    element: Element,
    quantity: i32,
}

enum Element {
    O,
    C,
    H, //..
}

impl Element {
    fn as_list() -> Vec<Element> {
        vec![Element::O, Element::C, Element::H]
    }
}

struct Mol {
    o: u32,
    c: u32,
    h: u32, //..
}

impl Default for Mol {
    fn default() -> Self {
        Self { o: 0, c: 0, h: 0 }
    }
}

fn to_vector_molecule(m: Mol) -> Molecule {
    Molecule(Vector3::new(m.c as f64, m.h as f64, m.o as f64))
}

#[cfg(test)]
mod test {
    use bevy::prelude::default;

    use super::{balance, to_vector_molecule, Mol, ReactionMoleculeUnits};

    #[test]
    fn test() {
        let mol1 = to_vector_molecule(Mol {
            c: 3,
            h: 8,
            ..default()
        });
        let mol2 = to_vector_molecule(Mol { o: 2, ..default() });
        let mol3 = to_vector_molecule(Mol {
            c: 1,
            o: 2,
            ..default()
        });
        let mol4 = to_vector_molecule(Mol {
            h: 2,
            o: 1,
            ..default()
        });

        let units = ReactionMoleculeUnits {
            input: vec![mol1, mol2],
            output: vec![mol3, mol4],
        };

        let balanced_reaction = balance(units);

        println!("balanced reaction: {:?}", balanced_reaction);

        // let reaction_mol1 = reaction.input[0].clone();
        // let reaction_mol2 = reaction.input[1].clone();
        // let reaction_mol3 = reaction.output[0].clone();
        // let reaction_mol4 = reaction.output[1].clone();

        // assert_eq!(reaction_mol1.0, Vector3::new(0, 0, 0));
        // assert_eq!(reaction_mol2.0, Vector3::new(0, 0, 0));
        // assert_eq!(reaction_mol3.0, Vector3::new(0, 0, 0));
        // assert_eq!(reaction_mol4.0, Vector3::new(0, 0, 0));
    }
}

// fn to_lin_alg_vec(molecule: Molecule) -> Vector3<u32> {
//     Vector3::new(1, 1, 1)
//     Vector3::new(molecule., 1, 1)

// }
