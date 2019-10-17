//! See [http://docs.screeps.com/api/#Game.cpu]
//!
//! [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
use std::collections;

use serde::{Deserialize, Serialize};

use crate::{constants::ReturnCode, traits::TryInto};

/// See [`v8_getheapstatistics`]
///
/// [`v8_getheapstatistics`]: https://nodejs.org/dist/latest-v8.x/docs/api/v8.html#v8_v8_getheapstatistics
#[derive(Default, Serialize, Deserialize)]
pub struct HeapStatistics {
    pub total_heap_size: u32,
    pub total_heap_size_executable: u32,
    pub total_physical_size: u32,
    pub used_heap_size: u32,
    pub heap_size_limit: u32,
    pub malloced_memory: u32,
    pub peak_malloced_memory: u32,
    pub does_zap_garbage: u32,
    pub externally_allocated_size: u32,
}

js_serializable!(HeapStatistics);
js_deserializable!(HeapStatistics);

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn limit() -> f64 {
    js_unwrap!(Game.cpu.limit)
}

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn tick_limit() -> f64 {
    js_unwrap!(Game.cpu.tickLimit)
}

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn bucket() -> f64 {
    js_unwrap!(Game.cpu.bucket)
}

/// See [http://docs.screeps.com/api/#Game.cpu]
///
/// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
pub fn shard_limits() -> collections::HashMap<String, f64> {
    js_unwrap!(Game.cpu.shardLimits)
}

/// See [http://docs.screeps.com/api/#Game.getHeapStatistics]
///
/// [http://docs.screeps.com/api/#Game.getHeapStatistics]: http://docs.screeps.com/api/#Game.getHeapStatistics
///
/// Returns object with all 0 values if heap statistics are not available.
pub fn get_heap_statistics() -> HeapStatistics {
    use stdweb::Value;

    let heap_stats: Value = js_unwrap!(Game.cpu.getHeapStatistics && Game.cpu.getHeapStatistics());

    match heap_stats {
        Value::Null | Value::Undefined | Value::Bool(false) => HeapStatistics::default(),
        other => other.try_into().expect(
            "expected Game.cpu.getHeapStatistics() to return an object with a known format",
        ),
    }
}

/// See [http://docs.screeps.com/api/#Game.getUsed]
///
/// [http://docs.screeps.com/api/#Game.getUsed]: http://docs.screeps.com/api/#Game.getUsed
pub fn get_used() -> f64 {
    js_unwrap!(Game.cpu.getUsed())
}

/// See [http://docs.screeps.com/api/#Game.setShardLimits]
///
/// [http://docs.screeps.com/api/#Game.setShardLimits]: http://docs.screeps.com/api/#Game.setShardLimits
pub fn set_shard_limits(limits: collections::HashMap<String, f64>) -> ReturnCode {
    js_unwrap!(Game.cpu.setShardLimits(@{limits}))
}

/// Reset your runtime environment and wipe all data in heap memory.
///
/// See [Game.cpu.halt()](https://docs.screeps.com/api/#Game.halt).
pub fn halt() {
    js! {
        Game.cpu.halt();
    }
}
