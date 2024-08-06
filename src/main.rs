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
    let cool_fn: Function = fn_exp(fn_div(fn_const(-1.0), fn_mul(X, X))); // e^(-1/x^2)
    let f = fn_mul(cool_fn, fn_sin(fn_mul(X, X))); // e^(-1/x^2) * sin(x^2)

    let mut cam = Camera::default();

    loop {
        clear_background(WHITE);

        // update camera aspect ratio to match screen aspect ratio (in case of window rescale)
        cam.set_aspect_ratio((screen_width() / screen_height()) as f64);

        // move camera when screen is dragged
        if is_mouse_button_down(MouseButton::Left) {
            cam.move_by(mouse_delta_position());
        }

        // zoom camera when scrolling
        let y_scroll = mouse_wheel().1;
        if y_scroll < 0.0 {
            cam.zoom_by(1.5);
        } else if y_scroll > 0.0 {
            cam.zoom_by(1.0 / 1.5);
        }
        

        // draw stuff
        cam.draw_grid();
        cam.draw_function(&f);

        // finish frame
        next_frame().await
    }
}
