//! Interface for Screeps [`InterShardMemory`], allowing communication between
//! instances of your code running on different shards.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory)

use js_sys::JsString;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    pub type InterShardMemory;

    /// Get the current local [`JsString`] intershard memory for this shard.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.getLocal)
    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = getLocal)]
    pub fn get_local() -> Option<JsString>;

    /// Overwrite the current shard's intershard memory segment with new
    /// contents.  Maximum allowed length of [`INTER_SHARD_MEMORY_SIZE_LIMIT`]
    /// bytes.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.setLocal)
    ///
    /// [`INTER_SHARD_MEMORY_SIZE_LIMIT`]:
    /// crate::constants::INTER_SHARD_MEMORY_SIZE_LIMIT
    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = setLocal)]
    pub fn set_local(val: &JsString);

    /// Get the data that another shard's code instance has written to its
    /// intershard memory segment.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#InterShardMemory.getRemote)
    #[wasm_bindgen(static_method_of = InterShardMemory, js_name = getRemote)]
    pub fn get_remote(val: &JsString) -> Option<JsString>;
}
