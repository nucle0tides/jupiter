extern crate js_sys;
extern crate wasm_bindgen;
extern crate web_sys;

use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::console;

fn window() -> web_sys::Window {
    web_sys::window().expect("no global `window` exists")
}

fn document() -> web_sys::Document {
    window()
        .document()
        .expect("should have a document on window")
}

#[wasm_bindgen]
pub fn jupiter() {
    let file_input = document()
        .get_element_by_id("file-selector")
        .unwrap()
        .dyn_into::<web_sys::HtmlElement>()
        .unwrap()
        .dyn_into::<web_sys::HtmlInputElement>()
        .unwrap();

    console::log_1(&file_input.files().unwrap().get(0).into());
}
