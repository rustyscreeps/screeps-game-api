use wasm_bindgen::prelude::*;

use crate::{
    constants::ErrorCode,
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePowerSpawn`], which can process
    /// power to contribute to your GPL as well as renewing power creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructurePowerSpawn;

    /// The [`Store`] of the power spawn, which can contain power and energy.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructurePowerSpawn) -> Store;

    /// Process power, consuming 1 power and [`POWER_SPAWN_ENERGY_RATIO`] energy
    /// and increasing your GPL by one point.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn.processPower)
    ///
    /// [`POWER_SPAWN_ENERGY_RATIO`]:
    /// crate::constants::numbers::POWER_SPAWN_ENERGY_RATIO
    #[wasm_bindgen(method, js_name = processPower)]
    fn process_power_internal(this: &StructurePowerSpawn) -> i8;
}

impl StructurePowerSpawn {
    /// Process power, consuming 1 power and [`POWER_SPAWN_ENERGY_RATIO`] energy
    /// and increasing your GPL by one point.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePowerSpawn.processPower)
    ///
    /// [`POWER_SPAWN_ENERGY_RATIO`]: crate::constants::POWER_SPAWN_ENERGY_RATIO
    pub fn process_power(&self) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.process_power_internal())
    }
}

impl HasStore for StructurePowerSpawn {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl Attackable for StructurePowerSpawn {}
impl Dismantleable for StructurePowerSpawn {}
impl Repairable for StructurePowerSpawn {}
impl Transferable for StructurePowerSpawn {}
impl Withdrawable for StructurePowerSpawn {}
