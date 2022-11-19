use crate::{
    constants::{ReturnCode, StructureType},
    js_collections::JsCollectionFromValue,
    objects::{Owner, RoomObject},
    prelude::*,
};
use js_sys::JsString;
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    /// A [`ConstructionSite`] which is an object representing a structure under
    /// construction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type ConstructionSite;

    /// The Object ID of the [`ConstructionSite`], or `None` if it was created
    /// this tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &ConstructionSite) -> Option<JsString>;

    /// Whether you own the [`ConstructionSite`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &ConstructionSite) -> bool;

    /// The [`Owner`] of this construction site, which contains the owner's
    /// username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &ConstructionSite) -> Owner;

    /// The current progress toward completion of the structure being built.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.progress)
    #[wasm_bindgen(method, getter)]
    pub fn progress(this: &ConstructionSite) -> u32;

    /// The total progess toward constuction progress needed for the structure
    /// to be completed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.progressTotal)
    #[wasm_bindgen(method, getter = progressTotal)]
    pub fn progress_total(this: &ConstructionSite) -> u32;

    /// The type of structure being constructed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Structure.structureType)
    #[wasm_bindgen(method, getter = structureType)]
    pub fn structure_type(this: &ConstructionSite) -> StructureType;

    /// Remove the [`ConstructionSite`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#ConstructionSite.remove)
    #[wasm_bindgen(method)]
    pub fn remove(this: &ConstructionSite) -> ReturnCode;
}

impl MaybeHasNativeId for ConstructionSite {
    fn try_native_id(&self) -> Option<JsString> {
        Self::id_internal(self)
    }
}

impl JsCollectionFromValue for ConstructionSite {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}
