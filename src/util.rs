use rand::{thread_rng, Rng};

// =============================================================================

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point {
    pub x: f64,
    pub y: f64,
}

impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    pub fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// random ======================================================================

pub fn sample_interval_equidistributed(interval: (f64, f64), steps: usize) -> Vec<f64> {
    let delta = (interval.1 - interval.0) / steps as f64;

    (0..=steps)
        .map(|i| interval.0 + delta * (i as f64)) // x_i = left + delta_x * i
        .collect()
}

pub fn sample_interval_random(interval: (f64, f64), steps: usize) -> Vec<f64> {
    let mut rng = thread_rng();
    (0..steps)
        .map(|_| rng.gen_range(interval.0..interval.1))
        .collect()
}

// integer operations ==========================================================

pub fn factorial(n: u32) -> u64 {
    (1..=n as u64).product()
}