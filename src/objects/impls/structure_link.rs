use crate::{
    objects::{OwnedStructure, RoomObject, Structure, Store},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureLink`], which can hold energy and transfer it to other links within the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureLink;

    /// The number of ticks until the [`StructureLink`] can use [`StructureLink::transfer`] again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureLink) -> u32;

    /// The [`Store`] of the extension, which contains information about the amount of energy in it.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureLink) -> Store;

    /// Transfer energy from this [`StructureLink`] to another, losing [`LINK_LOSS_RATIO`] and incurring a cooldown of [`LINK_COOLDOWN`] per range to the target.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLink.transferEnergy)
    #[wasm_bindgen(method, js_name = transferEnergy)]
    pub fn transfer_energy(this: &StructureLink, target: &StructureLink) -> i8;
}

// use crate::{constants::ReturnCode, objects::StructureLink};

// impl StructureLink {
//     pub fn transfer_energy(&self, target: &StructureLink, amount: Option<u32>) -> ReturnCode {
//         match amount {
//             None => js_unwrap! {@{self.as_ref()}.transferEnergy(@{target.as_ref()})},
//             Some(amount) => {
//                 js_unwrap! {@{self.as_ref()}.transferEnergy(@{target.as_ref()}, @{amount})}
//             }
//         }
//     }
// }
