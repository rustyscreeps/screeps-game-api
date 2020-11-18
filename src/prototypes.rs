//! Prototypes of Screeps object types, used for constructing some object types without using their default constructor.
use wasm_bindgen::prelude::*;
use js_sys::Object;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = prototype, js_namespace = RoomPosition)]
    pub static ROOM_POSITION_PROTOTYPE: Object;

    #[wasm_bindgen(js_name = prototype, js_namespace = ["PathFinder", "CostMatrix"])]
    pub static COST_MATRIX_PROTOTYPE: Object;
}
