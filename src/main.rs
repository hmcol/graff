use macroquad::prelude::*;

mod func;

use func::*;

#[macroquad::main("Graphing Program")]
async fn main() {
    let cool_fn: Function = fn_exp(fn_div(fn_const(-1.0), fn_mul(X, X)));
    let f = fn_mul(cool_fn, fn_sin(fn_mul(X, X)));

    let mut cam = CameraState::new(Point::origin(), 20.0, 10.0);


    loop {
        clear_background(WHITE);

        // update camera aspect ration to match screen aspect ratio
        let screen_aspect_ratio = (screen_width() / screen_height()) as f64;
        let desired_cam_height = cam.width() / screen_aspect_ratio;
        cam.set_height(desired_cam_height);

        // // move camera a lil
        // let cam_pos = cam.center();
        // let new_pos = Point::new(cam_pos.x + 0.1, cam_pos.y);
        // cam.move_to(new_pos);

        // draw stuff
        cam.draw_grid();
        cam.plot_fn(&f);

        // finish frame
        next_frame().await
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
struct Point {
    x: f64,
    y: f64,
}

impl Point {
    fn new(x: f64, y: f64) -> Self {
        Point { x, y }
    }

    fn origin() -> Self {
        Point { x: 0.0, y: 0.0 }
    }
}

#[derive(Debug)]
struct CameraState {
    left: f64,
    right: f64,
    top: f64,
    bottom: f64,
}

impl CameraState {
    fn new(center: Point, width: f64, height: f64) -> Self {
        CameraState {
            left: center.x - width / 2.0,
            right: center.x + width / 2.0,
            top: center.y + height / 2.0,
            bottom: center.y - height / 2.0,
        }
    }

    fn width(&self) -> f64 {
        self.right - self.left
    }

    fn height(&self) -> f64 {
        self.top - self.bottom
    }

    fn center(&self) -> Point {
        Point::new((self.left + self.right) / 2.0, (self.top + self.bottom) / 2.0)
    }

    fn set_height(&mut self, height: f64) {
        let center = Point::new((self.left + self.right) / 2.0, (self.top + self.bottom) / 2.0);

        self.top = center.y + height / 2.0;
        self.bottom = center.y - height / 2.0;
    }

    fn move_to(&mut self, center: Point) {
        let width = self.width();
        let height = self.height();

        self.left = center.x - width / 2.0;
        self.right = center.x + width / 2.0;
        self.top = center.y + height / 2.0;
        self.bottom = center.y - height / 2.0;
    }

    fn draw_grid(&self) {
        // vertical lines at x = n for integers n inside the camera width

        let v_min = self.left.floor() as i32;
        let v_max = self.right.ceil() as i32;

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

        let h_min = self.bottom.floor() as i32;
        let h_max = self.top.ceil() as i32;

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

    fn euc_to_screen_x(&self, x: f64) -> f32 {
        // (x - left) + screen_width / cam_width
        ((x - self.left) * (screen_width() as f64) / self.width()) as f32 
    }

    fn euc_to_screen_y(&self, y: f64) -> f32 {
        // -(y - top) + screen_height / cam_height
        (-(y - self.top) * (screen_height() as f64) / self.height()) as f32
    }

    fn euc_to_screen(&self, p: Point) -> (f32, f32) {
        (self.euc_to_screen_x(p.x), self.euc_to_screen_y(p.y))
    }

    fn plot_fn(&self, f: &Function) {
        let samples = f.sample((self.left, self.right), 1000);

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
}
