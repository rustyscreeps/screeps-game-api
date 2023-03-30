//! Utility functions for visuals that the game API exposes on the `console`
//! object.
use js_sys::JsString;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// Add a visual, in json format, or multiple visuals separated by `\n`.
    /// Each line must be:
    ///   - A serialized [`Visual`], applying to a given room, if the target is
    ///     the name of a room
    ///   - A serialized [`Visual`], which will be shown in all rooms, if target
    ///     is `None`
    ///   - A serialized [`MapVisualShape`], if the target is "map"
    ///
    /// [`Visual`]: crate::objects::Visual
    /// [`MapVisualShape`]: crate::objects::MapVisualShape
    #[wasm_bindgen(js_namespace = console, js_name = addVisual)]
    pub fn add_visual(target: Option<&JsString>, visual: &JsValue);

    /// Get the visuals applied to a given target so far in the current tick
    /// separated by `\n`, with the taget being visuals applied to a given room,
    /// `None` for visuals applied for all rooms, or "map" for map visuals.
    #[wasm_bindgen(js_namespace = console, js_name = getVisual)]
    pub fn get_visual(target: Option<&JsString>) -> Option<JsString>;

    /// Get the size of the visuals applied for the current tick, either for a
    /// given room, `None` for visuals applied for all rooms, or "map" for
    /// map visuals.
    #[wasm_bindgen(js_namespace = console, js_name = getVisualSize)]
    pub fn get_visual_size(target: Option<&JsString>) -> u32;

    /// Clear all of the set visuals for the current tick, either for a given
    /// room, `None` for visuals applied for all rooms, or "map" for map
    /// visuals.
    #[wasm_bindgen(js_namespace = console, js_name = clearVisual)]
    pub fn clear_visual(target: Option<&JsString>);
}
