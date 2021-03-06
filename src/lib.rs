use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;

use rand::{thread_rng, Rng};

// When attached to a pub function this attribute will configure the start 
// section of the wasm executable to be emitted, executing the tagged function 
// as soon as the wasm module is instantiated.
#[wasm_bindgen(start)]
pub fn main_js() -> Result<(), JsValue> {
    console_error_panic_hook::set_once();

    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();
    let canvas = document
        .get_element_by_id("canvas")
        .unwrap()
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .unwrap();

    //  call the dyn_into function to cast it into HtmlCanvasElement. 
    //This method was brought into scope with the `use wasm_bindgen::JsCast` declaration.
/*
    let context = canvas.get_context("2d")
        .unwrap().unwrap()  //  get_context returns a Result<Option<Object>>
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
*/
    let context = canvas
        .get_context("2d")? //  get_context returns a Result<Option<Object>>
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()?;

    sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], (0, 255, 0), 5);

    //context.fill();

    console::log_1(&JsValue::from_str("Hello world!"));

    console::log_2(&"Color : %s ".into(),&context.fill_style().into());

    Ok(())
}

//----------------
fn draw_triangle(   context: &web_sys::CanvasRenderingContext2d, 
                    points: [(f64, f64); 3], 
                    color: (u8, u8, u8) ) {
        let [top, left, right] = points;

        // https://github.com/rustwasm/wasm-bindgen/issues/1705
        // https://docs.rs/wasm-bindgen/latest/wasm_bindgen/struct.JsValue.html#method.from_str
        let color_str = format!("rgb({}, {}, {})", color.0, color.1, color.2);
        context.set_fill_style(&wasm_bindgen::JsValue::from_str(&color_str));

        context.move_to(top.0, top.1);

        context.begin_path();
        context.line_to(left.0, left.1);
        context.line_to(right.0, right.1);
        context.line_to(top.0, top.1);
        context.close_path();

        context.stroke();
        context.fill()

}

fn midpoint(point_1: (f64, f64), point_2: (f64, f64)) -> (f64, f64) {

    ((point_1.0 + point_2.0) / 2.0, (point_1.1 + point_2.1) / 2.0)
}

fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], color: (u8, u8, u8), depth: u8) {
    //draw_triangle(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);
    draw_triangle(&context, points, color);
    
    let depth = depth - 1;
    let [top, left, right] = points;

    if depth > 0 {
        let mut rng = thread_rng();

        let next_color = (
            rng.gen_range(0..255),
            rng.gen_range(0..255),
            rng.gen_range(0..255),
        );

        let left_middle = midpoint(top, left);
        let right_middle = midpoint(top, right);
        let bottom_middle = midpoint(left, right);

        sierpinski(&context, [top, left_middle, right_middle], next_color, depth);
        sierpinski(&context, [left_middle, left, bottom_middle], next_color, depth);
        sierpinski(&context, [right_middle, bottom_middle, right], next_color, depth);
    }    


}
