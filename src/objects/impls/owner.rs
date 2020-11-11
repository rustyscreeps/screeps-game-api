use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Object with owner info for an owned game object.
    #[wasm_bindgen]
    pub type Owner;

    /// The name of the player that owns this structure as a [`JsString`].
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &Owner) -> JsString;
}
