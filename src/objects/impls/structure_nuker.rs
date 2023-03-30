use wasm_bindgen::prelude::*;

use crate::{
    constants::ReturnCode,
    objects::{OwnedStructure, RoomObject, RoomPosition, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureNuker`], which consumes energy and
    /// ghodium to fire [`Nuke`]s.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker)
    ///
    /// [`Nuke`]: crate::objects::Nuke
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureNuker;

    /// The number of ticks until the [`StructureNuker`] can use
    /// [`StructureNuker::launch_nuke`] again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureNuker) -> u32;

    /// The [`Store`] of the nuker, which can have energy and ghodium
    /// transferred in (but not withdrawn).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureNuker) -> Store;

    /// Transfer energy from this [`StructureNuker`] to another, losing
    /// [`LINK_LOSS_RATIO`] and incurring a cooldown of [`LINK_COOLDOWN`] per
    /// range to the target.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker.launchNuke)
    #[wasm_bindgen(method, js_name = launchNuke)]
    pub fn launch_nuke(this: &StructureNuker, target: &RoomPosition) -> ReturnCode;
}

impl HasCooldown for StructureNuker {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasStore for StructureNuker {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
