//! Information about, and functions to manage, your code's resource utilization
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game.cpu)
use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

use crate::{constants::ErrorCode, prelude::*};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_name = "cpu")]
    type Cpu;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, getter, js_name = limit)]
    fn limit() -> u32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, getter, js_name = tickLimit)]
    fn tick_limit() -> f64;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, getter, js_name = bucket)]
    fn bucket() -> i32;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, js_name = shardLimits)]
    fn shard_limits() -> Object;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, getter, js_name = unlocked)]
    fn unlocked() -> bool;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, getter, js_name = unlockedTime)]
    fn unlocked_time() -> Option<u64>;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, js_name = getHeapStatistics)]
    fn get_heap_statistics() -> HeapStatistics;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, js_name = getUsed)]
    fn get_used() -> f64;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, js_name = halt)]
    fn halt();

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, js_name = setShardLimits)]
    fn set_shard_limits(limits: &Object) -> i8;

    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, js_name = unlock)]
    fn unlock() -> i8;

    #[cfg(feature = "generate-pixel")]
    #[wasm_bindgen(js_namespace = ["Game"], js_class = "cpu", static_method_of = Cpu, js_name = generatePixel)]
    fn generate_pixel() -> i8;
}

/// Your assigned CPU for the current shard.
pub fn limit() -> u32 {
    Cpu::limit()
}

/// The amount of CPU available for execution this tick, which consists of
/// your per-tick CPU [`limit`] plus your accrued [`bucket`], up to a maximum of
/// 500 ([`CPU_TICK_LIMIT_MAX`]); [`f64::INFINITY`] on sim.
///
/// [`CPU_TICK_LIMIT_MAX`]: crate::constants::extra::CPU_TICK_LIMIT_MAX
pub fn tick_limit() -> f64 {
    Cpu::tick_limit()
}

/// The amount of CPU that has accumulated in your bucket.
pub fn bucket() -> i32 {
    Cpu::bucket()
}

/// Your assigned CPU limits for each shard in an [`Object`], with shard
/// names in [`JsString`] form as keys and numbers as values. This is the
/// same format accepted by [`set_shard_limits`].
pub fn shard_limits() -> JsHashMap<JsString, u32> {
    Cpu::shard_limits().into()
}

/// Whether your account is unlocked to have full CPU.
pub fn unlocked() -> bool {
    Cpu::unlocked()
}

/// If your account has been unlocked for a limited time, contains the time
/// it's unlocked until in milliseconds since epoch.
pub fn unlocked_time() -> Option<u64> {
    Cpu::unlocked_time()
}

/// Get information about your script's memory heap usage.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.getHeapStatistics)
pub fn get_heap_statistics() -> HeapStatistics {
    Cpu::get_heap_statistics()
}

/// Get the amount of CPU time used for execution so far this tick.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.getUsed)
pub fn get_used() -> f64 {
    Cpu::get_used()
}

/// Stop execution of your script immediately and requests the destruction of
/// your code's environment, which will start fresh on the following tick.
///
/// Note that this will cause your code to not complete API calls called earlier
/// in the current tick; no log messages will be sent to the console, email
/// messages sent via `game::notify` are not sent, and game actions taken should
/// not complete.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.halt)
pub fn halt() {
    Cpu::halt()
}

/// Sets new shard limits for your script in an [`Object`], with shard names
/// in [`JsString`] form as keys and numbers as values. This is the same
/// format accepted by [`shard_limits`]. Total amount of CPU should
/// remain equal to the sum of the values of [`shard_limits`]. This method
/// can be used only once per 12 hours ([`CPU_SET_SHARD_LIMITS_COOLDOWN`]).
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.setShardLimits)
///
/// [`CPU_SET_SHARD_LIMITS_COOLDOWN`]: crate::constants::CPU_SET_SHARD_LIMITS_COOLDOWN
pub fn set_shard_limits(limits: &Object) -> Result<(), ErrorCode> {
    ErrorCode::result_from_i8(Cpu::set_shard_limits(limits))
}

/// Consume a [`CpuUnlock`] to unlock your full CPU for 24 hours.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.unlock)
///
/// [`CpuUnlock`]: crate::constants::IntershardResourceType::CpuUnlock
pub fn unlock() -> Result<(), ErrorCode> {
    ErrorCode::result_from_i8(Cpu::unlock())
}

/// Generate a [`Pixel`], consuming [`PIXEL_CPU_COST`] CPU from your bucket.
///
/// [Screeps documentation](https://docs.screeps.com/api/#Game.cpu.generatePixel)
///
/// [`Pixel`]: crate::constants::IntershardResourceType::Pixel
/// [`PIXEL_CPU_COST`]: crate::constants::PIXEL_CPU_COST
#[cfg(feature = "generate-pixel")]
pub fn generate_pixel() -> Result<(), ErrorCode> {
    ErrorCode::result_from_i8(Cpu::generate_pixel())
}

#[wasm_bindgen]
extern "C" {
    /// Object with info about the memory heap of your virtual machine.
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

    /// Whether the virtual machine overwrites memory as it deallocates -
    /// usually 0.
    #[wasm_bindgen(method, getter)]
    pub fn does_zap_garbage(this: &HeapStatistics) -> u32;

    /// External allocations that are outside of the v8 heap but still count
    /// against the memory limit.
    #[wasm_bindgen(method, getter)]
    pub fn externally_allocated_size(this: &HeapStatistics) -> u32;
}
