use js_sys::JsString;
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    constants::Color,
    objects::{RoomObject, RoomPosition},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// A [`Flag`], which can be placed by the player or created automatically
    /// and are only visible to their owners. You can't create more than
    /// [`FLAGS_LIMIT`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag)
    ///
    /// [`FLAGS_LIMIT`]: crate::constants::FLAGS_LIMIT
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Flag;

    /// Primary color of the flag.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.color)
    #[wasm_bindgen(method, getter)]
    pub fn color(this: &Flag) -> Color;

    /// A shortcut to `Memory.flags[flag.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.memory)
    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &Flag) -> JsValue;

    /// Sets a new value to `Memory.flags[flag.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.memory)
    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &Flag, val: &JsValue);

    /// The flag's name as an owned reference to a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.name)
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Flag) -> JsString;

    /// Remove the flag.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.remove)
    #[wasm_bindgen(method)]
    pub fn remove(this: &Flag);

    /// Set the color (and optionally, the secondary color) of the flag.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.setColor)
    #[wasm_bindgen(method, js_name = setColor)]
    pub fn set_color(this: &Flag, color: Color, secondary_color: Option<Color>);

    /// Set the position of the flag
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.setPosition)
    #[wasm_bindgen(method, js_name = setPosition)]
    pub fn set_position(this: &Flag, pos: RoomPosition);
}

impl JsCollectionFromValue for Flag {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}
