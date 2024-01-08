use wasm_bindgen::prelude::*;

use crate::{
    objects::{OwnedStructure, RoomObject, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureKeeperLair`], which regularly spawns
    /// creeps to defend nearby resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureKeeperLair)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureKeeperLair;

    /// The number of ticks until the [`StructureKeeperLair`] will spawn a new
    /// creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureKeeperLair.ticksToSpawn)
    #[wasm_bindgen(method, getter = ticksToSpawn)]
    pub fn ticks_to_spawn(this: &StructureKeeperLair) -> u32;
}

impl Attackable for StructureKeeperLair {}
