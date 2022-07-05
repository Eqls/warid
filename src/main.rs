// use serde_json::json;
// use serde_json::{Result, Value};
use std::f64;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys;

struct Data {
    _id: &'static str,
    company: &'static str,
    email: &'static str,
    phone: &'static str,
    address: &'static str,
    registered: &'static str,
    latitude: f32,
    longitude: f32,
}

const DATA: [Data; 28] = [
    Data {
        _id: "62b1e7c9611e84a1e140045b",
        company: "XYQAG",
        email: "desireescott@xyqag.com",
        phone: "+1 (800) 431-3187",
        address: "274 Strickland Avenue, Websterville, Arkansas, 7427",
        registered: "2020-11-24T12:35:50 -02:00",
        latitude: 20.176423,
        longitude: -164.679918,
    },
    Data {
        _id: "62b1e7c9e0884d02dfb22433",
        company: "ISOTRACK",
        email: "desireescott@isotrack.com",
        phone: "+1 (966) 499-2944",
        address: "808 Hegeman Avenue, Castleton, Colorado, 4102",
        registered: "2021-03-26T07:56:37 -02:00",
        latitude: 7.905266,
        longitude: -126.352018,
    },
    Data {
        _id: "62b1e7c9bfb9895baea98348",
        company: "LOCAZONE",
        email: "desireescott@locazone.com",
        phone: "+1 (883) 529-2480",
        address: "301 Glenwood Road, Geyserville, Virginia, 2314",
        registered: "2019-03-23T03:09:18 -02:00",
        latitude: -23.32131,
        longitude: 0.968465,
    },
    Data {
        _id: "62b1e7c9d3f4ab9a9c1d176b",
        company: "ECLIPSENT",
        email: "desireescott@eclipsent.com",
        phone: "+1 (999) 575-3353",
        address: "272 Livonia Avenue, Chesterfield, New Hampshire, 8538",
        registered: "2019-01-12T01:13:47 -02:00",
        latitude: -23.004942,
        longitude: -102.960328,
    },
    Data {
        _id: "62b1e7c9d26d15956f36d8b0",
        company: "CEDWARD",
        email: "desireescott@cedward.com",
        phone: "+1 (911) 409-2872",
        address: "677 Dover Street, Carlton, Hawaii, 8358",
        registered: "2016-04-06T01:36:05 -03:00",
        latitude: -38.282586,
        longitude: -107.162395,
    },
    Data {
        _id: "62b1e7c910529e4013d2acc3",
        company: "MARVANE",
        email: "desireescott@marvane.com",
        phone: "+1 (910) 471-2408",
        address: "382 Hutchinson Court, Washington, California, 3311",
        registered: "2020-09-03T08:16:01 -03:00",
        latitude: 39.851635,
        longitude: 89.610976,
    },
    Data {
        _id: "62b1e7c9be68b18bf972b068",
        company: "STRALUM",
        email: "desireescott@stralum.com",
        phone: "+1 (922) 508-2710",
        address: "273 Celeste Court, Olney, Maine, 7115",
        registered: "2021-10-02T10:05:15 -03:00",
        latitude: 33.483145,
        longitude: -48.67096,
    },
    Data {
        _id: "62b1e7c9611e84a1e140045b",
        company: "XYQAG",
        email: "desireescott@xyqag.com",
        phone: "+1 (800) 431-3187",
        address: "274 Strickland Avenue, Websterville, Arkansas, 7427",
        registered: "2020-11-24T12:35:50 -02:00",
        latitude: 20.176423,
        longitude: -164.679918,
    },
    Data {
        _id: "62b1e7c9e0884d02dfb22433",
        company: "ISOTRACK",
        email: "desireescott@isotrack.com",
        phone: "+1 (966) 499-2944",
        address: "808 Hegeman Avenue, Castleton, Colorado, 4102",
        registered: "2021-03-26T07:56:37 -02:00",
        latitude: 7.905266,
        longitude: -126.352018,
    },
    Data {
        _id: "62b1e7c9bfb9895baea98348",
        company: "LOCAZONE",
        email: "desireescott@locazone.com",
        phone: "+1 (883) 529-2480",
        address: "301 Glenwood Road, Geyserville, Virginia, 2314",
        registered: "2019-03-23T03:09:18 -02:00",
        latitude: -23.32131,
        longitude: 0.968465,
    },
    Data {
        _id: "62b1e7c9d3f4ab9a9c1d176b",
        company: "ECLIPSENT",
        email: "desireescott@eclipsent.com",
        phone: "+1 (999) 575-3353",
        address: "272 Livonia Avenue, Chesterfield, New Hampshire, 8538",
        registered: "2019-01-12T01:13:47 -02:00",
        latitude: -23.004942,
        longitude: -102.960328,
    },
    Data {
        _id: "62b1e7c9d26d15956f36d8b0",
        company: "CEDWARD",
        email: "desireescott@cedward.com",
        phone: "+1 (911) 409-2872",
        address: "677 Dover Street, Carlton, Hawaii, 8358",
        registered: "2016-04-06T01:36:05 -03:00",
        latitude: -38.282586,
        longitude: -107.162395,
    },
    Data {
        _id: "62b1e7c910529e4013d2acc3",
        company: "MARVANE",
        email: "desireescott@marvane.com",
        phone: "+1 (910) 471-2408",
        address: "382 Hutchinson Court, Washington, California, 3311",
        registered: "2020-09-03T08:16:01 -03:00",
        latitude: 39.851635,
        longitude: 89.610976,
    },
    Data {
        _id: "62b1e7c9be68b18bf972b068",
        company: "STRALUM",
        email: "desireescott@stralum.com",
        phone: "+1 (922) 508-2710",
        address: "273 Celeste Court, Olney, Maine, 7115",
        registered: "2021-10-02T10:05:15 -03:00",
        latitude: 33.483145,
        longitude: -48.67096,
    },
    Data {
        _id: "62b1e7c9611e84a1e140045b",
        company: "XYQAG",
        email: "desireescott@xyqag.com",
        phone: "+1 (800) 431-3187",
        address: "274 Strickland Avenue, Websterville, Arkansas, 7427",
        registered: "2020-11-24T12:35:50 -02:00",
        latitude: 20.176423,
        longitude: -164.679918,
    },
    Data {
        _id: "62b1e7c9e0884d02dfb22433",
        company: "ISOTRACK",
        email: "desireescott@isotrack.com",
        phone: "+1 (966) 499-2944",
        address: "808 Hegeman Avenue, Castleton, Colorado, 4102",
        registered: "2021-03-26T07:56:37 -02:00",
        latitude: 7.905266,
        longitude: -126.352018,
    },
    Data {
        _id: "62b1e7c9bfb9895baea98348",
        company: "LOCAZONE",
        email: "desireescott@locazone.com",
        phone: "+1 (883) 529-2480",
        address: "301 Glenwood Road, Geyserville, Virginia, 2314",
        registered: "2019-03-23T03:09:18 -02:00",
        latitude: -23.32131,
        longitude: 0.968465,
    },
    Data {
        _id: "62b1e7c9d3f4ab9a9c1d176b",
        company: "ECLIPSENT",
        email: "desireescott@eclipsent.com",
        phone: "+1 (999) 575-3353",
        address: "272 Livonia Avenue, Chesterfield, New Hampshire, 8538",
        registered: "2019-01-12T01:13:47 -02:00",
        latitude: -23.004942,
        longitude: -102.960328,
    },
    Data {
        _id: "62b1e7c9d26d15956f36d8b0",
        company: "CEDWARD",
        email: "desireescott@cedward.com",
        phone: "+1 (911) 409-2872",
        address: "677 Dover Street, Carlton, Hawaii, 8358",
        registered: "2016-04-06T01:36:05 -03:00",
        latitude: -38.282586,
        longitude: -107.162395,
    },
    Data {
        _id: "62b1e7c910529e4013d2acc3",
        company: "MARVANE",
        email: "desireescott@marvane.com",
        phone: "+1 (910) 471-2408",
        address: "382 Hutchinson Court, Washington, California, 3311",
        registered: "2020-09-03T08:16:01 -03:00",
        latitude: 39.851635,
        longitude: 89.610976,
    },
    Data {
        _id: "62b1e7c9be68b18bf972b068",
        company: "STRALUM",
        email: "desireescott@stralum.com",
        phone: "+1 (922) 508-2710",
        address: "273 Celeste Court, Olney, Maine, 7115",
        registered: "2021-10-02T10:05:15 -03:00",
        latitude: 33.483145,
        longitude: -48.67096,
    },
    Data {
        _id: "62b1e7c9611e84a1e140045b",
        company: "XYQAG",
        email: "desireescott@xyqag.com",
        phone: "+1 (800) 431-3187",
        address: "274 Strickland Avenue, Websterville, Arkansas, 7427",
        registered: "2020-11-24T12:35:50 -02:00",
        latitude: 20.176423,
        longitude: -164.679918,
    },
    Data {
        _id: "62b1e7c9e0884d02dfb22433",
        company: "ISOTRACK",
        email: "desireescott@isotrack.com",
        phone: "+1 (966) 499-2944",
        address: "808 Hegeman Avenue, Castleton, Colorado, 4102",
        registered: "2021-03-26T07:56:37 -02:00",
        latitude: 7.905266,
        longitude: -126.352018,
    },
    Data {
        _id: "62b1e7c9bfb9895baea98348",
        company: "LOCAZONE",
        email: "desireescott@locazone.com",
        phone: "+1 (883) 529-2480",
        address: "301 Glenwood Road, Geyserville, Virginia, 2314",
        registered: "2019-03-23T03:09:18 -02:00",
        latitude: -23.32131,
        longitude: 0.968465,
    },
    Data {
        _id: "62b1e7c9d3f4ab9a9c1d176b",
        company: "ECLIPSENT",
        email: "desireescott@eclipsent.com",
        phone: "+1 (999) 575-3353",
        address: "272 Livonia Avenue, Chesterfield, New Hampshire, 8538",
        registered: "2019-01-12T01:13:47 -02:00",
        latitude: -23.004942,
        longitude: -102.960328,
    },
    Data {
        _id: "62b1e7c9d26d15956f36d8b0",
        company: "CEDWARD",
        email: "desireescott@cedward.com",
        phone: "+1 (911) 409-2872",
        address: "677 Dover Street, Carlton, Hawaii, 8358",
        registered: "2016-04-06T01:36:05 -03:00",
        latitude: -38.282586,
        longitude: -107.162395,
    },
    Data {
        _id: "62b1e7c910529e4013d2acc3",
        company: "MARVANE",
        email: "desireescott@marvane.com",
        phone: "+1 (910) 471-2408",
        address: "382 Hutchinson Court, Washington, California, 3311",
        registered: "2020-09-03T08:16:01 -03:00",
        latitude: 39.851635,
        longitude: 89.610976,
    },
    Data {
        _id: "62b1e7c9be68b18bf972b068",
        company: "STRALUM",
        email: "desireescott@stralum.com",
        phone: "+1 (922) 508-2710",
        address: "273 Celeste Court, Olney, Maine, 7115",
        registered: "2021-10-02T10:05:15 -03:00",
        latitude: 33.483145,
        longitude: -48.67096,
    },
];

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

fn draw(ctx: &web_sys::CanvasRenderingContext2d, startX: i32, startY: i32) {
    ctx.begin_path();
    ctx.clear_rect(0.0, 0.0, 500.0, 500.0);
    for (index, item) in DATA.iter().enumerate() {
        if startY >= (index as i32 + 1) * ROW_HEIGHT as i32 || startY == 1 {
            draw_row(&ctx, &item, index);
        }
    }

    ctx.stroke();
}

fn draw_row(ctx: &web_sys::CanvasRenderingContext2d, item: &Data, i: usize) {
    ctx.stroke_rect(10.0, i as f64 * ROW_HEIGHT+2.0, 100.0, ROW_HEIGHT);
    ctx.fill_text( String::as_str(&i.to_string()), 40.0, i as f64 * ROW_HEIGHT - ROW_HEIGHT/2.0).unwrap();
}

fn scroll_handler(document: &web_sys::Document, ctx: web_sys::CanvasRenderingContext2d) {
    let scroller = document.get_element_by_id("scroller").unwrap();
    let handle_scroll = Closure::wrap(Box::new(move |event: web_sys::MouseScrollEvent| {
        draw(&ctx, event.client_x(), event.client_y());
        console_log!("{:?}", event.client_y());
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
