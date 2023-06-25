//! Interface for Screeps [`RawMemory`] global object.
//!
//! This is available as an alternative to the `Memory` object in the js heap,
//! which itself is just a light wrapper around serializing into and
//! deserializing JSON into [`RawMemory`]. String data stored can be retrieved
//! after the running bot code is restarted (either by the server or by a new
//! version of the code being uploaded) and decoded using serde or another
//! option.
//!
//! Also contains functions for accessing memory segments and other
//! players' active foreign segments.
//!
//! [`RawMemory`]: https://docs.screeps.com/api/#RawMemory
use js_sys::{Array, JsString, Object};

use wasm_bindgen::prelude::*;

use crate::js_collections::JsHashMap;

#[wasm_bindgen]
extern "C" {
    type RawMemory;

    #[wasm_bindgen(static_method_of = RawMemory, getter = segments)]
    fn segments() -> Object;

    #[wasm_bindgen(static_method_of = RawMemory, getter = foreignSegment)]
    fn foreign_segment() -> Option<ForeignSegment>;

    #[wasm_bindgen(static_method_of = RawMemory)]
    fn get() -> JsString;

    #[wasm_bindgen(static_method_of = RawMemory)]
    fn set(val: &JsString);

    #[wasm_bindgen(static_method_of = RawMemory, js_name = setActiveSegments)]
    fn set_active_segments(segment_ids: &Array);

    #[wasm_bindgen(static_method_of = RawMemory, js_name = setActiveForeignSegment)]
    fn set_active_foreign_segment(username: &JsString, segment_id: Option<u8>);

    #[wasm_bindgen(static_method_of = RawMemory, js_name = setDefaultPublicSegment)]
    fn set_default_public_segment(segment_id: Option<u8>);

    #[wasm_bindgen(static_method_of = RawMemory, js_name = setPublicSegments)]
    fn set_public_segments(segment_ids: &[u8]);
}

/// Get a [`JsHashMap<u8, String>`] with all of the segments requested on
/// the previous tick, with segment numbers as keys and segment data in
/// [`JsString`] form as values.
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.segments)
pub fn segments() -> JsHashMap<u8, String> {
    RawMemory::segments().into()
}

/// Get the foreign memory segment belonging to another player requested
/// last tick.
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.foreignSegment)
pub fn foreign_segment() -> Option<ForeignSegment> {
    RawMemory::foreign_segment()
}

/// Get the stored serialized memory as a [`JsString`].
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.get)
pub fn get() -> JsString {
    RawMemory::get()
}

/// Overwrite the stored memory with a new [`JsString`]. Maximum allowed
/// size [`MEMORY_SIZE_LIMIT`] UTF-16 units.
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.set)
///
/// [`MEMORY_SIZE_LIMIT`]: crate::constants::MEMORY_SIZE_LIMIT
pub fn set(val: &JsString) {
    RawMemory::set(val)
}

/// Sets available memory segments for the next tick, as an array of numbers
/// from 0 to 99 (max of 10 segments allowed).
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setActiveSegments)
pub fn set_active_segments(segment_ids: &[u8]) {
    let segment_ids: Array = segment_ids
        .iter()
        .map(|s| *s as f64)
        .map(JsValue::from_f64)
        .collect();

    RawMemory::set_active_segments(&segment_ids)
}

/// Sets available foreign memory segment for the next tick to a memory
/// segment marked as public by another user. If no id is passed, the user's
/// default public segment is retrieved.
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setActiveForeignSegment)
pub fn set_active_foreign_segment(username: &JsString, segment_id: Option<u8>) {
    RawMemory::set_active_foreign_segment(username, segment_id)
}

/// Sets your default foreign memory segment for other players to read, or
/// remove your public segment with `None`.
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setDefaultPublicSegment)
pub fn set_default_public_segment(segment_id: Option<u8>) {
    RawMemory::set_default_public_segment(segment_id)
}

/// Sets which of your memory segments are readable to other players as
/// foreign segments, overriding previous settings.
///
/// [Screeps documentation](https://docs.screeps.com/api/#RawMemory.setPublicSegments)
pub fn set_public_segments(segment_ids: &[u8]) {
    RawMemory::set_public_segments(segment_ids)
}

#[wasm_bindgen]
extern "C" {
    /// The data from another user's foreign memory segment, which can be
    /// retrieved by [`foreign_segment`], after being requested on the previous
    /// tick by [`set_active_foreign_segment`].
    #[wasm_bindgen]
    pub type ForeignSegment;
    #[wasm_bindgen(method, getter)]
    pub fn username(this: &ForeignSegment) -> JsString;
    #[wasm_bindgen(method, getter)]
    pub fn id(this: &ForeignSegment) -> u8;
    #[wasm_bindgen(method, getter)]
    pub fn data(this: &ForeignSegment) -> JsString;
}
