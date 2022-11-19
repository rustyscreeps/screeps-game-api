use crate::objects::{RoomObject, Structure};
use wasm_bindgen::prelude::*;

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
