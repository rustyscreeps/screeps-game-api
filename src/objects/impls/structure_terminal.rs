use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    constants::{ErrorCode, ResourceType},
    local::RoomName,
    objects::{OwnedStructure, RoomObject, Store, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructureTerminal`], which can send resources
    /// to distant rooms and participate in the market.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal)
    #[wasm_bindgen(extends = RoomObject, extends = Structure, extends = OwnedStructure)]
    #[derive(Clone, Debug)]
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

    #[wasm_bindgen(method, js_name = send)]
    fn send_internal(
        this: &StructureTerminal,
        resource_type: ResourceType,
        amount: u32,
        destination: &JsString,
        description: Option<&JsString>,
    ) -> i8;
}

impl StructureTerminal {
    /// Send resources to another room's terminal.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructureTerminal.send)
    pub fn send(
        &self,
        resource_type: ResourceType,
        amount: u32,
        destination: RoomName,
        description: Option<&str>,
    ) -> Result<(), ErrorCode> {
        let desination = destination.into();
        let description = description.map(JsString::from);

        ErrorCode::result_from_i8(self.send_internal(
            resource_type,
            amount,
            &desination,
            description.as_ref(),
        ))
    }
}

impl HasCooldown for StructureTerminal {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasStore for StructureTerminal {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl Attackable for StructureTerminal {}
impl Dismantleable for StructureTerminal {}
impl Repairable for StructureTerminal {}
impl Transferable for StructureTerminal {}
impl Withdrawable for StructureTerminal {}
