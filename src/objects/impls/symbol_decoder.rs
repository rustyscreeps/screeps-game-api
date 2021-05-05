use crate::{
    constants::ResourceType,
    objects::RoomObject,
    prelude::*,
};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`SymbolDecoder`], which can be used to decode
    /// matching symbol resources to score points on the leaderboard.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolDecoder)
    ///
    /// [`ResourceType::Score`]: crate::constants::ResourceType::Score
    #[wasm_bindgen(extends = RoomObject)]
    #[cfg_attr(docsrs, doc(cfg(feature = "enable-symbols")))]
    #[derive(Clone)]
    pub type SymbolDecoder;

    /// Object ID of the collector, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolDecoder.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &SymbolDecoder) -> JsString;

    /// The [`ResourceType`] allowed to be transferred to this [`SymbolDecoder`]
    /// to score points.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolDecoder.resourceType)
    #[wasm_bindgen(method, getter = resourceType)]
    pub fn resource_type(this: &SymbolDecoder) -> ResourceType;

    /// The multipler applied to points scored at this decoder, as determined by
    /// the level of the room's controller.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#SymbolDecoder.scoreMultiplier)
    #[wasm_bindgen(method, getter = scoreMultiplier)]
    pub fn score_multiplier(this: &SymbolDecoder) -> u32;
}

impl HasNativeId for SymbolDecoder {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}

impl HasPosition for SymbolDecoder {
    fn pos(&self) -> Option<RoomPosition> {
        RoomObject::pos(self.as_ref())
    }
}

impl RoomObjectProperties for SymbolDecoder {
    fn effects(&self) -> Array {
        RoomObject::effects(self.as_ref())
    }

    fn room(&self) -> Option<Room> {
        RoomObject::room(self.as_ref())
    }
}
