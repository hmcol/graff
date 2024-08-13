use std::vec;

// -----------------------------------------------------------------------------

use crate::{
    func::*,
    integration::{int_inner_product, IntMethod},
    polynomial::{get_legendre_rodrigues, poly_eval},
    util::sample_interval_random,
};

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

// =============================================================================

pub fn compute_legendre_approx(f: &Function, n: usize, int_method: IntMethod) -> Function {
    let mut p = fn_const(0.0);

    for k in 0..n {
        let legendre_coeffs = get_legendre_rodrigues(k);
        let legendre_fn = fn_poly(legendre_coeffs);

        // <l_n, h> = int_-1^1 l_n(x) * f(x) dx
        let inner_product = int_inner_product(f, &legendre_fn, (-1.0, 1.0), int_method);

        // (2n + 1) / 2
        let scalar = (2.0 * (k as f64) + 1.0) / 2.0;

        // coefficient a_n for the nth Legendre polynomial
        let a = fn_const(scalar * inner_product);

        let component = fn_mul(a, legendre_fn);

        p = fn_add(p, component);
    }

    p
}

// =============================================================================
