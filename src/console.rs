use js_sys::JsString;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console, js_name = addVisual)]
    pub fn add_visual(room_name: Option<&JsString>, visual: &JsValue);
}