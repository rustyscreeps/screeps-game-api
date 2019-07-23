//! An interface for communicating between shards.
//!
//! Quoting from [`InterShardMemory`] documentation::
//!
//! > InterShardMemory object provides an interface for communicating between
//! > shards. Your script is executed separatedly on each shard, and their Memory
//! > objects are isolated from each other. In order to pass messages and data
//! > between shards, you need to use InterShardMemory instead.
//! >
//! > Every shard can have its own data string that can be accessed by all other
//! > shards. A shard can write only to its own data, other shards' data is
//! > read-only.
//! >
//! > This data has nothing to do with Memory contents, it's a separate data
//! > container.
//!
//! [`InterShardMemory`]: https://docs.screeps.com/api/#InterShardMemory
use crate::macros::*;

/// Returns the string contents of the current shard's data.
pub fn get_local() -> String {
    js_unwrap!(InterShardMemory.getLocal())
}

/// Replace the current shard's data with the new value.
pub fn set_local(value: &str) {
    js! {
        InterShardMemory.setLocal(@{value});
    }
}

/// Returns the string contents of another shard's data.
///
/// Consider using [`game::cpu::shard_limits`] to retrieve shard names.
///
/// [`game::cpu::shard_limits`]: crate::game::cpu::shard_limits
pub fn get_remote(shard: &str) -> String {
    js_unwrap!(InterShardMemory.getRemote(@{shard}))
}
