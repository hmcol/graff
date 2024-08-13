use crate::{fn_mul, Function};

// methods =====================================================================

#[derive(Debug, Clone, Copy)]
pub enum IntMethod {
    Midpoint(usize),
    Trapezoidal(usize),
    CompositeTrapezoidal(usize),
}

pub fn integrate(f: &Function, interval: (f64, f64), method: IntMethod) -> f64 {
    match method {
        IntMethod::Midpoint(n) => int_midpoint(f, interval, n),
        IntMethod::Trapezoidal(n) => int_trapezoidal(f, interval, n),
        IntMethod::CompositeTrapezoidal(n) => int_composite_trapezoidal(f, interval, n),
    }
}

// =============================================================================

/// computes the integral of f over the interval [a, b] using the midpoint rule
/// with n subintervals
pub fn int_midpoint(f: &Function, (a, b): (f64, f64), n: usize) -> f64 {
    // width of each subinterval
    let delta = (b - a) / (n as f64);

    // sample point in the middle of subinterval
    let x_0 = a + delta / 2.0;

    let mut sum = 0.0;
    for i in 0..n {
        let x = x_0 + delta * (i as f64);
        sum += f.eval([x]) * delta;
    }
    sum
}

/// computes the integral of f over the interval [a, b] using the trapezoidal rule
/// with n subintervals
pub fn int_trapezoidal(f: &Function, (a, b): (f64, f64), n: usize) -> f64 {
    // width of each subinterval
    let delta = (b - a) / (n as f64);

    let mut sum = 0.0;
    // i = 0, 1, ..., n - 1 (since we calculate x_i and x_{i+1})
    for i in 0..n {
        // left and right endpoints of subinterval
        let x0 = a + delta * (i as f64);
        let x1 = a + delta * ((i + 1) as f64);

        // area of trapezoid = (f(x_0) + f(x_1)) * delta / 2
        sum += (f.eval([x0]) + f.eval([x1])) * delta / 2.0;
    }
    sum
}

/// computes the integral of f over the interval [a, b] using the composite trapezoidal rule
/// with n subintervals
pub fn int_composite_trapezoidal(f: &Function, (a, b): (f64, f64), n: usize) -> f64 {
    // width of each subinterval
    let delta = (b - a) / (n as f64);

    let mut sum = 0.0;

    // i = 0, 1, ..., n
    for i in 0..=n {
        // f(x_i) where x_i = a + delta * i
        let fx = f.eval([a + delta * (i as f64)]);

        let contribution = if i == 0 || i == n {
            // left and right of total interval only counted once
            fx / 2.0
        } else {
            // all other points counted twice
            fx
        };

        sum += contribution * delta;
    }

    sum
}

// =============================================================================

pub fn int_inner_product(
    f: &Function,
    g: &Function,
    interval: (f64, f64),
    method: IntMethod,
) -> f64 {
    let f_times_g = fn_mul(f.clone(), g.clone());
    integrate(&f_times_g, interval, method)
}

// tests =======================================================================

#[cfg(test)]
mod test {
    use super::*;
    use crate::func::*;

    #[test]
    fn test_int() {
        // f(x) = e^(-x^2)
        let f = fn_exp(fn_mul(fn_const(-1.0), fn_powi(X, 2)));

        let expected_int = 0.746824132812;

        for num in [2, 3, 4, 5, 10, 15, 20, 100, 1000, 10000] {
            let int_m = int_midpoint(&f, (0.0, 1.0), num);
            let int_t = int_trapezoidal(&f, (0.0, 1.0), num);
            let int_ct = int_composite_trapezoidal(&f, (0.0, 1.0), num);

            // print all three integral approximation errors in a nicely formatted row using scientific notation
            println!(
                "n = {:5} | e_m = {:1.2e} | e_t = {:1.2e} | e_ct = {:1.2e}",
                num,
                int_m - expected_int,
                int_t - expected_int,
                int_ct - expected_int
            );
        }
    }

    #[test]
    fn test_inner_product() {
        // f(x) = e^(-x^2)
        let f = fn_exp(fn_neg(fn_powi(X, 2)));
        // g(x) = 1 - x
        let g = fn_sub(fn_const(1.0), X);

        let expected_int = 1.493_648_265_624_854;

        let interval = (-1.0, 1.0);

        for num in [2, 3, 4, 5, 10, 15, 20, 100, 1000, 10000] {
            let int_m = int_inner_product(&f, &g, interval, IntMethod::Midpoint(num));
            let int_t = int_inner_product(&f, &g, interval, IntMethod::Trapezoidal(num));
            let int_ct = int_inner_product(&f, &g, interval, IntMethod::CompositeTrapezoidal(num));

            // print all three integral approximation errors in a nicely formatted row using scientific notation
            println!(
                "n = {:5} | e_m = {:1.2e} | e_t = {:1.2e} | e_ct = {:1.2e}",
                num,
                int_m - expected_int,
                int_t - expected_int,
                int_ct - expected_int
            );
        }
    }
}
