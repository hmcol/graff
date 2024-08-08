use rand::prelude::*;

// -----------------------------------------------------------------------------

use crate::func::*;

// =============================================================================

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Polynomial {
    pub coefficients: Vec<f64>,
}

impl Polynomial {
    pub fn new_with_coefficients(coefficients: &[f64]) -> Self {
        Polynomial {
            coefficients: coefficients.to_vec(),
        }
    }

    pub fn new_random_with_degree(degree: usize) -> Self {
        let mut coefficients = vec![0.0; degree + 1];

        let mut rng = rand::thread_rng();
        for c in coefficients.iter_mut() {
            *c = rng.gen_range(-1.0..1.0);
        }

        Polynomial::new_with_coefficients(&coefficients)
    }

    pub fn degree(&self) -> usize {
        self.coefficients.len() - 1
    }

    pub fn to_function_of_x(&self) -> Function {
        let mut terms = Vec::new();
        for (i, c) in self.coefficients.iter().enumerate() {
            if *c != 0.0 {
                terms.push(fn_mul(fn_const(*c), fn_powi(X, i as i32)));
            }
        }
        fn_sum(terms)
    }
}
