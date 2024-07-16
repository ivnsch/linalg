#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use peroxide::fuga::{matrix, LinearAlgebra, Shape::Col};

    fn as_vec(nutrient: Nutrient) -> Vec<f64> {
        vec![nutrient.protein, nutrient.carbs, nutrient.fat]
    }

    #[derive(Debug)]
    struct Nutrient {
        protein: f64,
        carbs: f64,
        fat: f64,
    }

    // based on an example from Lay's linear algebra
    #[test]
    fn balance_diet() {
        // some nutrients with their respective macro distributions
        let milk = Nutrient {
            protein: 36.0,
            carbs: 52.0,
            fat: 0.0,
        };
        let soy = Nutrient {
            protein: 51.0,
            carbs: 34.0,
            fat: 7.0,
        };
        let whey = Nutrient {
            protein: 13.0,
            carbs: 74.0,
            fat: 1.1,
        };

        // what the diet recommends
        // ok-ish to reuse struct here: an aggregate of macros is technically a nutrient
        let desired_total_amount = Nutrient {
            protein: 33.0,
            carbs: 45.0,
            fat: 3.0,
        };

        // we create a system of 3 equations, which add respectively protein, etc. to output the required total
        // e.g. the first would be 36x + 51y + 13z = 33 total protein
        // with vector of unknowns [x, y, z] for milk, soy, whey quantity
        // by reducing this matrix, we get the required quantities of ingredients that follow the diet [33, 45, 3]
        // proof: e.g. 36 * 0.277 + 51 * 0.392 + 13 * 0.233 = 32.993 =~ 33 protein
        let all_cols = [
            as_vec(milk),
            as_vec(soy),
            as_vec(whey),
            as_vec(desired_total_amount),
        ]
        .concat()
        .to_owned();
        let a = matrix(all_cols, 3, 4, Col);

        let rref_a = a.rref();
        // println!("rref_a: {:?}", rref_a);

        let c4 = rref_a.col(3);
        println!("result: milk: {}, soy: {}, whey: {}", c4[0], c4[1], c4[2]);

        assert_relative_eq!(c4[0], 0.277, epsilon = 0.001);
        assert_relative_eq!(c4[1], 0.392, epsilon = 0.001);
        assert_relative_eq!(c4[2], 0.233, epsilon = 0.001);
    }
}
