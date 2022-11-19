use crate::{
    objects::{RoomObject, Structure},
    prelude::*,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureRoad`], which allows creeps to move
    /// onto this position for half of the fatigue of moving onto a plains tile,
    /// as well as through terrain walls.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRoad)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    #[derive(Clone, Debug)]
    pub type StructureRoad;

    /// The number of ticks until the road will decay, losing
    /// [`ROAD_DECAY_AMOUNT`] hits.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureRoad.ticksToDecay)
    ///
    /// [`ROAD_DECAY_AMOUNT`]: crate::constants::ROAD_DECAY_AMOUNT
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructureRoad) -> u32;
}

impl CanDecay for StructureRoad {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}
