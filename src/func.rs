// create modules --------------------------------------------------------------

use crate::polynomial::{poly_eval, poly_mul};

// Variable Index ==============================================================

/// type for identifying variables in functions
///
/// args are given to functions as slices `[x0,x1,...,xn]`
/// the i-th variable is x_i and i is the VarIdx
type VarIdx = usize;

// Function Expression =========================================================

#[derive(Debug, Clone)]
pub enum Function {
    Var(VarIdx), // picks out the i-th variable of the input x = (x_1, x_2, ..., x_n)
    Const(f64),
    Add(Box<Function>, Box<Function>),
    Sub(Box<Function>, Box<Function>),
    Neg(Box<Function>),
    Mul(Box<Function>, Box<Function>),
    Div(Box<Function>, Box<Function>),
    Sin(Box<Function>),
    Cos(Box<Function>),
    Tan(Box<Function>),
    Exp(Box<Function>),           // exponential (e^x)
    Log(Box<Function>),           // natural logarithm (log base e)
    Sum(Vec<Function>),           // could be replaced with recursive Add
    Prod(Vec<Function>),          // could be replaced with recursive Mul
    PowI(Box<Function>, i32),     // integer power
    Poly(Vec<f64>),               // polynomial with constant coefficients sum_i c_i*x^i
    PolyF(Vec<Function>, VarIdx), // polynomial with function coefficients sum_i f_i(x)*x^i
}

// evaluation ------------------------------------------------------------------

impl Function {
    pub fn eval<T: AsRef<[f64]>>(&self, args: T) -> f64 {
        let args = args.as_ref();
        match self {
            // if i is out of bounds, return 0, should maybe return an Option::None
            Function::Var(i) => get_arg(args, *i),
            Function::Const(c) => *c,
            Function::Add(f, g) => f.eval(args) + g.eval(args),
            Function::Sub(f, g) => f.eval(args) - g.eval(args),
            Function::Neg(f) => -f.eval(args),
            Function::Mul(f, g) => f.eval(args) * g.eval(args),
            Function::Div(f, g) => f.eval(args) / g.eval(args),
            Function::Sin(f) => f.eval(args).sin(),
            Function::Cos(f) => f.eval(args).cos(),
            Function::Tan(f) => f.eval(args).tan(),
            Function::Exp(f) => f.eval(args).exp(),
            Function::Log(f) => f.eval(args).ln(),
            Function::Sum(fs) => fs.iter().map(|f| f.eval(args)).sum(),
            Function::Prod(fs) => fs.iter().map(|f| f.eval(args)).product(),
            Function::PowI(f, n) => f.eval(args).powi(*n),
            Function::Poly(coeffs) => poly_eval(coeffs, get_arg(args, 0)),
            Function::PolyF(fs, i) => {
                let x = get_arg(args, *i);
                fs.iter()
                    .enumerate()
                    .map(|(i, f)| f.eval(args) * x.powi(i as i32))
                    .sum()
            }
        }
    }

    pub fn sample(&self, interval: (f64, f64), steps: usize) -> Vec<(f64, f64)> {
        let delta = (interval.1 - interval.0) / steps as f64;

        (0..=steps)
            .map(|i| interval.0 + delta * (i as f64)) // x_i = left + delta_x * i
            .map(|x| (x, self.eval([x]))) // point_i = (x_i, f(x_i))
            .collect()
    }
}

// utility ---------------------------------------------------------------------

fn get_arg(args: &[f64], i: usize) -> f64 {
    args.get(i).copied().unwrap_or(0.0)
}

impl From<f64> for Function {
    fn from(c: f64) -> Self {
        fn_const(c)
    }
}

// function constructors -------------------------------------------------------

pub const X: Function = Function::Var(0);
pub const Y: Function = Function::Var(1);
pub const Z: Function = Function::Var(2);

pub fn fn_var(i: usize) -> Function {
    Function::Var(i)
}

pub fn fn_const(c: f64) -> Function {
    Function::Const(c)
}

/// returns the function (f1 + f2)(x) := f1(x) + f2(x)
///
/// Performs simplifications:
/// - const(c1) + const(c2) := const(c1 + c2)
/// - const(0) + f := f
/// - f + const(0) := f
pub fn fn_add(f1: Function, f2: Function) -> Function {
    match (f1, f2) {
        (Function::Const(c1), Function::Const(c2)) => fn_const(c1 + c2),
        (Function::Const(0.0), f) => f,
        (f, Function::Const(0.0)) => f,
        (f1, f2) => Function::Add(Box::new(f1), Box::new(f2)),
    }
}

/// returns the function (f1 - f2)(x) := f1(x) - f2(x)
///
/// Performs simplifications:
/// - const(c1) - const(c2) := const(c1 - c2)
/// - f - const(0) := f
/// - const(0) - f := neg(f)
pub fn fn_sub(f1: Function, f2: Function) -> Function {
    match (f1, f2) {
        (Function::Const(c1), Function::Const(c2)) => fn_const(c1 - c2),
        (f, Function::Const(0.0)) => f,
        (Function::Const(0.0), f) => fn_neg(f),
        (f1, f2) => Function::Sub(Box::new(f1), Box::new(f2)),
    }
}

/// returns the function -f(x) := -f(x)
///
/// Performs simplifications:
/// - `-const(c) := const(-c)`
pub fn fn_neg(f: Function) -> Function {
    match f {
        Function::Const(c) => fn_const(-c),
        Function::Neg(g) => *g,
        f => Function::Neg(Box::new(f)),
    }
}

/// returns the function (f1 * f2)(x) := f1(x) * f2(x)
///
/// Performs simplifications:
/// - const(c1) * const(c2) := const(c1 * c2)
/// - const(0) * f := const(0)
/// - f * const(0) := const(0)
/// - const(1) * f := f
/// - f * const(1) := f
/// - poly(c1) * poly(c2) := poly(c1 * c2)
pub fn fn_mul(f1: Function, f2: Function) -> Function {
    match (f1, f2) {
        (Function::Const(c1), Function::Const(c2)) => fn_const(c1 * c2),
        (Function::Const(0.0), _) => fn_const(0.0),
        (_, Function::Const(0.0)) => fn_const(0.0),
        (Function::Const(1.0), f) => f,
        (f, Function::Const(1.0)) => f,
        (Function::Poly(c1), Function::Poly(c2)) => fn_poly(poly_mul(&c1, &c2)),
        (f1, f2) => Function::Mul(Box::new(f1), Box::new(f2)),
    }
}

/// returns the function (f1 / f2)(x) := f1(x) / f2(x)
///
/// Performs simplifications:
/// - const(c1) / const(c2) := const(c1 / c2)   // panics if c2 == 0
/// - f / const(1) := f
pub fn fn_div(f1: Function, f2: Function) -> Function {
    match (f1, f2) {
        (Function::Const(c1), Function::Const(c2)) => fn_const(c1 / c2),
        (f, Function::Const(1.0)) => f,
        (f1, f2) => Function::Div(Box::new(f1), Box::new(f2)),
    }
}

/// returns the function sin(f(x))
///
/// Performs simplifications:
/// - sin(const(c)) := const(sin(c))
pub fn fn_sin(f: Function) -> Function {
    match f {
        Function::Const(c) => fn_const(c.sin()),
        f => Function::Sin(Box::new(f)),
    }
}

/// returns the function cos(f(x))
///
/// Performs simplifications:
/// - cos(const(c)) := const(cos(c))
pub fn fn_cos(f: Function) -> Function {
    match f {
        Function::Const(c) => fn_const(c.cos()),
        f => Function::Cos(Box::new(f)),
    }
}

/// returns the function tan(f(x))
///
/// Performs simplifications:
/// - tan(const(c)) := const(tan(c))
pub fn fn_tan(f: Function) -> Function {
    match f {
        Function::Const(c) => fn_const(c.tan()),
        f => Function::Tan(Box::new(f)),
    }
}

/// returns the function exp(f(x))
///
/// Performs simplifications:
/// - exp(const(c)) := const(exp(c))
pub fn fn_exp(f: Function) -> Function {
    match f {
        Function::Const(c) => fn_const(c.exp()),
        f => Function::Exp(Box::new(f)),
    }
}

/// returns the function log(f(x))
///
/// Performs simplifications:
/// - log(const(c)) := const(log(c))
pub fn fn_log(f: Function) -> Function {
    match f {
        Function::Const(c) => fn_const(c.ln()),
        f => Function::Log(Box::new(f)),
    }
}

/// returns the function (f1 + f2 + ... + fn)(x) := f1(x) + f2(x) + ... + fn(x)
///
/// Performs no simplifications
pub fn fn_sum(fs: Vec<Function>) -> Function {
    Function::Sum(fs)
}

/// returns the function (f1 * f2 * ... * fn)(x) := f1(x) * f2(x) * ... * fn(x)
///
/// Performs no simplifications
pub fn fn_prod(fs: Vec<Function>) -> Function {
    Function::Prod(fs)
}

/// returns the function f^n(x) := f(x)^n
///
/// Performs simplifications:
/// - const(c)^n := const(c^n)
/// - f^0 := const(1)
/// - f^1 := f
pub fn fn_powi(f: Function, n: i32) -> Function {
    match (f, n) {
        (Function::Const(c), n) => fn_const(c.powi(n)),
        (_, 0) => fn_const(1.0),
        (f, 1) => f,
        (f, n) => Function::PowI(Box::new(f), n),
    }
}

/// returns the function sum_k c_k*x^k where coeffs = [c0,...,cn] and x = x_i
/// poly([c0,...,cn], i)(x) := c0 + c1*xi + ... + cn*xi^n
///
/// Performs simplifications:
/// - `poly([]) := const(0)`
/// - `poly([c]) := const(c)`
/// - `poly([c0,...,cm,0,...,0]) := poly([c0,...,cm])`
pub fn fn_poly(coeffs: Vec<f64>) -> Function {
    if coeffs.is_empty() {
        return fn_const(0.0);
    }
    if coeffs.len() == 1 {
        return fn_const(coeffs[0]);
    }

    let mut coeffs = coeffs;

    while let Some(&c) = coeffs.last() {
        if c == 0.0 {
            coeffs.pop();
        } else {
            break;
        }
    }

    Function::Poly(coeffs)
}

/// returns the function sum_k f_k(x)*x^k where fs = [f0,...,fn] and x = x_i
pub fn fn_poly_with_roots(roots: &[f64]) -> Function {
    roots
        .iter()
        .map(|&r| fn_poly(vec![-r, 1.0]))
        .fold(fn_const(1.0), fn_mul)
}

// =============================================================================

/// symbolically computes the partial derivative of f with respect to the i-th variable
pub fn fn_pdv(f: &Function, i: usize) -> Function {
    match f {
        Function::Var(j) => {
            if i == *j {
                fn_const(1.0)
            } else {
                fn_const(0.0)
            }
        }
        Function::Const(_) => fn_const(0.0),
        Function::Add(f1, f2) => fn_add(fn_pdv(f1, i), fn_pdv(f2, i)),
        Function::Sub(f1, f2) => fn_sub(fn_pdv(f1, i), fn_pdv(f2, i)),
        Function::Neg(f) => fn_neg(fn_pdv(f, i)),
        Function::Mul(f1, f2) => fn_add(
            fn_mul(fn_pdv(f1, i), *f2.clone()),
            fn_mul(*f1.clone(), fn_pdv(f2, i)),
        ),
        Function::Div(f1, f2) => fn_div(
            fn_sub(
                fn_mul(fn_pdv(f1, i), *f2.clone()),
                fn_mul(*f1.clone(), fn_pdv(f2, i)),
            ),
            fn_mul(*f2.clone(), *f2.clone()),
        ),
        Function::Sin(f) => fn_mul(fn_cos(*f.clone()), fn_pdv(f, i)),
        Function::Cos(f) => fn_mul(fn_sub(fn_const(0.0), fn_sin(*f.clone())), fn_pdv(f, i)),
        Function::Tan(f) => fn_mul(
            fn_div(fn_const(1.0), fn_powi(fn_cos(*f.clone()), 2)),
            fn_pdv(f, i),
        ),
        Function::Exp(f) => fn_mul(fn_exp(*f.clone()), fn_pdv(f, i)),
        Function::Log(f) => fn_mul(fn_div(fn_const(1.0), *f.clone()), fn_pdv(f, i)),
        Function::Sum(fs) => fn_sum(fs.iter().map(|f| fn_pdv(f, i)).collect()),
        Function::Prod(fs) => {
            let mut summands = Vec::new();
            for (j, f) in fs.iter().enumerate() {
                let mut factors = fs.clone();
                factors.remove(j);
                summands.push(fn_mul(f.clone(), fn_prod(factors)));
            }
            fn_sum(summands)
        }
        Function::PowI(f, n) => fn_mul(
            fn_const(*n as f64),
            fn_mul(fn_powi(*f.clone(), *n - 1), fn_pdv(f, i)),
        ),
        Function::Poly(coeffs) => {
            let mut new_coeffs = Vec::new();
            for (k, c) in coeffs.iter().enumerate() {
                // looking at term cx^k (i.e., c = c_k)
                if k == 0 {
                    continue;
                }
                // pdv(cx^k) = (c*k)x^(k-1)
                new_coeffs.push(c * (k as f64));
            }
            fn_poly(new_coeffs)
        }
        Function::PolyF(fs, j) => {
            let mut terms = Vec::new();
            for (k, f) in fs.iter().enumerate() {
                // looking at term f(x)*x^k
                // pdv(f(x)*x^k) = pdv(f(x))*x^k + f(x)*pdv(x^k)

                // using product rule: pdv(x^k) = k*x^(k-1)
                let xk = fn_powi(fn_var(*j), k as i32);

                let df = fn_pdv(f, i);
                let dxk = fn_pdv(&xk, i);

                let df_x = fn_mul(df, fn_powi(fn_var(*j), k as i32));
                let f_dx = fn_mul(f.clone(), dxk);

                terms.push(fn_add(df_x, f_dx));
            }
            fn_sum(terms)
        }
    }
}

// could use similar recursive structure for other symbolic manipulation:
// - function simplification (generally replacing subexpressions)
// - displaying with latex
// - fixing variables (given f(x, y), can obtain function f(x) with y fixed to some constant)

// =============================================================================

impl std::fmt::Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Function::Var(i) => write!(f, "x_{}", i),
            Function::Const(c) => write!(f, "{}", c),
            Function::Add(f1, f2) => write!(f, "({} + {})", f1, f2),
            Function::Sub(f1, f2) => write!(f, "({} - {})", f1, f2),
            Function::Neg(g) => write!(f, "-{}", g),
            Function::Mul(f1, f2) => write!(f, "({}*{})", f1, f2),
            Function::Div(f1, f2) => write!(f, "({}/{})", f1, f2),
            Function::Sin(g) => write!(f, "sin({})", g),
            Function::Cos(g) => write!(f, "cos({})", g),
            Function::Tan(g) => write!(f, "tan({})", g),
            Function::Exp(g) => write!(f, "exp({})", g),
            Function::Log(g) => write!(f, "log({})", g),
            Function::Sum(fs) => {
                write!(f, "(")?;
                for (i, func) in fs.iter().enumerate() {
                    if i > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{}", func)?;
                }
                write!(f, ")")
            }
            Function::Prod(fs) => {
                write!(f, "(")?;
                for (i, func) in fs.iter().enumerate() {
                    if i > 0 {
                        write!(f, " * ")?;
                    }
                    write!(f, "{}", func)?;
                }
                write!(f, ")")
            }
            Function::PowI(g, n) => write!(f, "({}^{})", g, n),
            Function::Poly(coeffs) => {
                write!(f, "(")?;
                for (k, c) in coeffs.iter().enumerate() {
                    if k > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{}*x^{}", c, k)?;
                }
                write!(f, ")")
            }
            Function::PolyF(fs, i) => {
                write!(f, "(")?;
                for (j, func) in fs.iter().enumerate() {
                    if j > 0 {
                        write!(f, " + ")?;
                    }
                    write!(f, "{}*x_{}^{}", func, i, j)?;
                }
                write!(f, ")")
            }
        }
    }
}

// tests =======================================================================

#[cfg(test)]
mod test {
    use std::vec;

    use crate::util::sample_interval_equidistributed;

    use super::*;

    #[test]
    fn test_constructor() {
        let roots = [1.0, 2.0, 3.0];

        let p = roots
            .iter()
            .map(|&r| fn_poly(vec![-r, 1.0]))
            .fold(fn_const(1.0), fn_mul);

        print!("{}", p);
    }

    #[test]
    fn test_eval() {
        let f = fn_sub(fn_exp(X), fn_const(1.0));

        let xs = sample_interval_equidistributed((-10.0, 10.0), 20);

        for x in xs {
            let y = f.eval([x]);
            println!("f({}) = {}", x, y);
        }
    }
}
