pub trait Evaluate {
    fn eval(&self, x: f64) -> f64;
    fn sample(&self, interval: (f64, f64), steps: usize) -> Vec<(f64, f64)> {
        let delta = (interval.1 - interval.0) / steps as f64;

        (0..=steps)
            .map(|i| interval.0 + delta * (i as f64)) // x_i = left + delta_x * i
            .map(|x| (x, self.eval(x))) // point_i = (x_i, f(x_i))
            .collect()
    }
}

// =============================================================================

#[derive(Debug, Clone)]
pub enum Function {
    Identity,
    Constant(f64),
    Add(Box<Function>, Box<Function>),
    Sub(Box<Function>, Box<Function>),
    Mul(Box<Function>, Box<Function>),
    Div(Box<Function>, Box<Function>),
    Sin(Box<Function>),
    Cos(Box<Function>),
    Tan(Box<Function>),
    Exp(Box<Function>), // exponential (e^x)
    Log(Box<Function>), // natural logarithm (log base e)
}

impl Evaluate for Function {
    fn eval(&self, x: f64) -> f64 {
        match self {
            Function::Identity => x,
            Function::Constant(c) => *c,
            Function::Add(f1, f2) => f1.eval(x) + f2.eval(x),
            Function::Sub(f1, f2) => f1.eval(x) - f2.eval(x),
            Function::Mul(f1, f2) => f1.eval(x) * f2.eval(x),
            Function::Div(f1, f2) => f1.eval(x) / f2.eval(x),
            Function::Sin(f) => f.eval(x).sin(),
            Function::Cos(f) => f.eval(x).cos(),
            Function::Tan(f) => f.eval(x).tan(),
            Function::Exp(f) => f.eval(x).exp(),
            Function::Log(f) => f.eval(x).ln(),
        }
    }
}

// function constructors -------------------------------------------------------

pub const X: Function = Function::Identity;

pub fn fn_const(c: f64) -> Function {
    Function::Constant(c)
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
