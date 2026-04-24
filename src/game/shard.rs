//! Information about the current shard.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#Game.shard)
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[cfg(feature = "mmo")]
use crate::{enums::action_error_codes::game::shard::*, prelude::*};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "shard")]
    type Shard;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = name)]
    fn name() -> JsString;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = type)]
    fn shard_type() -> JsString;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = ptr)]
    fn ptr() -> bool;

    #[cfg(feature = "mmo")]
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = access)]
    fn access() -> bool;

    #[cfg(feature = "mmo")]
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, getter, js_name = accessTime)]
    fn access_time() -> Option<f64>;

    #[cfg(feature = "mmo")]
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "shard", static_method_of = Shard, js_name = activateAccess)]
    fn activate_access() -> i8;
}

/// Current shard name.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard.name)
pub fn name() -> String {
    Shard::name().into()
}

/// Shard type. Currently always "normal".
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard.type)
pub fn shard_type() -> String {
    Shard::shard_type().into()
}

/// Flag for if this is a public test server or not.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard.ptr)
pub fn ptr() -> bool {
    Shard::ptr()
}

/// Whether you have access to the current shard. Always true for all shards
/// other than shardX.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard.access)
#[cfg(feature = "mmo")]
pub fn access() -> bool {
    Shard::access()
}

// note: f64 due to https://github.com/rustwasm/wasm-bindgen/issues/4113
/// If your access to the shard is for a limited time, contains the time it's
/// unlocked until, in milliseconds since epoch.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard.accessTime)
#[cfg(feature = "mmo")]
pub fn access_time() -> Option<f64> {
    Shard::access_time()
}

/// Consume an [`AccessKey`] to unlock your access to the premium shard for 30
/// days.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.shard.activateAccess)
///
/// [`AccessKey`]: crate::constants::IntershardResourceType::AccessKey
#[cfg(feature = "mmo")]
pub fn activate_access() -> Result<(), ActivateAccessErrorCode> {
    ActivateAccessErrorCode::result_from_i8(Shard::activate_access())
}
