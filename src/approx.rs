use std::vec;

// -----------------------------------------------------------------------------

use crate::{func::*, util::sample_interval_random, poly::poly_eval};

// =============================================================================

/// returns a new polynomial that is the result of one step of gradient descent
pub fn compute_gradient_descent_step(
    f: &Function,
    coeffs: &mut [f64],
    interval: (f64, f64),
    sample_size: usize,
    step_size: f64,
) {
    let xs = sample_interval_random(interval, sample_size);
    let grad = average_error_gradient(f, coeffs, &xs);

    for k in 0..coeffs.len() {
        coeffs[k] += step_size * grad[k];
    }
}

/// returns the gradient of the error function e = (p - f)^2 averaged over a
/// random sample of the interval
fn average_error_gradient(f: &Function, coeffs: &[f64], xs: &[f64]) -> Vec<f64> {
    let degree = coeffs.len() - 1;
    let mut grad = vec![0.0; coeffs.len()];

    for &x in xs {
        let fx = f.eval([x]);
        let px = poly_eval(coeffs, x);

        for k in 0..degree {
            // c = [c_0, c_1, ..., c_d]
            // d/dc_k (f(x) - p_c(x))^2 = 2(p_c(x) - f(x)) * x^k
            // just ignore the 2 and it gets sucked into learning rate i guess
            grad[k] += (fx - px) * x.powi(k as i32) / (xs.len() as f64);
        }
    }

    grad
}

// -----------------------------------------------------------------------------
