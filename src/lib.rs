use wasm_bindgen::prelude::*;
use web_sys::console;
use wasm_bindgen::JsCast;

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

    sierpinski(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)], 2);

    //context.fill();

    console::log_1(&JsValue::from_str("Hello world!"));

    console::log_2(&"Color : %s ".into(),&context.fill_style().into());

    Ok(())
}

//----------------
fn draw_triangle(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3]) {
        let [top, left, right] = points;

        context.move_to(top.0, top.1);
        context.begin_path();
        context.line_to(left.0, left.1);
        context.line_to(right.0, right.1);
        context.line_to(top.0, top.1);
        context.close_path();
        context.stroke();
}


fn sierpinski(context: &web_sys::CanvasRenderingContext2d, points: [(f64, f64); 3], depth: u8) {
    //draw_triangle(&context, [(300.0, 0.0), (0.0, 600.0), (600.0, 600.0)]);
    draw_triangle(&context, points);
    
    let depth = depth - 1;
    if depth > 0 { 
        draw_triangle(&context, [(300.0, 0.0), (150.00, 300.0),  (450.0, 300.0)]);
        draw_triangle(&context, [(150.0, 300.0), (0.0, 600.0), (300.0,600.0)]);
        draw_triangle(&context, [(450.0, 300.0), (300.0, 600.0), (600.0, 600.0)]);
    }
}
