#[cfg(test)]
mod test {
    use approx::assert_relative_eq;
    use peroxide::fuga::{matrix, LinearAlgebra, Shape::Col};

    fn as_resistance_vec(l: Loop) -> Vec<f64> {
        vec![l.resistance_l1, l.resistance_l2, l.resistance_l3]
    }

    // a loop with its resistance coefficients, coming from loop 1, loop 2, loop 3
    #[derive(Debug)]
    struct Loop {
        resistance_l1: f64,
        resistance_l2: f64,
        resistance_l3: f64,
        voltage: f64,
    }

    // based on an example from Lay's linear algebra
    #[test]
    fn find_loop_currents() {
        let l1 = Loop {
            resistance_l1: 11.0,
            resistance_l2: -3.0,
            resistance_l3: 0.0,
            voltage: 30.0,
        };
        let l2 = Loop {
            resistance_l1: -3.0,
            resistance_l2: 6.0,
            resistance_l3: -1.0,
            voltage: 5.0,
        };
        let l3 = Loop {
            resistance_l1: 0.0,
            resistance_l2: -1.0,
            resistance_l3: 3.0,
            // -5 from loop 2 and -20 from loop 3 (sign due to direction)
            voltage: -25.0,
        };

        let loop_voltages = vec![l1.voltage, l2.voltage, l3.voltage];

        // finding the currents for 3 loops in a circuit, based on V = IR respectively
        // we build one such equation per loop and put it in a system, to calculate the currents
        // it's not entirely clear to me how the voltage sources across the loops remain largely independent, but not digging into this.
        let all_cols = [
            as_resistance_vec(l1),
            as_resistance_vec(l2),
            as_resistance_vec(l3),
            loop_voltages,
        ]
        .concat()
        .to_owned();
        let a = matrix(all_cols, 3, 4, Col);
        // println!("a: {:?}", a);

        let rref_a = a.rref();
        println!("rref_a: {:?}", rref_a);

        let c4 = rref_a.col(3);
        println!(
            "result: l1 current: {}, l2 current: {}, l3 current: {}",
            c4[0], c4[1], c4[2]
        );

        assert_relative_eq!(c4[0], 3.0, epsilon = 0.000000000000000000000000001);
        assert_relative_eq!(c4[1], 1.0, epsilon = 0.000000000000000000000000001);
        assert_relative_eq!(c4[2], -8.0, epsilon = 0.000000000000000000000000001);
    }
}
