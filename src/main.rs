use approx::compute_legendre_approx;
use integration::{int_inner_product, IntMethod};
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets, Skin};

// -----------------------------------------------------------------------------

mod approx;
mod cam;
mod func;
mod integration;
mod polynomial;
mod util;

use cam::Camera;
use func::*;
use polynomial::Polynomial;
use util::Point;

// =============================================================================

#[macroquad::main("Graffing Program")]
async fn main() {
    // style setup
    let custom_skin = load_skin();
    root_ui().push_skin(&custom_skin);

    // camera setup
    let mut cam = Camera::default();

    // function setup

    // f1(x) = e^(-x^2)
    let f1 = fn_exp(fn_neg(fn_powi(X, 2)));

    // f2(x) = 1 - x
    let f2 = fn_sub(fn_const(1.0), X);

    let f = fn_mul(f1.clone(), f2.clone());

    let p = compute_legendre_approx(&f, IntMethod::CompositeTrapezoidal(10000));


    // print functions
    // println!("f(x) = {}", f);

    // polynomial setup
    // let mut p = Polynomial::new_random_with_degree(16);
    // let mut coeffs = p.coefficients.clone();

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

        // computations --------------------------------------------------------

        // approx::compute_gradient_descent_step(&f, &mut coeffs, (-1.0, 1.0), 1000, 0.1);
        // p.coefficients.clone_from(&coeffs);

        // drawing -------------------------------------------------------------
        cam.draw_grid();
        cam.draw_function(&f1, BLUE);
        cam.draw_function(&f2, PURPLE);
        cam.draw_function(&f, RED);
        cam.draw_function(&p, GREEN);
        // cam.draw_function(&p1, GREEN);
        // cam.draw_function(&p2, YELLOW);

        // ui ------------------------------------------------------------------
        root_ui().label(None, "hello megaui");
        if root_ui().button(None, "Push me") {
            println!("pushed");
        }

        // finish frame --------------------------------------------------------
        next_frame().await
    }
}

fn load_skin() -> Skin {
    let font_bytes = include_bytes!(".././assets/cmunrm.ttf");

    let label_style = root_ui()
        .style_builder()
        .font(font_bytes)
        .unwrap()
        .font_size(20)
        .build();

    let button_style = root_ui()
        .style_builder()
        .font(font_bytes)
        .unwrap()
        .text_color(Color::from_rgba(180, 180, 100, 255))
        .font_size(40)
        .build();

    let editbox_style = root_ui()
        .style_builder()
        .background_margin(RectOffset::new(0., 0., 0., 0.))
        .font(font_bytes)
        .unwrap()
        .text_color(Color::from_rgba(120, 120, 120, 255))
        .color_selected(Color::from_rgba(190, 190, 190, 255))
        .font_size(50)
        .build();

    Skin {
        editbox_style,
        button_style,
        label_style,
        ..root_ui().default_skin()
    }
}
