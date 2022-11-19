use crate::{constants::ResourceType, objects::RoomObject, prelude::*};
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// A [`Deposit`], which can be harvested for resources used in commodities.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Deposit)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Deposit;

    /// Ticks until the deposit can be harvested again.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Deposit.cooldown)
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &Deposit) -> u32;

    /// Type of resource contained in this deposit.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Deposit.depositType)
    #[wasm_bindgen(method, getter = depositType)]
    pub fn deposit_type(this: &Deposit) -> ResourceType;

    /// Object ID of the deposit, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Deposit.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Deposit) -> JsString;

    /// The cooldown caused by the most recent harvest action for this deposit.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Deposit.lastCooldown)
    #[wasm_bindgen(method, getter = lastCooldown)]
    pub fn last_cooldown(this: &Deposit) -> u32;

    /// The number of ticks until this deposit disappears, which is reset if it
    /// is harvested.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Deposit.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &Deposit) -> u32;
}

impl CanDecay for Deposit {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

impl HasCooldown for Deposit {
    fn cooldown(&self) -> u32 {
        Self::cooldown(self)
    }
}

impl HasNativeId for Deposit {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}
