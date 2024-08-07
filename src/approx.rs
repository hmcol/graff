use std::vec;

use rand::{distributions::Uniform, prelude::*};

// -----------------------------------------------------------------------------

use crate::{func::Function, poly::Polynomial, Evaluate};

// =============================================================================

/// 
pub fn compute_gradient_descent_step(
    f: &Function,
    p: &Polynomial,
    interval: (f64, f64),
    step_size: f64,
) -> Polynomial {
    let grad = error_gradient(f, p, interval);

    let mut new_coefficients = vec![0.0; p.degree() + 1];

    for (k, c) in p.coefficients.iter().enumerate() {
        new_coefficients[k] = c - step_size * grad[k];
    }

    Polynomial::new_with_coefficients(&new_coefficients)
}

/// returns the gradient of the error function e = (p - f)^2
fn error_gradient(f: &Function, p: &Polynomial, interval: (f64, f64)) -> Vec<f64> {
    // options
    let num_samples = 1000;

    // data
    let mut grad = vec![0.0; p.degree() + 1];

    // random sample points at which to compute the error gradient
    let mut rng = rand::thread_rng();
    let distr = Uniform::from(interval.0..interval.1);

    let xs: Vec<f64> = distr.sample_iter(&mut rng).take(num_samples).collect();

    for x in xs {
        let fx = f.eval(x);
        let px = p.eval(x);

        for (k, c) in p.coefficients.iter().enumerate() {
            // d/dc_k (p_c(x) - f(x))^2
            // c = [c_0, c_1, ..., c_d]
            grad[k] += (px - fx) * x.powi(k as i32) / (num_samples as f64);
        }
    }

    grad
}
