use macroquad::prelude::*;

// -----------------------------------------------------------------------------

mod approx;
mod cam;
mod func;
mod num;
mod poly;

use cam::Camera;
use func::*;
use num::Point;
use poly::Polynomial;

// =============================================================================

#[macroquad::main("Graffing Program")]
async fn main() {
    // camera setup
    let mut cam = Camera::default();

    // function setup
    let four_x_squared = fn_mul(fn_const(4.0), fn_mul(X, X));

    let cool_fn: Function = fn_exp(fn_div(fn_const(-1.0), four_x_squared.clone())); // e^(-1/x^2)
    let f = fn_mul(cool_fn, fn_sin(four_x_squared)); // e^(-1/x^2) * sin(x^2)

    let interval = (-1.0, 1.0);

    let mut p = Polynomial::new_random_with_degree(12);

    loop {
        clear_background(WHITE);

        // this camera stuff could be handled inside the camera, but i want to keep the input handling here in case it needs to change.

        // update camera aspect ratio to match screen aspect ratio (in case of window rescale)
        cam.set_aspect_ratio((screen_width() / screen_height()) as f64);

        // move camera when screen is dragged
        if is_mouse_button_down(MouseButton::Left) {
            cam.move_by(mouse_delta_position());
        }

        // zoom camera when scrolling
        let y_scroll = mouse_wheel().1;
        if y_scroll != 0.0 {
            cam.zoom_by(y_scroll);
        }

        p = approx::compute_gradient_descent_step(&f, &p, interval, 0.2);

        // draw stuff
        cam.draw_grid();
        cam.draw_function(&f, RED);
        cam.draw_function(&p.to_function_of_x(), BLUE);
        // cam.draw_function(&p1, GREEN);
        // cam.draw_function(&p2, YELLOW);

        // finish frame
        next_frame().await
    }
}
