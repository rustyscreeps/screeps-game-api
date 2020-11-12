use crate::objects::{RoomObject, Structure, Store};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Ruin`], which represents a destroyed structure and can have resources withdrawn from it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin)
    #[wasm_bindgen(extends = RoomObject)]
    pub type Ruin;

    /// The tick that the structure was destroyed
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.destroyTime)
    #[wasm_bindgen(method, getter = destroyTime)]
    pub fn destroy_time(this: &Ruin) -> u32;

    /// Object ID of the ruin, which can be used to efficiently fetch a fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.id)
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &Ruin) -> JsString;

    /// The [`Store`] of the ruin, which contains any resources in the ruin.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &Ruin) -> Store;

    /// The destroyed [`Structure`] that this ruin represents. Note that this object is not fully safe to use as a [`Structure`], missing critical properties such as position; it's only safe to access basic information about the structure on this object, like the structure type, owner name, and id.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.structure)
    #[wasm_bindgen(method, getter)]
    pub fn structure(this: &Ruin) -> Structure;

    /// The number of ticks until this ruin disappears.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Ruin.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &Ruin) -> u32;
}


// use crate::{
//     constants::StructureType,
//     objects::{RawObjectId, Ruin},
//     traits::TryInto,
// };

// simple_accessors! {
//     impl Ruin {
//         pub fn destroy_time() -> u32 = destroyTime;
//     }
// }

// impl Ruin {
//     /// Ruin.structure doesn't return complete object data, so instead of
//     /// implementing that directly, this function exposes relevant
//     /// properties of the ruin's structure directly in a tuple of the type,
//     /// id, and owner
//     pub fn structure_info(&self) -> (StructureType, RawObjectId, Option<String>) {
//         (
//             js_unwrap!(__structure_type_str_to_num(@{self.as_ref()}.structure.structureType)),
//             RawObjectId::from_packed_js_val(
//                 js_unwrap!(object_id_to_packed(@{self.as_ref()}.structure.id)),
//             )
//             .expect(
//                 "expected ruin structure's JavaScript id to be a 12-byte number encoded in hex",
//             ),
//             (js! {
//                 var self = @{self.as_ref()};
//                 if (self.structure.owner) {
//                     return self.structure.owner.username;
//                 } else {
//                     return null;
//                 }
//             })
//             .try_into()
//             .expect("expected ruin structure's owner.username to be a string"),
//         )
//     }
// }
