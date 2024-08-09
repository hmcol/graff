use std::vec;

// -----------------------------------------------------------------------------

use crate::{func::*, num::sample_interval_random, poly::Polynomial};

// =============================================================================

/// returns a new polynomial that is the result of one step of gradient descent
pub fn compute_gradient_descent_step(
    f: &Function,
    p: &Polynomial,
    interval: (f64, f64),
    sample_size: usize,
    step_size: f64,
) -> Polynomial {
    let grad = error_gradient(f, p, interval, sample_size);

    let mut new_coefficients = vec![0.0; p.degree() + 1];

    for (k, c) in p.coefficients.iter().enumerate() {
        new_coefficients[k] = c - step_size * grad[k];
    }

    Polynomial::new_with_coefficients(&new_coefficients)
}

/// returns the gradient of the error function e = (p - f)^2 averaged over a
/// random sample of the interval
fn error_gradient(
    f: &Function,
    p: &Polynomial,
    interval: (f64, f64),
    sample_size: usize,
) -> Vec<f64> {
    let xs = sample_interval_random(interval, sample_size);

    let mut grad = vec![0.0; p.degree() + 1];

    for x in xs {
        let fx = f.eval([x]);
        let px = p.to_function_of_x().eval([x]);

        for (k, _) in p.coefficients.iter().enumerate() {
            // c = [c_0, c_1, ..., c_d]
            // d/dc_k (p_c(x) - f(x))^2 = 2(p_c(x) - f(x)) * x^k
            // just ignore the 2 and it gets sucked into learning rate i guess
            grad[k] += (px - fx) * x.powi(k as i32) / (sample_size as f64);
        }
    }

    grad
}

// -----------------------------------------------------------------------------

