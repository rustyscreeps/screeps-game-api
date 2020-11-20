use crate::{
    constants::Color,
    objects::{RoomObject, RoomPosition},
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

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

// use stdweb::Value;

// use crate::{
//     constants::{Color, ReturnCode},
//     objects::{Flag, HasPosition},
//     traits::TryFrom,
// };

// simple_accessors! {
//     impl Flag {
//         pub fn color() -> Color = color;
//         pub fn name() -> String = name;
//         pub fn secondary_color() -> Color = secondaryColor;
//     }
// }

// impl Flag {
//     /// Useful method for constructing Flag from the result of
//     /// `Position.createFlag` or `Room.createFlag`.
//     ///
//     /// String names are mapped to Ok(Ok(s)), return codes are mapped to
//     /// Ok(Err(e)), other unknown inputs are mapped to Err(e).
//     pub(crate) fn interpret_creation_ret_value(
//         value: Value,
//     ) -> Result<Result<String, ReturnCode>, crate::ConversionError> {
//         match value {
//             num @ Value::Number(_) => Ok(Err(ReturnCode::try_from(num)?)),
//             other => String::try_from(other).map(Ok),
//         }
//     }

//     pub fn remove(&self) {
//         js! { @(no_return)
//             @{self.as_ref()}.remove();
//         }
//     }

//     pub fn set_color(&self, color: Color, secondary_color: Option<Color>) {
//         match secondary_color {
//             None => js! { @(no_return)
//                 @{self.as_ref()}.setColor(@{color as u8});
//             },
//             Some(sec_color) => js! { @(no_return)
//                 @{self.as_ref()}.setColor(
//                     @{color as u8},
//                     @{sec_color as u8},
//                 );
//             },
//         };
//     }

//     pub fn set_position<T: HasPosition>(&self, pos: T) {
//         let pos = pos.pos();
//         js! { @(no_return)
//             @{self.as_ref()}.setPosition(pos_from_packed(@{pos.packed_repr()}));
//         }
//     }

//     pub fn set_position_xy(&self, x: u32, y: u32) {
//         js! { @(no_return)
//             @{self.as_ref()}.setPosition(@{x}, @{y});
//         }
//     }
// }
