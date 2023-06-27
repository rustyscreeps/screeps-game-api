use wasm_bindgen::prelude::*;

use crate::{
    constants::ErrorCode,
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

    #[wasm_bindgen(method, js_name = launchNuke)]
    fn launch_nuke_internal(this: &StructureNuker, target: &RoomPosition) -> i8;
}

impl StructureNuker {
    /// Launch a nuke at a target [`RoomPosition`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureNuker.launchNuke)
    pub fn launch_nuke(&self, target: &RoomPosition) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.launch_nuke_internal(target))
    }
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
