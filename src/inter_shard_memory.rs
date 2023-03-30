//! Interface for Screeps inter-shard memory, allowing communication between
//! instances of your code running on different shards.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory)
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    type InterShardMemory;

    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = getLocal)]
    fn get_local() -> Option<JsString>;

    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = setLocal)]
    fn set_local(val: &JsString);

    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = getRemote)]
    fn get_remote(shard: &JsString) -> Option<JsString>;
}

/// Get the current local [`JsString`] intershard memory for this shard.
///
/// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.getLocal)
pub fn get_local() -> Option<JsString> {
    InterShardMemory::get_local()
}

/// Overwrite the current shard's intershard memory segment with new
/// contents.  Maximum allowed length of [`INTER_SHARD_MEMORY_SIZE_LIMIT`]
/// UTF-16 units.
///
/// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.setLocal)
///
/// [`INTER_SHARD_MEMORY_SIZE_LIMIT`]:
/// crate::constants::INTER_SHARD_MEMORY_SIZE_LIMIT
pub fn set_local(val: &JsString) {
    InterShardMemory::set_local(val)
}

/// Get the data that another shard's code instance has written to its
/// intershard memory segment.
///
/// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.getRemote)
pub fn get_remote(shard: &JsString) -> Option<JsString> {
    InterShardMemory::get_remote(shard)
}
