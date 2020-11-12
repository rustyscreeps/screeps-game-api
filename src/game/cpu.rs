//! Information about, and functions to manage, your code's resource utilization
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game.cpu)
use js_sys::Object;

use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Object with info about your CPU allocations and limits from [`Game::cpu`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu)
    #[wasm_bindgen]
    pub type CpuInfo;

    /// Your assigned CPU for the current shard.
    #[wasm_bindgen(method, getter)]
    pub fn limit(this: &CpuInfo) -> u32;

    /// The amount of CPU available for execution this tick, which consists of your [`CpuInfo::limit`] and [`CpuInfo::bucket`] up to a maximum of 500 ([`CPU_TICK_LIMIT_MAX`]).
    ///
    /// [`CPU_TICK_LIMIT_MAX`]: crate::constants::extra::CPU_TICK_LIMIT_MAX
    #[wasm_bindgen(method, getter = tickLimit)]
    pub fn tick_limit(this: &CpuInfo) -> u32;

    /// The amount of CPU that has accumulated in your bucket.
    #[wasm_bindgen(method, getter)]
    pub fn bucket(this: &CpuInfo) -> u32;

    /// Your assigned CPU limits for each shard in an [`Object`], with shard names in [`JsString`] form as keys and numbers as values. This is the same format accepted by [`Game::cpu_set_shard_limits`].
    #[wasm_bindgen(method, getter = shardLimits)]
    pub fn shard_limits(this: &CpuInfo) -> Object;

    /// Whether your account is unlocked to have full CPU.
    #[wasm_bindgen(method, getter)]
    pub fn unlocked(this: &CpuInfo) -> bool;

    /// If your account has been unlocked for a limited time, contains the time it's unlocked until in milliseconds since epoch.
    #[wasm_bindgen(method, getter = unlockedTime)]
    pub fn unlocked_time(this: &CpuInfo) -> Option<u64>;
}

#[wasm_bindgen]
extern "C" {
    /// Object with info about the memory heap of your virtual machine
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.getHeapStatistics)
    #[wasm_bindgen]
    pub type HeapStatistics;

    /// The total heap consumed.
    #[wasm_bindgen(method, getter)]
    pub fn total_heap_size(this: &HeapStatistics) -> u32;

    /// The total heap consumed by executable code.
    #[wasm_bindgen(method, getter)]
    pub fn total_heap_size_executable(this: &HeapStatistics) -> u32;

    /// The total amount of heap committed to memory.
    #[wasm_bindgen(method, getter)]
    pub fn total_physical_size(this: &HeapStatistics) -> u32;

    /// Amount of heap available for allocation.
    #[wasm_bindgen(method, getter)]
    pub fn total_available_size(this: &HeapStatistics) -> u32;

    /// Total heap consumed by application data.
    #[wasm_bindgen(method, getter)]
    pub fn used_heap_size(this: &HeapStatistics) -> u32;

    /// The allowed limit for total heap memory.
    #[wasm_bindgen(method, getter)]
    pub fn heap_size_limit(this: &HeapStatistics) -> u32;

    /// Total amount of memory obtained by malloc.
    #[wasm_bindgen(method, getter)]
    pub fn malloced_memory(this: &HeapStatistics) -> u32;

    /// Maximum amount of memory obtained by malloc.
    #[wasm_bindgen(method, getter)]
    pub fn peak_malloced_memory(this: &HeapStatistics) -> u32;

    /// Whether the virtual machine overwrites memory as it deallocates - usually 0.
    #[wasm_bindgen(method, getter)]
    pub fn does_zap_garbage(this: &HeapStatistics) -> u32;

    /// External allocations that are outside of the v8 heap but still count against the memory limit.
    #[wasm_bindgen(method, getter)]
    pub fn externally_allocated_size(this: &HeapStatistics) -> u32;
}


// use std::collections;

// use serde::{Deserialize, Serialize};

// use crate::{constants::ReturnCode, traits::TryInto};

// /// See [`v8_getheapstatistics`]
// ///
// /// [`v8_getheapstatistics`]: https://nodejs.org/dist/latest-v8.x/docs/api/v8.html#v8_v8_getheapstatistics
// #[derive(Default, Serialize, Deserialize)]
// pub struct HeapStatistics {
//     pub total_heap_size: u32,
//     pub total_heap_size_executable: u32,
//     pub total_physical_size: u32,
//     pub total_available_size: i32,
//     pub used_heap_size: u32,
//     pub heap_size_limit: u32,
//     pub malloced_memory: u32,
//     pub peak_malloced_memory: u32,
//     pub does_zap_garbage: u32,
//     pub externally_allocated_size: u32,
// }

// js_serializable!(HeapStatistics);
// js_deserializable!(HeapStatistics);

// /// See [http://docs.screeps.com/api/#Game.cpu]
// ///
// /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
// pub fn limit() -> u32 {
//     js_unwrap!(Game.cpu.limit)
// }

// /// See [http://docs.screeps.com/api/#Game.cpu]
// ///
// /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
// pub fn tick_limit() -> u32 {
//     js_unwrap!(Game.cpu.tickLimit)
// }

// /// See [http://docs.screeps.com/api/#Game.cpu]
// ///
// /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
// pub fn bucket() -> u32 {
//     js_unwrap!(Game.cpu.bucket)
// }

// /// See [http://docs.screeps.com/api/#Game.cpu]
// ///
// /// [http://docs.screeps.com/api/#Game.cpu]: http://docs.screeps.com/api/#Game.cpu
// pub fn shard_limits() -> collections::HashMap<String, u32> {
//     js_unwrap!(Game.cpu.shardLimits)
// }

// /// See [http://docs.screeps.com/api/#Game.getHeapStatistics]
// ///
// /// [http://docs.screeps.com/api/#Game.getHeapStatistics]: http://docs.screeps.com/api/#Game.getHeapStatistics
// ///
// /// Returns object with all 0 values if heap statistics are not available.
// pub fn get_heap_statistics() -> HeapStatistics {
//     use stdweb::Value;

//     let heap_stats: Value = js_unwrap!(Game.cpu.getHeapStatistics && Game.cpu.getHeapStatistics());

//     match heap_stats {
//         Value::Null | Value::Undefined | Value::Bool(false) => HeapStatistics::default(),
//         other => other.try_into().expect(
//             "expected Game.cpu.getHeapStatistics() to return an object with a known format",
//         ),
//     }
// }

// /// See [http://docs.screeps.com/api/#Game.getUsed]
// ///
// /// [http://docs.screeps.com/api/#Game.getUsed]: http://docs.screeps.com/api/#Game.getUsed
// pub fn get_used() -> f64 {
//     js_unwrap!(Game.cpu.getUsed())
// }

// /// See [http://docs.screeps.com/api/#Game.setShardLimits]
// ///
// /// [http://docs.screeps.com/api/#Game.setShardLimits]: http://docs.screeps.com/api/#Game.setShardLimits
// pub fn set_shard_limits(limits: collections::HashMap<String, u32>) -> ReturnCode {
//     js_unwrap!(Game.cpu.setShardLimits(@{limits}))
// }

// /// Reset your runtime environment and wipe all data in heap memory.
// ///
// /// See [Game.cpu.halt()](https://docs.screeps.com/api/#Game.halt).
// pub fn halt() {
//     js! {
//         Game.cpu.halt();
//     }
// }
