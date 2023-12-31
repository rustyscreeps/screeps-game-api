use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    constants::{ErrorCode, StructureType},
    objects::RoomObject,
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// Parent class for all objects that represent a structure in the game
    /// world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Structure;

    #[wasm_bindgen(method, getter = hits)]
    fn hits_internal(this: &Structure) -> Option<u32>;

    #[wasm_bindgen(method, getter = hitsMax)]
    fn hits_max_internal(this: &Structure) -> Option<u32>;

    /// Object ID of the structure, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Structure) -> JsString;

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

    /// Set whether a notification email should be sent when the structure is
    /// attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.notifyWhenAttacked)
    #[wasm_bindgen(method, js_name = notifyWhenAttacked)]
    pub fn notify_when_attacked(this: &Structure, val: bool) -> i8;
}

impl Structure {
    /// Retrieve the current hits of this structure, or `0` if this structure is
    /// indestructible, such as a notice area border wall, portal, or room
    /// controller.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.hits)
    pub fn hits(&self) -> u32 {
        self.hits_internal().unwrap_or(0)
    }

    /// Retrieve the maximum hits of this structure, or `0` if this structure is
    /// indestructible, such as a notice area border wall, portal, or room
    /// controller.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.hitsMax)
    pub fn hits_max(&self) -> u32 {
        self.hits_max_internal().unwrap_or(0)
    }
}

impl<T> HasId for T
where
    T: AsRef<Structure> + JsCast,
{
    fn js_raw_id(&self) -> JsString {
        Structure::id_internal(self.as_ref())
    }
}

impl<T> HasHits for T
where
    T: AsRef<Structure>,
{
    fn hits(&self) -> u32 {
        Structure::hits(self.as_ref())
    }

    fn hits_max(&self) -> u32 {
        Structure::hits_max(self.as_ref())
    }
}

impl<T> StructureProperties for T
where
    T: AsRef<Structure>,
{
    fn structure_type(&self) -> StructureType {
        Structure::structure_type(self.as_ref())
    }

    fn destroy(&self) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(Structure::destroy(self.as_ref()))
    }

    fn is_active(&self) -> bool {
        Structure::is_active(self.as_ref())
    }

    fn notify_when_attacked(&self, val: bool) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(Structure::notify_when_attacked(self.as_ref(), val))
    }
}
