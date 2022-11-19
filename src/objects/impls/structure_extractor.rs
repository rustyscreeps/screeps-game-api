use crate::{
    objects::{OwnedStructure, RoomObject, Structure},
    prelude::*,
};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureExtractor`], which can be placed on
    /// top of a [`Mineral`] to extract resources.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtractor)
    ///
    /// [`Mineral`]: crate::objects::Mineral
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
    pub type StructureExtractor;

    /// Ticks until this extractor can be used to [`Creep::harvest`] its
    /// [`Mineral`] after a previous harvest.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureExtractor.cooldown)
    ///
    /// [`Creep::harvest`]: crate::objects::Creep::harvest
    /// [`Mineral`]: crate::objects::Mineral
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureExtractor) -> u32;
}

impl HasCooldown for StructureExtractor {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}
