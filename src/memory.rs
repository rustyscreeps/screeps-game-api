//! Interface with Screeps' `Memory` global variable
//!
//! If you wish to access the `Memory` object stored in the javascript heap
//! which has its encoding, storage, and decoding from JSON handled by the game,
//! this allows accessing a reference to the [`ROOT`] of Memory object. Game
//! objects which have an automatic memory accessor can access references to
//! their respective parts of the object, eg.
//! [`Creep::memory`]/[`StructureSpawn::memory`]. You can work with these
//! objects using [`js_sys::Reflect`], or by converting the value into a
//! wasm_bindgen compatible type with the properly access functions you need via
//! [`wasm_bindgen::JsCast`].
//!
//! [`ROOT`]: crate::memory::ROOT
//! [`Creep::memory`]: crate::objects::Creep::memory
//! [`StructureSpawn::memory`]: crate::objects::StructureSpawn::memory
use js_sys::Object;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Get a reference to the `Memory` global object. Note that this object
    /// gets recreated each tick by the Screeps engine, so references from it
    /// should not be held beyond the current tick.
    #[wasm_bindgen(js_name = Memory)]
    pub static ROOT: Object;

}
