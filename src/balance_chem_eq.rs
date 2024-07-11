#[cfg(test)]
mod test {
    use bevy::prelude::default;
    use nalgebra::Vector3;
    use peroxide::fuga::{matrix, LinearAlgebra, Shape::Col};

    #[test]
    fn balance_chem_eq() {
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

        // to represent the reaction as a matrix, we put all the molecules on the left side of the equation, leaving the zero vector on the right.
        let all_cols = [
            as_vec(units.input[0].clone()),
            as_vec(units.input[1].clone()),
            as_negated_vec(units.output[0].clone()),
            as_negated_vec(units.output[1].clone()),
            vec![0.0, 0.0, 0.0],
        ]
        .concat()
        .to_owned();

        let a = matrix(all_cols, 3, 4, Col);

        // calculate reduced row echelon form
        // needed for the 4x3 system, with x4 as free variable (multiple solutions)
        // not sure how or whether it's possible to do this with the other matrix solvers - seem optimized / to work only for square matrices?
        let rref_a = a.rref();
        println!("rref_a: {:?}", rref_a);

        // x4, the free variable, is the molecule count of the last molecule
        // the count of the other molecules depends on this

        // remember that coefficients are the number of atoms of each element in the respective molecule:
        let c4 = rref_a.col(3);
        println!("c4 (water) coefficients (atom count) {:?}", c4);

        // so if the molecule is e.g. water (H20), we'd have a vector (0, 2, 1) (corresponding to count of C, H, O atoms),
        // and a scalar multiplier (the "x") corresponding to how many of these molecules go into the balanced equation.
        // in our example equation, the water molecule (by virtue of incidentally appearing last in the equation) is "free"
        // so we derive the count of the other molecules from the water we want to get

        // so let's set 4 water molecules and derive the other molecule counts:
        // (bit clunky way to calculate it, for now ok)
        let free_count = 4.0;
        // move the c4 vector to right to define the other variables in terms of it
        let c4_right: Vec<f64> = c4.into_iter().map(|value| value * -1.0).collect();
        let first_mol_count = c4_right[0] * free_count;
        assert_eq!(first_mol_count, 1.0);
        let second_mol_count = c4_right[1] * free_count;
        assert_eq!(second_mol_count, 5.0);
        let third_mol_count = c4_right[2] * free_count;
        assert_eq!(third_mol_count, 3.0);

        // side note: "balanced (chemical) equation" kinda unnecessarily verbose name IMO,
        // balanced means simply that it's a correct equation, if we have incorrect numbers, it's just a false equation.
    }

    #[derive(Debug)]
    pub struct ReactionMoleculeUnits {
        input: Vec<Molecule>,
        output: Vec<Molecule>,
    }

    fn as_vec(mol: Molecule) -> Vec<f64> {
        mol.0.into_iter().map(|e| *e as f64).collect()
    }

    fn as_negated_vec(mol: Molecule) -> Vec<f64> {
        mol.0.into_iter().map(|e| *e as f64 * -1.0).collect()
    }
    #[derive(Debug, Clone)]
    struct Molecule(Vector3<f64>);

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
}
