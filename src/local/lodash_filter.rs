use crate::MarketResourceType;

use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object that is used to filter returned results from Screeps functions
    /// that accept such filters
    #[wasm_bindgen]
    pub type LodashFilter;
    /// Set the `resourceType` to be returned. Pre-filters return results to
    /// increase performance when used with [`get_all_orders`].
    #[wasm_bindgen(method, setter = resourceType)]
    pub fn resource_type(this: &LodashFilter, resource: MarketResourceType);

    // todo add more setters for use with Room.find()
}

impl LodashFilter {
    pub fn new() -> LodashFilter {
        Object::new().unchecked_into()
    }
}

impl Default for LodashFilter {
    fn default() -> Self {
        Self::new()
    }
}
