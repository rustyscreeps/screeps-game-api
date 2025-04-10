use wasm_bindgen::prelude::*;

use crate::{
    enums::action_error_codes::structure_tower::*,
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureTower`], which can heal, repair, or
    /// attack anywhere in the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureTower;

    /// The [`Store`] of the tower, which contains energy which is consumed when
    /// it takes actions.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureTower) -> Store;

    #[wasm_bindgen(method, js_name = attack)]
    fn attack_internal(this: &StructureTower, target: &RoomObject) -> i8;

    #[wasm_bindgen(method, js_name = heal)]
    fn heal_internal(this: &StructureTower, target: &RoomObject) -> i8;

    #[wasm_bindgen(method, js_name = repair)]
    fn repair_internal(this: &StructureTower, target: &Structure) -> i8;
}

impl StructureTower {
    /// Attack a [`Creep`], [`PowerCreep`], or [`Structure`] in the room,
    /// dealing damage depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.attack)
    ///
    /// [`Creep`]: crate::objects::Creep
    /// [`PowerCreep`]: crate::objects::PowerCreep
    pub fn attack<T>(&self, target: &T) -> Result<(), TowerAttackErrorCode>
    where
        T: ?Sized + Attackable,
    {
        TowerAttackErrorCode::result_from_i8(self.attack_internal(target.as_ref()))
    }

    /// Heal a [`Creep`] or [`PowerCreep`] in the room, adding hit points
    /// depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.heal)
    ///
    /// [`Creep`]: crate::objects::Creep
    /// [`PowerCreep`]: crate::objects::PowerCreep
    pub fn heal<T>(&self, target: &T) -> Result<(), TowerHealErrorCode>
    where
        T: ?Sized + Healable,
    {
        TowerHealErrorCode::result_from_i8(self.heal_internal(target.as_ref()))
    }

    /// Repair a [`Structure`] in the room, adding hit points depending on
    /// range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.repair)
    pub fn repair<T>(&self, target: &T) -> Result<(), TowerRepairErrorCode>
    where
        T: ?Sized + Repairable,
    {
        TowerRepairErrorCode::result_from_i8(self.repair_internal(target.as_ref()))
    }
}

impl HasStore for StructureTower {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl Attackable for StructureTower {}
impl Dismantleable for StructureTower {}
impl Repairable for StructureTower {}
impl Transferable for StructureTower {}
impl Withdrawable for StructureTower {}
