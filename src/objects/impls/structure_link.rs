use wasm_bindgen::prelude::*;

use crate::{
    constants::ReturnCode,
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureLink`], which can hold energy and
    /// transfer it to other links within the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureLink;

    /// The number of ticks until the [`StructureLink`] can use
    /// [`StructureLink::transfer`] again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureLink) -> u32;

    /// The [`Store`] of the extension, which contains information about the
    /// amount of energy in it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureLink) -> Store;

    /// Transfer energy from this [`StructureLink`] to another, losing
    /// [`LINK_LOSS_RATIO`] percent of the energt and incurring a cooldown of
    /// [`LINK_COOLDOWN`] tick per range to the target.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.transferEnergy)
    ///
    /// [`LINK_LOSS_RATIO`]: crate::constants::LINK_LOSS_RATIO
    /// [`LINK_COOLDOWN`]: crate::constants::LINK_COOLDOWN
    #[wasm_bindgen(method, js_name = transferEnergy)]
    pub fn transfer_energy(
        this: &StructureLink,
        target: &StructureLink,
        amount: Option<u32>,
    ) -> ReturnCode;
}

impl HasCooldown for StructureLink {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasStore for StructureLink {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
