use wasm_bindgen::prelude::*;

use crate::{
    constants::{ErrorCode, ResourceType},
    objects::{Creep, OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureLab`], which can be used to create
    /// mineral compounds.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
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

    #[wasm_bindgen(method, js_name = boostCreep)]
    fn boost_creep_internal(this: &StructureLab, creep: &Creep, body_part_count: Option<u32>)
        -> i8;

    #[wasm_bindgen(method, js_name = reverseReaction)]
    fn reverse_reaction_internal(
        this: &StructureLab,
        lab1: &StructureLab,
        lab2: &StructureLab,
    ) -> i8;

    #[wasm_bindgen(method, js_name = runReaction)]
    fn run_reaction_internal(this: &StructureLab, lab1: &StructureLab, lab2: &StructureLab) -> i8;

    #[wasm_bindgen(method, js_name = unboostCreep)]
    fn unboost_creep_internal(this: &StructureLab, creep: &Creep) -> i8;
}

impl StructureLab {
    /// Boost a [`Creep`] in melee range, consuming [`LAB_BOOST_ENERGY`] energy
    /// and [`LAB_BOOST_MINERAL`] of the boost compound from the
    /// [`StructureLab::store`] per boosted body part.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.boostCreep)
    ///
    /// [`LAB_BOOST_ENERGY`]: crate::constants::LAB_BOOST_ENERGY
    /// [`LAB_BOOST_MINERAL`]: crate::constants::LAB_BOOST_MINERAL
    pub fn boost_creep(
        &self,
        creep: &Creep,
        body_part_count: Option<u32>,
    ) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.boost_creep_internal(creep, body_part_count))
    }

    /// Reverse a reaction, splitting the compound in this [`StructureLab`] into
    /// its components in two other labs.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.reverseReaction)
    pub fn reverse_reaction(
        &self,
        lab1: &StructureLab,
        lab2: &StructureLab,
    ) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.reverse_reaction_internal(lab1, lab2))
    }

    /// Run a reaction, combining components from two other [`StructureLab`]s
    /// into a new compound in this lab.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.runReaction)
    pub fn run_reaction(&self, lab1: &StructureLab, lab2: &StructureLab) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.run_reaction_internal(lab1, lab2))
    }

    /// Unboost a [`Creep`], removing all boosts from its body and dropping
    /// [`LAB_UNBOOST_MINERAL`] per body part on the ground, with a cooldown
    /// equal to the total time to produce the removed boosts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureLab.unboostCreep)
    ///
    /// [`LAB_UNBOOST_MINERAL`]: crate::constants::LAB_UNBOOST_MINERAL
    pub fn unboost_creep(&self, creep: &Creep) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.unboost_creep_internal(creep))
    }
}

impl HasCooldown for StructureLab {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasStore for StructureLab {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
