use crate::objects::{RoomObject, Structure};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureRoad`], which allows creeps to move
    /// onto this position for half of the fatigue of moving onto a plains tile,
    /// as well as through terrain walls.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRoad)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructureRoad;

    /// The number of ticks until the road will decay, losing TODO CONSTANTS
    /// hits
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRoad.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureRoad) -> u32;
}
