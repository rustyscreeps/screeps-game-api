use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    constants::Color,
    enums::action_error_codes::flag::*,
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

    /// The flag's name as a [`String`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.name)
    #[wasm_bindgen(method, getter)]
    pub fn name(this: &Flag) -> String;

    /// The flag's name as a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.name)
    #[wasm_bindgen(method, getter = name)]
    pub fn name_jsstring(this: &Flag) -> JsString;

    #[wasm_bindgen(method, js_name = remove)]
    fn remove_internal(this: &Flag) -> i8;

    #[wasm_bindgen(method, js_name = setColor)]
    fn set_color_internal(this: &Flag, color: Color, secondary_color: Option<Color>) -> i8;

    #[wasm_bindgen(method, js_name = setPosition)]
    fn set_position_internal(this: &Flag, pos: RoomPosition) -> i8;
}

impl Flag {
    /// Remove the flag.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.remove)
    pub fn remove(&self) -> Result<(), FlagRemoveErrorCode> {
        FlagRemoveErrorCode::result_from_i8(self.remove_internal())
    }

    /// Set the color (and optionally, the secondary color) of the flag.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.setColor)
    pub fn set_color(
        &self,
        color: Color,
        secondary_color: Option<Color>,
    ) -> Result<(), SetColorErrorCode> {
        SetColorErrorCode::result_from_i8(self.set_color_internal(color, secondary_color))
    }

    /// Set the position of the flag
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Flag.setPosition)
    pub fn set_position(&self, pos: RoomPosition) -> Result<(), SetPositionErrorCode> {
        SetPositionErrorCode::result_from_i8(self.set_position_internal(pos))
    }
}

impl JsCollectionFromValue for Flag {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}
