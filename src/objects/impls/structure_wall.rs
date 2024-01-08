use wasm_bindgen::prelude::*;

use crate::{
    objects::{RoomObject, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureWall`], which blocks movement of all
    /// creeps.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureWall)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    #[derive(Clone, Debug)]
    pub type StructureWall;
}

impl Attackable for StructureWall {}
impl Dismantleable for StructureWall {}
impl Repairable for StructureWall {}
