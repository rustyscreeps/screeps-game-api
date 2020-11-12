use crate::objects::{OwnedStructure, RoomObject, Structure, Store};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureTower`], which can heal, repair, or attack anywhere in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureTower;

    /// The [`Store`] of the tower, which contains energy which is consumed when it takes actions.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureTower) -> Store;

    /// Attack a [`Creep`], [`PowerCreep`], or [`Structure`] in the room, dealing damage depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.attack)
    #[wasm_bindgen(method)]
    pub fn attack(this: &StructureTower, target: &RoomObject) -> i8;

    /// Heal a [`Creep`] or [`PowerCreep`] in the room, adding hit points depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.heal)
    #[wasm_bindgen(method)]
    pub fn heal(this: &StructureTower, target: &RoomObject) -> i8;

    /// Repair a [`Structure`] in the room, adding hit points depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.repair)
    #[wasm_bindgen(method)]
    pub fn repair(this: &StructureTower, target: &Structure) -> i8;
}



// use crate::{
//     constants::ReturnCode,
//     objects::{Attackable, SharedCreepProperties, StructureProperties, StructureTower},
// };

// impl StructureTower {
//     pub fn attack<T>(&self, target: &T) -> ReturnCode
//     where
//         T: Attackable,
//     {
//         js_unwrap! { @{self.as_ref()}.attack( @{target.as_ref()} ) }
//     }

//     pub fn heal<T>(&self, target: &T) -> ReturnCode
//     where
//         T: SharedCreepProperties,
//     {
//         js_unwrap! { @{self.as_ref()}.heal( @{target.as_ref()} ) }
//     }

//     pub fn repair<T>(&self, target: &T) -> ReturnCode
//     where
//         T: StructureProperties,
//     {
//         js_unwrap! { @{self.as_ref()}.repair( @{target.as_ref()} ) }
//     }
// }
