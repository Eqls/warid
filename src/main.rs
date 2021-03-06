// use serde_json::json;
// use serde_json::{Result, Value};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

const ROW_HEIGHT: f64 = 30.0;
const CANVAS_HEIGHT: i32 = 500;
const CANVAS_WIDTH: i32 = 500;

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

fn draw(ctx: &web_sys::CanvasRenderingContext2d, start_x: i32, start_y: i32, data: &Vec<i32>) {
    ctx.clear_rect(0.0, 0.0, CANVAS_WIDTH as f64, CANVAS_HEIGHT as f64);
    ctx.begin_path();
    let start_index = (start_y as f64 / ROW_HEIGHT).floor() as i32;
    let end_index = (start_y as f64 / ROW_HEIGHT + CANVAS_HEIGHT as f64 / ROW_HEIGHT).ceil() as i32;

    for index in start_index..end_index {
        draw_row(&ctx, index, start_y);
    }

    ctx.stroke();
    console_log!("draw: end");
}

fn draw_row(ctx: &web_sys::CanvasRenderingContext2d, index: i32, top_scroll: i32) {
    let height = index * ROW_HEIGHT as i32;
    let offset = height - top_scroll;
    let ypos = height as f64 - top_scroll as f64;

    ctx.stroke_rect(10.0, ypos, 100.0, ROW_HEIGHT);
    ctx.fill_text(
        String::as_str(&index.to_string()),
        50.0,
        ypos + ROW_HEIGHT / 2.0,
    )
    .unwrap();
}

fn scroll_handler(
    document: &web_sys::Document,
    ctx: web_sys::CanvasRenderingContext2d,
    data: Vec<i32>,
) {
    let scroller = document.get_element_by_id("scroller").unwrap();
    let scroller_inner = document.get_element_by_id("scroller-inner").unwrap();
    let innter = document
        .get_element_by_id("scroller-inner")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
    innter
        .style()
        .set_property(
            "height",
            (((data.len() * ROW_HEIGHT as usize) as usize).to_string() + "px").as_str(),
        )
        .unwrap();
    let inner_block_count = ((data.len() * ROW_HEIGHT as usize) as usize) / 100000;
    let item = document
        .get_element_by_id("scroller-item")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap();
        item
        .style()
        .set_property(
            "height",
            "100000px",
        )
        .unwrap();

        if inner_block_count > 1 {
            for i in 1..inner_block_count {
                let t = item.clone_node().unwrap();
                scroller_inner.append_child(&t).unwrap();
            }
        }
    let scroller_clone = scroller.clone();
    let handle_scroll = Closure::wrap(Box::new(move |event: web_sys::MouseScrollEvent| {
        draw(&ctx, event.client_x(), scroller_clone.scroll_top(), &data);
    }) as Box<dyn FnMut(_)>);

    scroller
        .add_event_listener_with_callback("scroll", handle_scroll.as_ref().unchecked_ref())
        .unwrap();

    handle_scroll.forget();
}

// #[wasm_bindgen(start)]
pub fn main() {
    let document = web_sys::window().unwrap().document().unwrap();
    let ratio = web_sys::window().unwrap().device_pixel_ratio();
    let canvas = document.get_element_by_id("canvas").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();
    // TODO: HDPi fix attempt
    // canvas.set_width((CANVAS_WIDTH as f64 * ratio) as u32);
    // canvas.set_height((CANVAS_HEIGHT as f64 * ratio) as u32);
    // canvas
    //     .style()
    //     .set_property("width", (CANVAS_WIDTH.to_string() + "px").as_str())
    //     .unwrap();
    // canvas
    //     .style()
    //     .set_property("height", (CANVAS_HEIGHT.to_string() + "px").as_str())
    //     .unwrap();

    let ctx = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();
    let data: Vec<i32> = (0..500000).collect();

    scroll_handler(&document, ctx.clone(), data.clone());
    draw(&ctx, 1, 1, &data);
}
