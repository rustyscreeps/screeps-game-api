use crate::{constants::ResourceType, objects::RoomObject, prelude::*};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Resource`] is an object representing resources that have been
    /// dropped and can be picked up.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Resource;

    /// Amount of resource this contains.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource.amount)
    #[wasm_bindgen(method, getter)]
    pub fn amount(this: &Resource) -> u32;

    /// Object ID of the resource, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Resource) -> JsString;

    /// The type of resource this contains.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Resource.resourceType)
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &Resource) -> ResourceType;
}

impl HasNativeId for Resource {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}
