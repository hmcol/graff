use macroquad::prelude::*;

// -----------------------------------------------------------------------------

mod cam;
mod func;
mod num;

use cam::Camera;
use func::*;
use num::Point;

// =============================================================================

#[macroquad::main("Graffing Program")]
async fn main() {
    let cool_fn: Function = fn_exp(fn_div(fn_const(-1.0), fn_mul(X, X)));
    let f = fn_mul(cool_fn, fn_sin(fn_mul(X, X)));

    let mut cam = Camera::new(Point::origin(), 20.0, 10.0);

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
        cam.draw_function(&f);

        // finish frame
        next_frame().await
    }
}
