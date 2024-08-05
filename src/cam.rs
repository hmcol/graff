use macroquad::prelude::*;

// -----------------------------------------------------------------------------

use crate::func::Function;
use crate::num::Point;

// =============================================================================

#[derive(Debug)]
pub struct Camera {
    pub center: Point,
    width: f64,
    height: f64,
}

impl Camera {
    pub fn new(center: Point, width: f64, height: f64) -> Self {
        Camera {
            center,
            width,
            height,
        }
    }

    // camera info -------------------------------------------------------------

    pub fn left(&self) -> f64 {
        self.center.x - self.width / 2.0
    }

    pub fn right(&self) -> f64 {
        self.center.x + self.width / 2.0
    }

    pub fn top(&self) -> f64 {
        self.center.y + self.height / 2.0
    }

    pub fn bottom(&self) -> f64 {
        self.center.y - self.height / 2.0
    }

    /// leaves the width the same and adjusts the height to match the aspect ratio
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f64) {
        self.height = self.width / aspect_ratio;
    }

    pub fn move_to(&mut self, center: Point) {
        self.center = center;
    }

    // drawing -----------------------------------------------------------------

    pub fn draw_grid(&self) {
        // vertical lines at x = n for integers n inside the camera width

        let v_min = self.left().floor() as i32;
        let v_max = self.right().ceil() as i32;

        for v in v_min..=v_max {
            let mut color = GRAY;
            let mut stroke = 1.0;

            if v == 0 {
                color = BLACK;
                stroke = 2.0;
            }

            let x = self.euc_to_screen_x(v as f64);
            draw_line(x, 0.0, x, screen_height(), stroke, color);
        }

        // horizontal lines at y = n for integers n inside the camera height

        let h_min = self.bottom().floor() as i32;
        let h_max = self.top().ceil() as i32;

        for h in h_min..=h_max {
            let mut color = GRAY;
            let mut stroke = 1.0;

            if h == 0 {
                color = BLACK;
                stroke = 2.0;
            }

            let y = self.euc_to_screen_y(h as f64);
            draw_line(0.0, y, screen_width(), y, stroke, color);
        }
    }

    pub fn draw_function(&self, f: &Function) {
        let interval = (self.left(), self.right());
        let samples = f.sample(interval, 1000);

        let screen_points: Vec<(f32, f32)> = samples
            .iter()
            .map(|p| self.euc_to_screen(Point::new(p.0, p.1)))
            .collect();

        for pair in screen_points.windows(2) {
            let (x1, y1) = pair[0];
            let (x2, y2) = pair[1];

            draw_line(x1, y1, x2, y2, 2.0, RED);
        }
    }

    // computations ------------------------------------------------------------

    fn euc_to_screen_x(&self, x: f64) -> f32 {
        // (x - left) + screen_width / cam_width
        ((x - self.left()) * (screen_width() as f64) / self.width) as f32
    }

    fn euc_to_screen_y(&self, y: f64) -> f32 {
        // -(y - top) + screen_height / cam_height
        (-(y - self.top()) * (screen_height() as f64) / self.height) as f32
    }

    fn euc_to_screen(&self, p: Point) -> (f32, f32) {
        (self.euc_to_screen_x(p.x), self.euc_to_screen_y(p.y))
    }
}
