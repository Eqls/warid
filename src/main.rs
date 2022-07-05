// use serde_json::json;
// use serde_json::{Result, Value};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

const ROW_HEIGHT: f64 = 30.0;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}

macro_rules! console_log {
    // Note that this is using the `log` function imported above during
    // `bare_bones`
    ($($t:tt)*) => (log(&format_args!($($t)*).to_string()))
}

fn draw(ctx: &web_sys::CanvasRenderingContext2d, start_x: i32, start_y: i32) {
    ctx.clear_rect(0.0, 0.0, 500.0, 500.0);
    ctx.begin_path();
    let limit = 500/ROW_HEIGHT as i32;
    let mut count = 0;
    for index in 0..1000 {
        if count > limit {
            break;
        }
        let height = (index as i32 + 1) * ROW_HEIGHT as i32;
        if start_y <= height || start_y == 1  {
            draw_row(&ctx, &count, index, start_y);
            count += 1;
        }
    }

    ctx.stroke();
}

fn draw_row(ctx: &web_sys::CanvasRenderingContext2d, i: &i32, item_index: usize, top_scroll: i32) {
    let height = i.to_owned() * ROW_HEIGHT as i32;
    let offset = top_scroll - height;
    console_log!("{:?}", &offset);
    ctx.stroke_rect(10.0, height as f64 - offset as f64, 100.0, ROW_HEIGHT);
    ctx.fill_text(
        String::as_str(&item_index.to_string()),
        40.0,
        height as f64 - offset as f64 + ROW_HEIGHT/2.0,
    )
    .unwrap();
}

fn scroll_handler(document: &web_sys::Document, ctx: web_sys::CanvasRenderingContext2d) {
    let scroller = document.get_element_by_id("scroller").unwrap();
    let scroller_clone = scroller.clone();
    let handle_scroll = Closure::wrap(Box::new(move |event: web_sys::MouseScrollEvent| {
        draw(&ctx, event.client_x(), scroller_clone.scroll_top());
    }) as Box<dyn FnMut(_)>);

    scroller
        .add_event_listener_with_callback("wheel", handle_scroll.as_ref().unchecked_ref())
        .unwrap();

    handle_scroll.forget();
}

// #[wasm_bindgen(start)]
pub fn main() {
    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    scroll_handler(&document, ctx.clone());
    draw(&ctx, 1, 1);
}
