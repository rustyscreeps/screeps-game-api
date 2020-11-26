use crate::{
    prelude::*,
    constants::ResourceType,
    objects::{OwnedStructure, RoomObject, Store, Structure},
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureTerminal`], which can send resources
    /// to distant rooms and participate in the market.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    pub type StructureTerminal;

    /// The number of ticks until the [`StructureTerminal`] can use
    /// [`StructureTerminal::send`] or be used in a market transaction again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &StructureTerminal) -> u32;

    /// The [`Store`] of the terminal, which contains information about what
    /// resources it is it holding.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &StructureTerminal) -> Store;

    /// Send resources to another room's terminal.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal.send)
    #[wasm_bindgen(method)]
    pub fn send(
        this: &StructureTerminal,
        resource_type: ResourceType,
        amount: u32,
        destination: &JsString,
        description: Option<&JsString>,
    ) -> i8;
}

impl Attackable for StructureTerminal {}
impl IsStructure for StructureTerminal {}
impl HasStore for StructureTerminal {
    fn store(&self) -> Store {
        Self::store(self)
    }
}
