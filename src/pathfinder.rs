//! Manually accessing the [`PathFinder`] API
//!
//! This contains functionality from the [`PathFinder`] object in Screeps, which
//! is itself a binding to a C++ Jump-Point Search pathfinding algorithm
//! optimized for Screeps.
//!
//! This is both more fine-grained and less automatic than other pathing
//! methods, such as [`Room::find_path_to`]. [`PathFinder`] knows about terrain
//! by default, but you must configure any other obstacles you want it to
//! consider.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#PathFinder)
//!
//! [`Room::find_path_to`]: crate::objects::Room::find_path_to

use std::convert::TryInto;

use crate::{CostMatrix, Position, RoomName, objects::RoomPosition};
use js_sys::{Array, JsString, Object};
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use serde::Serialize;
use serde_wasm_bindgen;

#[wasm_bindgen]
extern "C" {
    /// Interfaces for calling the default Screeps [`PathFinder`].
    #[wasm_bindgen]
    pub type PathFinder;

    /// Search for a path from an origin to a goal or array of goals.
    ///
    /// The goal, or each entry in the goal array if using an array, must be an
    /// object with a position and optionally a `range` key, if a target
    /// distance other than 0 is needed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.search)
    #[wasm_bindgen(static_method_of = PathFinder, js_name = search)]
    fn search_internal(
        origin: &RoomPosition,
        goal: &JsValue,
        options: &JsValue,
    ) -> SearchResults;
}

#[wasm_bindgen]
extern "C" {
    /// Object that represents a set of options for a call to
    /// [`PathFinder::search`].
    #[wasm_bindgen]
    pub type JsSearchOptions;

    /// Room callback, which should return a [`CostMatrix`], or
    /// [`JsValue::FALSE`] to avoid pathing through a room.
    #[wasm_bindgen(method, setter = roomCallback)]
    pub fn room_callback(this: &JsSearchOptions, callback: &Closure<dyn FnMut(JsString) -> JsValue>);

    /// Set the cost of moving on plains tiles during this pathfinder search.
    /// Defaults to 1.
    #[wasm_bindgen(method, setter = plainCost)]
    pub fn plain_cost(this: &JsSearchOptions, cost: u8);

    /// Set the cost of moving on swamp tiles during this pathfinder search.
    /// Defaults to 5.
    #[wasm_bindgen(method, setter = swampCost)]
    pub fn swamp_cost(this: &JsSearchOptions, cost: u8);

    /// Set whether to flee to a certain distance away from the target instead
    /// of attempting to find a path to it. Defaults to false.
    #[wasm_bindgen(method, setter = flee)]
    pub fn flee(this: &JsSearchOptions, val: bool);

    /// Set the maximum number of operations to allow the pathfinder to complete
    /// before returning an incomplete path. Defaults to 2,000.
    #[wasm_bindgen(method, setter = maxOps)]
    pub fn max_ops(this: &JsSearchOptions, ops: u32);

    /// Set the maximum number of rooms allowed to be pathed through. Defaults
    /// to 16, maximum of 64.
    #[wasm_bindgen(method, setter = maxRooms)]
    pub fn max_rooms(this: &JsSearchOptions, rooms: u8);

    /// Set the maximum total path cost allowed. No limit by default.
    #[wasm_bindgen(method, setter = maxCost)]
    pub fn max_cost(this: &JsSearchOptions, cost: f64);

    /// Heuristic weight to use for the A* algorithm to be guided toward the
    /// goal. Defaults to 1.2.
    #[wasm_bindgen(method, setter = heuristicWeight)]
    pub fn heuristic_weight(this: &JsSearchOptions, weight: f64);
}

impl JsSearchOptions {
    pub fn new() -> JsSearchOptions {
        Object::new().unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    /// An object representing the results of a [`PathFinder::search`].
    #[wasm_bindgen]
    pub type SearchResults;

    /// Get the path that was found, an [`Array`] of [`RoomPosition`]. May be
    /// incomplete.
    #[wasm_bindgen(method, getter, js_name = path)]
    fn path_internal(this: &SearchResults) -> Array;

    /// The number of operations the pathfinding operation performed.
    #[wasm_bindgen(method, getter)]
    pub fn ops(this: &SearchResults) -> u32;

    /// Total cost of all tiles used in the path
    #[wasm_bindgen(method, getter)]
    pub fn cost(this: &SearchResults) -> u32;

    /// Whether this search successfully found a complete path.
    #[wasm_bindgen(method, getter)]
    pub fn incomplete(this: &SearchResults) -> bool;
}

impl SearchResults {
    pub fn path (&self) -> Vec<Position> {
        self
            .path_internal()
            .iter()
            .map(|p| p.unchecked_into())
            .map(|p: RoomPosition| p.into())
            .collect()
    }
}

pub trait RoomCostResult: Into<JsValue> {}

pub enum MultiRoomCostResult {
    CostMatrix(CostMatrix),
    Impassable,
    Default
}

impl RoomCostResult for MultiRoomCostResult {}

impl Default for MultiRoomCostResult {
    fn default() -> Self {
        MultiRoomCostResult::Default
    }
}

impl<'a> Into<JsValue> for MultiRoomCostResult {
    fn into(self) -> JsValue {
        match self {
            MultiRoomCostResult::CostMatrix(m) => m.into(),
            MultiRoomCostResult::Impassable => JsValue::from_bool(false),
            MultiRoomCostResult::Default => JsValue::undefined()
        }
    }
}

pub enum SingleRoomCostResult {
    CostMatrix(CostMatrix),
    Default
}

impl RoomCostResult for SingleRoomCostResult {}

impl Default for SingleRoomCostResult {
    fn default() -> Self {
        SingleRoomCostResult::Default
    }
}

impl<'a> Into<JsValue> for SingleRoomCostResult {
    fn into(self) -> JsValue {
        match self {
            SingleRoomCostResult::CostMatrix(m) => m.into(),
            SingleRoomCostResult::Default => JsValue::undefined()
        }
    }
}

pub struct SearchOptions<F>
where
    F: FnMut(RoomName) -> MultiRoomCostResult,
{
    room_callback: F,
    inner: InnerSearchOptions,
}

#[derive(Default, Clone, Serialize)]
struct InnerSearchOptions {
    plain_cost: Option<u8>,
    swamp_cost: Option<u8>,
    flee: Option<bool>,
    max_ops: Option<u32>,
    max_rooms: Option<u8>,
    max_cost: Option<f64>,
    heuristic_weight: Option<f64>,
}

impl<F> SearchOptions<F> where F: FnMut(RoomName) -> MultiRoomCostResult,
{
    pub(crate) fn as_js_options<R>(self, callback: impl Fn(&JsSearchOptions) -> R) -> R {
        // Serialize the inner options into a JsValue, then cast.
        let js_options: JsSearchOptions = serde_wasm_bindgen::to_value(&self.inner).expect("Unable to serialize search options.").unchecked_into();

        let mut raw_callback = self.room_callback;

        let mut owned_callback = move |room: RoomName| -> JsValue {
            raw_callback(room).into()
        };
    
        //
        // Type erased and boxed callback: no longer a type specific to the closure
        // passed in, now unified as &Fn
        //

        let callback_type_erased: &mut (dyn FnMut(RoomName) -> JsValue) = &mut owned_callback;
    
        // Overwrite lifetime of reference so it can be passed to javascript.
        // It's now pretending to be static data. This should be entirely safe
        // because we control the only use of it and it remains valid during the
        // pathfinder callback. This transmute is necessary because "some lifetime
        // above the current scope but otherwise unknown" is not a valid lifetime.
        //

        let callback_lifetime_erased: &'static mut (dyn FnMut(RoomName) -> JsValue) = unsafe { std::mem::transmute(callback_type_erased) };    
    
        let boxed_callback = Box::new(move |room: JsString| -> JsValue {
            let room = room.try_into().expect("expected room name in room callback");

            callback_lifetime_erased(room)
        }) as Box<dyn FnMut(JsString) -> JsValue>;
    
        let closure = Closure::wrap(boxed_callback);

        js_options.room_callback(&closure);

        callback(&js_options)
    }
}

impl Default for SearchOptions<fn(RoomName) -> MultiRoomCostResult> {
    fn default() -> Self {
        fn cost_matrix(_: RoomName) -> MultiRoomCostResult {
            MultiRoomCostResult::Default
        }

        SearchOptions {
            room_callback: cost_matrix,
            inner: Default::default(),
        }
    }
}

impl<F> SearchOptions<F>
where
    F: FnMut(RoomName) -> MultiRoomCostResult,
{
    #[inline]
    pub fn new(room_callback: F) -> Self {
        SearchOptions {
            room_callback,
            inner: Default::default(),
        }
    }

    pub fn room_callback<F2>(self, room_callback: F2) -> SearchOptions<F2>
    where
        F2: FnMut(RoomName) -> MultiRoomCostResult,
    {
        SearchOptions {
            room_callback,
            inner: self.inner,
        }
    }

    /// Sets plain cost - default `1`.
    #[inline]
    pub fn plain_cost(mut self, cost: u8) -> Self {
        self.inner.plain_cost = Some(cost);
        self
    }

    /// Sets swamp cost - default `5`.
    #[inline]
    pub fn swamp_cost(mut self, cost: u8) -> Self {
        self.inner.swamp_cost = Some(cost);
        self
    }

    /// Sets whether this is a flee search - default `false`.
    #[inline]
    pub fn flee(mut self, flee: bool) -> Self {
        self.inner.flee = Some(flee);
        self
    }

    /// Sets maximum ops - default `2000`.
    #[inline]
    pub fn max_ops(mut self, ops: u32) -> Self {
        self.inner.max_ops = Some(ops);
        self
    }

    /// Sets maximum rooms - default `16`, max `16`.
    #[inline]
    pub fn max_rooms(mut self, rooms: u8) -> Self {
        self.inner.max_rooms = Some(rooms);
        self
    }

    /// Sets maximum path cost - default `f64::Infinity`.
    #[inline]
    pub fn max_cost(mut self, cost: f64) -> Self {
        self.inner.max_cost = Some(cost);
        self
    }

    /// Sets heuristic weight - default `1.2`.
    #[inline]
    pub fn heuristic_weight(mut self, weight: f64) -> Self {
        self.inner.heuristic_weight = Some(weight);
        self
    }
}

#[wasm_bindgen]
pub struct SearchGoal {
    pos: Position,
    range: u32
}

impl SearchGoal {
    pub fn new(pos: Position, range: u32) -> Self {
        SearchGoal {
            pos,
            range
        }
    }
}

#[wasm_bindgen]
impl SearchGoal {
    #[wasm_bindgen(getter)]
    pub fn pos(&self) -> RoomPosition {
        self.pos.into()
    }

    #[wasm_bindgen(getter)]
    pub fn range(&self) -> u32 {
        self.range
    }    
}

pub fn search<F>(from: Position, to: Position, range: u32, options: Option<SearchOptions<F>>) -> SearchResults where F: FnMut(RoomName) -> MultiRoomCostResult {
    let goal = SearchGoal {
        pos: to.into(),
        range
    };

    let goal = JsValue::from(goal);

    search_real(from, &goal, options)
}

pub fn search_many<F>(from: Position, to: impl Iterator<Item = SearchGoal>, options: Option<SearchOptions<F>>) -> SearchResults where F: FnMut(RoomName) -> MultiRoomCostResult {
    let goals: Array = to
        .map(|g| JsValue::from(g))
        .collect();

    search_real(from, goals.as_ref(), options)
}

fn search_real<F>(from: Position, goal: &JsValue, options: Option<SearchOptions<F>>) -> SearchResults where F: FnMut(RoomName) -> MultiRoomCostResult {
    let from = from.into();

    if let Some(options) = options {
        options.as_js_options(|js_options| {
            PathFinder::search_internal(&from, goal, &js_options)    
        })        
    } else {
        PathFinder::search_internal(&from, goal, &JsValue::UNDEFINED)
    }
}
