use crate::{
    constants::ReturnCode,
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};
use wasm_bindgen::prelude::*;

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
    fn attack_internal(this: &StructureTower, target: &RoomObject) -> ReturnCode;

    #[wasm_bindgen(method, js_name = heal)]
    fn heal_internal(this: &StructureTower, target: &RoomObject) -> ReturnCode;

    /// Repair a [`Structure`] in the room, adding hit points depending on
    /// range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.repair)
    #[wasm_bindgen(method, js_name = repair)]
    pub fn repair(this: &StructureTower, target: &Structure) -> ReturnCode;
}

impl StructureTower {
    /// Attack a [`Creep`], [`PowerCreep`], or [`Structure`] in the room,
    /// dealing damage depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.attack)
    ///
    /// [`Creep`]: crate::objects::Creep
    /// [`PowerCreep`]: crate::objects::PowerCreep
    pub fn attack<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Attackable,
    {
        Self::attack_internal(self, target.as_ref())
    }

    /// Heal a [`Creep`] or [`PowerCreep`] in the room, adding hit points
    /// depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTower.heal)
    ///
    /// [`Creep`]: crate::objects::Creep
    /// [`PowerCreep`]: crate::objects::PowerCreep
    pub fn heal<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Healable,
    {
        Self::heal_internal(self, target.as_ref())
    }
}

impl HasStore for StructureTower {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
