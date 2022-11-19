use crate::objects::{OwnedStructure, RoomObject, Spawning, Structure};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureInvaderCore`], which is at the
    /// center of NPC strongholds, as well as reserving neutral rooms.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureInvaderCore)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureInvaderCore;

    /// The level of the [`StructureInvaderCore`]; 0 is a lesser invader core
    /// that simply reserves rooms, while levels 1-5 are strongholds which
    /// defend themselves.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureInvaderCore.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &StructureInvaderCore) -> u8;

    /// The number of ticks until the [`StructureInvaderCore`] is fully deployed
    /// and can be attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureInvaderCore.ticksToDeploy)
    #[wasm_bindgen(method, getter = ticksToDeploy)]
    pub fn ticks_to_deploy(this: &StructureInvaderCore) -> u32;

    /// Information about the spawning creep, if one is currently being spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureInvaderCore.spawning)
    #[wasm_bindgen(method, getter)]
    pub fn spawning(this: &StructureInvaderCore) -> Option<Spawning>;
}
