
// =============================================================================

#[derive(Debug, Clone)]
pub enum Function {
    Var(usize), // picks out the i-th variable of the input x = (x_1, x_2, ..., x_n)
    Const(f64),
    Add(Box<Function>, Box<Function>),
    Sub(Box<Function>, Box<Function>),
    Mul(Box<Function>, Box<Function>),
    Div(Box<Function>, Box<Function>),
    Sin(Box<Function>),
    Cos(Box<Function>),
    Tan(Box<Function>),
    Exp(Box<Function>),       // exponential (e^x)
    Log(Box<Function>),       // natural logarithm (log base e)
    Sum(Vec<Function>),       // could be replaced with recursive Add
    Prod(Vec<Function>),      // could be replaced with recursive Mul
    PowI(Box<Function>, i32), // integer power
}

impl Function {
    pub fn eval<T: AsRef<[f64]>>(&self, args: T) -> f64 {
        let args = args.as_ref();
        match self {
            // if i is out of bounds, return 0, should maybe return an Option::None
            Function::Var(i) => args.get(*i).copied().unwrap_or(0.0),
            Function::Const(c) => *c,
            Function::Add(f, g) => f.eval(args) + g.eval(args),
            Function::Sub(f, g) => f.eval(args) - g.eval(args),
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

// =============================================================================

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

pub fn fn_add(f1: Function, f2: Function) -> Function {
    Function::Add(Box::new(f1), Box::new(f2))
}

pub fn fn_sub(f1: Function, f2: Function) -> Function {
    Function::Sub(Box::new(f1), Box::new(f2))
}

pub fn fn_mul(f1: Function, f2: Function) -> Function {
    Function::Mul(Box::new(f1), Box::new(f2))
}

pub fn fn_div(f1: Function, f2: Function) -> Function {
    Function::Div(Box::new(f1), Box::new(f2))
}

pub fn fn_sin(f: Function) -> Function {
    Function::Sin(Box::new(f))
}

pub fn fn_cos(f: Function) -> Function {
    Function::Cos(Box::new(f))
}

pub fn fn_tan(f: Function) -> Function {
    Function::Tan(Box::new(f))
}

pub fn fn_exp(f: Function) -> Function {
    Function::Exp(Box::new(f))
}

pub fn fn_log(f: Function) -> Function {
    Function::Log(Box::new(f))
}

pub fn fn_sum(fs: Vec<Function>) -> Function {
    Function::Sum(fs)
}

pub fn fn_prod(fs: Vec<Function>) -> Function {
    Function::Prod(fs)
}

pub fn fn_powi(f: Function, n: i32) -> Function {
    Function::PowI(Box::new(f), n)
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
        }
    }
}
