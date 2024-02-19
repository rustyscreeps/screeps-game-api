use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    constants::{ErrorCode, StructureType},
    objects::{Owner, RoomObject},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// A [`ConstructionSite`] which is an object representing a structure under
    /// construction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type ConstructionSite;

    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &ConstructionSite) -> Option<JsString>;

    #[wasm_bindgen(method, getter = my)]
    fn my_internal(this: &ConstructionSite) -> bool;

    #[wasm_bindgen(method, getter = owner)]
    fn owner_internal(this: &ConstructionSite) -> Owner;

    #[wasm_bindgen(method, getter = progress)]
    fn progress_internal(this: &ConstructionSite) -> u32;

    #[wasm_bindgen(method, getter = progressTotal)]
    fn progress_total_internal(this: &ConstructionSite) -> u32;

    #[wasm_bindgen(method, getter = structureType)]
    fn structure_type_internal(this: &ConstructionSite) -> StructureType;

    #[wasm_bindgen(method, js_name = remove)]
    fn remove_internal(this: &ConstructionSite) -> i8;
}

impl ConstructionSite {
    /// Whether you own the [`ConstructionSite`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.my)
    pub fn my(&self) -> bool {
        self.my_internal()
    }

    /// The [`Owner`] of this construction site, which contains the owner's
    /// username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.owner)
    pub fn owner(&self) -> Owner {
        self.owner_internal()
    }

    /// The current progress toward completion of the structure being built.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.progress)
    pub fn progress(&self) -> u32 {
        self.progress_internal()
    }

    /// The total progess toward constuction progress needed for the structure
    /// to be completed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.progressTotal)
    pub fn progress_total(&self) -> u32 {
        self.progress_total_internal()
    }

    /// The type of structure being constructed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.structureType)
    pub fn structure_type(&self) -> StructureType {
        self.structure_type_internal()
    }

    /// Remove the [`ConstructionSite`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.remove)
    pub fn remove(&self) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.remove_internal())
    }
}

impl MaybeHasId for ConstructionSite {
    /// The Object ID of the [`ConstructionSite`], or `None` if it was created
    /// this tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.id)
    fn try_js_raw_id(&self) -> Option<JsString> {
        self.id_internal()
    }
}

impl JsCollectionFromValue for ConstructionSite {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}
