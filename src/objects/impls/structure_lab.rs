use crate::{
    constants::ResourceType,
    objects::{Creep, OwnedStructure, RoomObject, Store, Structure},
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureLab`], which can be used to create
    /// mineral compounds.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureLab;

    /// The number of ticks until the [`StructureLab`] can use
    /// [`StructureLab::run_reaction`] or [`StructureLab::unboost_creep`] again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureLab) -> u32;

    /// The [`Store`] of the lab, which can contain energy and one type of
    /// resource at a time.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureLab) -> Store;

    /// Get the type of mineral currently contained in the lab, which can only
    /// hold one type at a time
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.mineralType)
    #[wasm_bindgen(method, getter = mineralType)]
    pub fn mineral_type(this: &StructureLab) -> Option<ResourceType>;

    /// Boost a [`Creep`] in melee range, consuming [`LAB_BOOST_ENERGY`] energy
    /// and [`LAB_BOOST_MINERAL`] of the boost compound from the
    /// [`StructureLab::store`] per boosted body part.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.boostCreep)
    ///
    /// [`LAB_BOOST_ENERGY`]: crate::constants::numbers::LAB_BOOST_ENERGY
    /// [`LAB_BOOST_MINERAL`]: crate::constants::numbers::LAB_BOOST_MINERAL
    #[wasm_bindgen(method, js_name = boostCreep)]
    pub fn boost_creep(this: &StructureLab, creep: &Creep, body_part_count: Option<u32>) -> i8;

    /// Reverse a reaction, splitting the compound in this [`StructureLab`] into
    /// its components in two other labs.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.reverseReaction)
    #[wasm_bindgen(method, js_name = reverseReaction)]
    pub fn reverse_reaction(this: &StructureLab, lab1: &StructureLab, lab2: &StructureLab) -> i8;

    /// Run a reaction, combining components from two other [`StructureLab`]s
    /// into a new compound in this lab.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.runReaction)
    #[wasm_bindgen(method, js_name = runReaction)]
    pub fn run_reaction(this: &StructureLab, lab1: &StructureLab, lab2: &StructureLab) -> i8;

    /// Unboost a [`Creep`], removing all boosts from its body and dropping
    /// [`LAB_UNBOOST_MINERAL`] per body part on the ground, with a cooldown
    /// equal to the total time to produce the removed boosts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.unboostCreep)
    ///
    /// [`LAB_UNBOOST_ENERGY`]: crate::constants::numbers::LAB_UNBOOST_ENERGY
    #[wasm_bindgen(method, js_name = unboostCreep)]
    pub fn unboost_creep(this: &StructureLab, creep: &Creep) -> i8;
}

// use crate::{
//     constants::{ResourceType, ReturnCode},
//     objects::{Creep, StructureLab},
// };

// impl StructureLab {
//     pub fn mineral_type(&self) -> ResourceType {
//         js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.mineralType))
//     }

//     pub fn boost_creep(&self, creep: &Creep, body_part_count: Option<u32>) -> ReturnCode {
//         match body_part_count {
//             None => js_unwrap! {@{self.as_ref()}.boostCreep(@{creep.as_ref()})},
//             Some(count) => js_unwrap! {@{self.as_ref()}.boostCreep(@{creep.as_ref()}, @{count})},
//         }
//     }

//     pub fn run_reaction(&self, lab1: &StructureLab, lab2: &StructureLab) -> ReturnCode {
//         js_unwrap! {@{self.as_ref()}.runReaction(@{lab1.as_ref()}, @{lab2.as_ref()})}
//     }

//     pub fn reverse_reaction(&self, lab1: &StructureLab, lab2: &StructureLab) -> ReturnCode {
//         js_unwrap! {@{self.as_ref()}.reverseReaction(@{lab1.as_ref()}, @{lab2.as_ref()})}
//     }

//     pub fn unboost_creep(&self, creep: &Creep) -> ReturnCode {
//         js_unwrap!(@{self.as_ref()}.unboostCreep(@{creep.as_ref()}))
//     }
// }
