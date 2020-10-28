use crate::{
    constants::{ReturnCode, StructureType},
    objects::RoomObject,
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Parent class for all objects that represent a structure in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Structure;

    /// Remaining hit points for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.hits)
    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &Structure) -> u32;

    /// Maximum hit points for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.hitsMax)
    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hits_max(this: &Structure) -> u32;

    /// Maximum hit points for the structure.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Structure) -> JsString;

    /// The type of structure this is.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.structureType)
    #[wasm_bindgen(method, getter = structureType)]
    pub fn structure_type(this: &Structure) -> StructureType;

    /// Destroy the structure, if possible.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.destroy)
    #[wasm_bindgen(method)]
    pub fn destroy(this: &Structure) -> i8;

    /// Determine if the structure is active and can be used at the current RCL.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.isActive)
    #[wasm_bindgen(method, js_name = isActive)]
    pub fn is_active(this: &Structure) -> bool;

    /// Set whether a notification email should be sent when the structure is attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.notifyWhenAttacked)
    #[wasm_bindgen(method, js_name = notifyWhenAttacked)]
    pub fn notify_when_attacked(this: &Structure, val: bool) -> i8;
}

