use std::vec;

use rand::prelude::*;

// -----------------------------------------------------------------------------

use crate::{func::*, util::factorial};

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

pub fn poly_eval(coeffs: &[f64], x: f64) -> f64 {
    coeffs
        .iter()
        .enumerate()
        .map(|(i, c)| c * x.powi(i as i32))
        .sum()
}

pub fn poly_scale(coeffs: &[f64], scalar: f64) -> Vec<f64> {
    coeffs.iter().map(|c| c * scalar).collect()
}

pub fn poly_mul(coeffs1: &[f64], coeffs2: &[f64]) -> Vec<f64> {
    let mut coeffs = vec![0.0; coeffs1.len() + coeffs2.len() - 1];

    for (k, c) in coeffs.iter_mut().enumerate() {
        for i in 0..=k {
            let a = coeffs1.get(i).copied().unwrap_or(0.0);
            let b = coeffs2.get(k - i).copied().unwrap_or(0.0);

            *c += a * b;
        }
    }

    coeffs
}

pub fn poly_derivative(coeffs: &[f64]) -> Vec<f64> {
    let mut new_coeffs = Vec::new();

    for (i, c) in coeffs.iter().enumerate().skip(1) {
        new_coeffs.push(c * i as f64);
    }

    new_coeffs
}

// legendre polynomials ========================================================

/// Returns the coefficients of the nth Legendre polynomial using the Rodrigues formula.
fn get_legendre_rodrigues(n: usize) -> Vec<f64> {
    // a(x) = (x^2 - 1)
    let a = vec![-1.0, 0.0, 1.0];
    
    // (x^2 - 1)^n
    let mut coeffs = vec![1.0];
    for _ in 0..n {
        coeffs = poly_mul(&coeffs, &a);
    }

    // d^n/dx^n (x^2 - 1)^n
    // let c = (0..n).fold(vec![1.0], |acc, _| poly_derivative(&acc));
    for _ in 0..n {
        coeffs = poly_derivative(&coeffs);
    }

    // 1 / (2^n * n!)
    let two_pow_n = (2.0f64).powi(n as i32);
    let n_fact = factorial(n as u32) as f64;
    let scalar = 1.0 / (two_pow_n * n_fact);

    // p_n(x) = scalar * d^n/dx^n (x^2 - 1)^n
    poly_scale(&coeffs, scalar)
}


// tests =======================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_legendre() {
        for i in 0..=10 {
            let p = Polynomial::new_with_coefficients(&get_legendre_rodrigues(i));
            let f = p.to_function_of_x();

            println!("P_{i}(x) = {f}");
        }

    }
}
