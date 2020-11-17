//! Manually accessing the [`PathFinder`] API
//!
//! This contains functionality from the [`PathFinder`] object in Screeps, which
//! is itself a binding to a C++ Jump-Point Search pathfinding algorithm
//! optimized for Screeps.
//!
//! This is both more fine-grained and less automatic than other pathing
//! methods, such as [`Room::find_path_to`]. [`PathFinder`] knows about terrain by
//! default, but you must configure any other obstacles you want it to consider.
//!
//! [Screeps documentation](https://docs.screeps.com/api/#PathFinder)
//!
//! [`Room::find_path_to`]: crate::objects::Room::find_path_to

use crate::objects::RoomPosition;
use wasm_bindgen::prelude::*;
use js_sys::{Array, JsString};

#[wasm_bindgen]
extern "C" {
    /// Interfaces for calling the default Screeps [`PathFinder`].
    #[wasm_bindgen]
    pub type PathFinder;

    /// Search for a path from an origin to a goal or array of goals.
    ///
    /// The goal, or each entry in the goal array if using an array, must be an object with a position and optionally a `range` key, if a target distance other than 0 is needed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PathFinder.search)
    #[wasm_bindgen(static_method_of = PathFinder)]
    pub fn search(origin: &RoomPosition, goal: &JsValue, options: Option<&SearchOptions>) -> SearchResults;
}

#[wasm_bindgen]
extern "C" {
    /// Object that represents a set of options for a call to [`PathFinder::search`].
    #[wasm_bindgen]
    pub type SearchOptions;

    /// Room callback, which should return a [`CostMatrix`], or [`JsValue::FALSE`] to avoid pathing through a room.
    #[wasm_bindgen(method, setter = roomCallback)]
    pub fn room_callback(this: &SearchOptions, callback: &Closure<dyn FnMut(JsString) -> JsValue>);

    /// Set the cost of moving on plains tiles during this pathfinder search. Defaults to 1.
    #[wasm_bindgen(method, setter = plainCost)]
    pub fn plain_cost(this: &SearchOptions, cost: u8);

    /// Set the cost of moving on swamp tiles during this pathfinder search. Defaults to 5.
    #[wasm_bindgen(method, setter = swampCost)]
    pub fn swamp_cost(this: &SearchOptions, cost: u8);

    /// Set whether to flee to a certain distance away from the target instead of attempting to find a path to it. Defaults to false.
    #[wasm_bindgen(method, setter = flee)]
    pub fn flee(this: &SearchOptions, val: bool);

    /// Set the maximum number of operations to allow the pathfinder to complete before returning an incomplete path. Defaults to 2,000.
    #[wasm_bindgen(method, setter = maxOps)]
    pub fn max_ops(this: &SearchOptions, ops: u32);

    /// Set the maximum number of rooms allowed to be pathed through. Defaults to 16, maximum of 64.
    #[wasm_bindgen(method, setter = maxRooms)]
    pub fn max_rooms(this: &SearchOptions, rooms: u8);

    /// Set the maximum total path cost allowed. No limit by default.
    #[wasm_bindgen(method, setter = maxCost)]
    pub fn max_cost(this: &SearchOptions, cost: u32);

    /// Heuristic weight to use for the A* algorithm to be guided toward the goal. Defaults to 1.2.
    #[wasm_bindgen(method, setter = heuristicWeight)]
    pub fn heuristic_weight(this: &SearchOptions, weight: u32);
}

#[wasm_bindgen]
extern "C" {
    /// An object representing the results of a [`PathFinder::search`].
    #[wasm_bindgen]
    pub type SearchResults;

    /// Get the path that was found, an [`Array`] of [`RoomPosition`]. May be incomplete.
    #[wasm_bindgen(method, getter)]
    pub fn path(this: &SearchResults) -> Array;

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


// use std::{f64, marker::PhantomData, mem, borrow::{Borrow}};

// use stdweb::{web::TypedArray, Array, Object, Reference, UnsafeTypedArray, Value};

// use crate::{local::Position, objects::HasPosition, traits::TryInto, RoomName};

// #[derive(Clone, Debug)]
// pub struct LocalCostMatrix {
//     /// Length should be 2500.
//     bits: Vec<u8>,
// }

// #[inline]
// fn pos_as_idx(x: u8, y: u8) -> usize {
//     (x as usize) * 50 + (y as usize)
// }

// impl Default for LocalCostMatrix {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl LocalCostMatrix {
//     #[inline]
//     pub fn new() -> Self {
//         LocalCostMatrix {
//             bits: vec![0; 2500],
//         }
//     }

//     #[inline]
//     pub fn set(&mut self, x: u8, y: u8, val: u8) {
//         self.bits[pos_as_idx(x, y)] = val;
//     }

//     #[inline]
//     pub fn get(&self, x: u8, y: u8) -> u8 {
//         self.bits[pos_as_idx(x, y)]
//     }

//     /// Copies all data into an JavaScript CostMatrix for use.
//     ///
//     /// This is slower than [`as_uploaded`], but much safer.
//     ///
//     /// [`as_uploaded`]: #method.as_uploaded
//     pub fn upload(&self) -> CostMatrix<'static> {
//         let bits: TypedArray<u8> = self.bits[..].into();

//         CostMatrix {
//             inner: (js! {
//                 var matrix = Object.create(PathFinder.CostMatrix.prototype);
//                 matrix._bits = @{bits};
//                 return matrix;
//             })
//             .try_into()
//             .expect("expected function returning CostMatrix to return a Reference"),
//             lifetime: PhantomData,
//         }
//     }

//     /// Temporarily exposes the bits of this matrix as a cost matrix.
//     ///
//     /// # Unsafety
//     ///
//     /// There are two main invariants you must uphold after using this function:
//     ///
//     /// 1. The `CostMatrix` can only be used in JS code as long as this
//     /// `LocalCostMatrix` is alive.    Doing otherwise will result in
//     /// undefined behavior, mainly JS being allowed to read/    manipulate
//     /// uninitialized rust memory or rust memory that's been repurposed.
//     ///
//     /// 2. The `set` method of the cost matrix must not be used - it must be
//     /// read only. This takes    &self, but technically allows mutation of
//     /// the inner Vec via JavaScript access. You    should not use this
//     /// method, or you will invoke Rust undefined behavior.
//     ///
//     /// The CostMatrix returned will _reference the internal data of this
//     /// `LocalCostMatrix`_.
//     pub unsafe fn as_uploaded<'a>(&'a self) -> CostMatrix<'a> {
//         let bits: UnsafeTypedArray<'_, u8> = UnsafeTypedArray::new(&self.bits);

//         CostMatrix {
//             inner: (js! {
//                 // using this first is necessary in order to uphold the invariant of
//                 // `UnsafeTypedArray`.
//                 var bits = @{bits};

//                 var matrix = Object.create(PathFinder.CostMatrix.prototype);
//                 matrix._bits = bits;

//                 return matrix;
//             })
//             .try_into()
//             .expect("expected function returning CostMatrix to return a Reference"),
//             lifetime: PhantomData,
//         }
//     }
// }

// impl Into<Vec<u8>> for LocalCostMatrix {
//     /// Returns a vector of bits length 2500, where each position is
//     /// `idx = ((x * 50) + y)`.
//     #[inline]
//     fn into(self) -> Vec<u8> {
//         self.bits
//     }
// }

// impl<'a> CostMatrixSet for LocalCostMatrix {
//     fn set_multi<D, B, P, V>(&mut self, data: D) where D: IntoIterator<Item = B>, B: Borrow<(P, V)>, P: HasLocalPosition, V: Borrow<u8> {
//         let iter = data.into_iter();

//         for entry in iter {
//             let (pos, cost) = entry.borrow();
            
//             self.set(pos.x(), pos.y(), *cost.borrow());
//         }
//     }
// }

// /// A `CostMatrix` that's valid to pass as a result from a `PathFinder.search`
// /// room callback.
// ///
// /// Lives as long as `'a` lifetime. It's unsound to leak to JS past this
// /// lifetime if this matrix was created by [`LocalCostMatrix::as_uploaded`].
// ///
// /// [`LocalCostMatrix::as_uploaded`]:
// /// struct.LocalCostMatrix.html#method.as_uploaded
// pub struct CostMatrix<'a> {
//     pub(crate) inner: Reference,
//     pub(crate) lifetime: PhantomData<&'a ()>,
// }

// impl Default for CostMatrix<'static> {
//     fn default() -> Self {
//         CostMatrix {
//             inner: js_unwrap!(new PathFinder.CostMatrix()),
//             lifetime: PhantomData,
//         }
//     }
// }

// impl<'a> Into<MultiRoomCostResult<'a>> for CostMatrix<'a> {
//     fn into(self) -> MultiRoomCostResult<'a> {
//         MultiRoomCostResult::CostMatrix(self)
//     }
// }

// impl<'a> Into<SingleRoomCostResult<'a>> for CostMatrix<'a> {
//     fn into(self) -> SingleRoomCostResult<'a> {
//         SingleRoomCostResult::CostMatrix(self)
//     }
// }

// pub trait HasLocalPosition {
//     fn x(&self) -> u8;
//     fn y(&self) -> u8;
// }

// pub trait CostMatrixSet {
//     fn set<P, V>(&mut self, position: P, cost: V) where P: HasLocalPosition, V: Borrow<u8> {
//         self.set_multi(&[(position, cost)])
//     }

//     fn set_multi<D, B, P, V>(&mut self, data: D) where D: IntoIterator<Item = B>, B: Borrow<(P, V)>, P: HasLocalPosition, V: Borrow<u8>;
// }

// impl<'a> CostMatrixSet for CostMatrix<'a> {
//     fn set_multi<D, B, P, V>(&mut self, data: D) where D: IntoIterator<Item = B>, B: Borrow<(P, V)>, P: HasLocalPosition, V: Borrow<u8> {
//         let iter = data.into_iter();
//         let (minimum_size, _maximum_size) = iter.size_hint();
//         let mut storage: Vec<u8> = Vec::with_capacity(minimum_size * 3);

//         for entry in iter {
//             let (pos, cost) = entry.borrow();
//             storage.push(pos.x());
//             storage.push(pos.y());
//             storage.push(*cost.borrow());
//         }

//         let bits: TypedArray<u8> = storage.as_slice().into();

//         js!(
//             let matrix = @{&self.inner};
//             let raw_data = @{bits};

//             const element_count = raw_data.length / 3;

//             for (let index = 0; index < element_count; ++index) {
//                 const offset = index * 3;

//                 matrix.set(raw_data[offset + 0], raw_data[offset + 1], raw_data[offset + 2]);
//             }
//         );
//     }
// }

// // need custom implementation in order to ensure length of 'bits' is always 2500
// mod serde_impls {
//     use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

//     use super::LocalCostMatrix;

//     impl Serialize for LocalCostMatrix {
//         fn serialize<S>(&self, s: S) -> Result<S::Ok, S::Error>
//         where
//             S: Serializer,
//         {
//             self.bits.serialize(s)
//         }
//     }

//     impl<'de> Deserialize<'de> for LocalCostMatrix {
//         fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
//         where
//             D: Deserializer<'de>,
//         {
//             let bits: Vec<u8> = Vec::deserialize(deserializer)?;

//             if bits.len() != 2500 {
//                 return Err(D::Error::invalid_length(
//                     bits.len(),
//                     &"a vec of length 2500",
//                 ));
//             }

//             Ok(LocalCostMatrix { bits })
//         }
//     }
// }

// pub trait RoomCostResult: Into<Value> {}

// pub enum MultiRoomCostResult<'a> {
//     CostMatrix(CostMatrix<'a>),
//     Impassable,
//     Default
// }

// impl<'a> RoomCostResult for MultiRoomCostResult<'a> {}

// impl<'a> Default for MultiRoomCostResult<'a> {
//     fn default() -> Self {
//         MultiRoomCostResult::Default
//     }
// }

// impl<'a> Into<Value> for MultiRoomCostResult<'a> {
//     fn into(self) -> Value {
//         match self {
//             MultiRoomCostResult::CostMatrix(m) => m.inner.into(),
//             MultiRoomCostResult::Impassable => Value::Bool(false),
//             MultiRoomCostResult::Default => Value::Undefined
//         }
//     }
// }

// pub enum SingleRoomCostResult<'a> {
//     CostMatrix(CostMatrix<'a>),
//     Default
// }

// impl<'a> RoomCostResult for SingleRoomCostResult<'a> {}

// impl<'a> Default for SingleRoomCostResult<'a> {
//     fn default() -> Self {
//         SingleRoomCostResult::Default
//     }
// }

// impl<'a> Into<Value> for SingleRoomCostResult<'a> {
//     fn into(self) -> Value {
//         match self {
//             SingleRoomCostResult::CostMatrix(m) => m.inner.into(),
//             SingleRoomCostResult::Default => Value::Undefined
//         }
//     }
// }

// pub struct SearchOptions<'a, F>
// where
//     F: FnMut(RoomName) -> MultiRoomCostResult<'a>,
// {
//     room_callback: F,
//     plain_cost: u8,
//     swamp_cost: u8,
//     flee: bool,
//     max_ops: u32,
//     max_rooms: u32,
//     max_cost: f64,
//     heuristic_weight: f64,
// }

// impl SearchOptions<'static, fn(RoomName) -> MultiRoomCostResult<'static>> {
//     /// Creates default SearchOptions
//     #[inline]
//     pub fn new() -> Self {
//         Self::default()
//     }
// }

// impl<'a, F> SearchOptions<'a, F>
// where
//     F: FnMut(RoomName) -> MultiRoomCostResult<'a>,
// {
//     /// Sets room callback - default `|_| { CostMatrix::default() }`.
//     pub fn room_callback<'b, F2>(self, room_callback: F2) -> SearchOptions<'b, F2>
//     where
//         F2: FnMut(RoomName) -> MultiRoomCostResult<'b>,
//     {
//         let SearchOptions {
//             room_callback: _,
//             plain_cost,
//             swamp_cost,
//             flee,
//             max_ops,
//             max_rooms,
//             max_cost,
//             heuristic_weight,
//         } = self;
//         SearchOptions {
//             room_callback,
//             plain_cost,
//             swamp_cost,
//             flee,
//             max_ops,
//             max_rooms,
//             max_cost,
//             heuristic_weight,
//         }
//     }

//     /// Sets plain cost - default `1`.
//     #[inline]
//     pub fn plain_cost(mut self, cost: u8) -> Self {
//         self.plain_cost = cost;
//         self
//     }

//     /// Sets swamp cost - default `5`.
//     #[inline]
//     pub fn swamp_cost(mut self, cost: u8) -> Self {
//         self.swamp_cost = cost;
//         self
//     }

//     /// Sets whether this is a flee search - default `false`.
//     #[inline]
//     pub fn flee(mut self, flee: bool) -> Self {
//         self.flee = flee;
//         self
//     }

//     /// Sets maximum ops - default `2000`.
//     #[inline]
//     pub fn max_ops(mut self, ops: u32) -> Self {
//         self.max_ops = ops;
//         self
//     }

//     /// Sets maximum rooms - default `16`, max `16`.
//     #[inline]
//     pub fn max_rooms(mut self, rooms: u32) -> Self {
//         self.max_rooms = rooms;
//         self
//     }

//     /// Sets maximum path cost - default `f64::Infinity`.
//     #[inline]
//     pub fn max_cost(mut self, cost: f64) -> Self {
//         self.max_cost = cost;
//         self
//     }

//     /// Sets heuristic weight - default `1.2`.
//     #[inline]
//     pub fn heuristic_weight(mut self, weight: f64) -> Self {
//         self.heuristic_weight = weight;
//         self
//     }
// }

// pub struct SearchResults {
//     path: Array,
//     pub ops: u32,
//     pub cost: u32,
//     pub incomplete: bool,
// }

// impl SearchResults {
//     #[inline]
//     pub fn opaque_path(&self) -> &Array {
//         &self.path
//     }
//     pub fn load_local_path(&self) -> Vec<Position> {
//         self.path
//             .clone()
//             .try_into()
//             .expect("expected PathFinder.search path result to be an array of RoomPositions")
//     }
// }

// /// Searches between a single origin and single goal.
// pub fn search<'a, O, G, F>(
//     origin: &O,
//     goal: &G,
//     range: u32,
//     opts: SearchOptions<'a, F>,
// ) -> SearchResults
// where
//     O: ?Sized + HasPosition,
//     G: ?Sized + HasPosition,
//     F: FnMut(RoomName) -> MultiRoomCostResult<'a> + 'a,
// {
//     let pos = goal.pos();
//     search_real(
//         origin.pos(),
//         &js_unwrap!({pos: pos_from_packed(@{pos.packed_repr()}), range: @{range}}),
//         opts,
//     )
// }

// /// Searches between a single origin and multiple goals.
// pub fn search_many<'a, O, G, I, F>(origin: &O, goal: G, opts: SearchOptions<'a, F>) -> SearchResults
// where
//     O: HasPosition,
//     G: IntoIterator<Item = (I, u32)>,
//     I: HasPosition,
//     F: FnMut(RoomName) -> MultiRoomCostResult<'a> + 'a,
// {
//     let goals: Vec<Object> = goal
//         .into_iter()
//         .map(|(target, range)| {
//             let pos = target.pos();
//             js_unwrap!({pos: pos_from_packed(@{pos.packed_repr()}), range: @{range}})
//         })
//         .collect();
//     if goals.is_empty() {
//         return SearchResults {
//             cost: 0,
//             incomplete: true,
//             ops: 0,
//             path: js_unwrap!([]),
//         };
//     }
//     let goals_js: Reference = js_unwrap!(@{goals});
//     search_real(origin.pos(), &goals_js, opts)
// }

// fn search_real<'a, F>(
//     origin: Position,
//     goal: &Reference,
//     opts: SearchOptions<'a, F>,
// ) -> SearchResults
// where
//     F: FnMut(RoomName) -> MultiRoomCostResult<'a> + 'a,
// {       
//     let SearchOptions {
//         plain_cost,
//         swamp_cost,
//         flee,
//         max_ops,
//         max_rooms,
//         heuristic_weight,
//         max_cost,
//         ..
//     } = opts;

//     let mut raw_callback = opts.room_callback;

//     let mut callback_boxed = move |room_name: RoomName| -> Value {
//         raw_callback(room_name).into()
//     };

//     // Type erased and boxed callback: no longer a type specific to the closure
//     // passed in, now unified as &Fn
//     let callback_type_erased: &mut (dyn FnMut(RoomName) -> Value + 'a) =
//         &mut callback_boxed;

//     // Overwrite lifetime of reference so it can be passed to javascript. 
//     // It's now pretending to be static data. This should be entirely safe
//     // because we control the only use of it and it remains valid during the
//     // pathfinder callback. This transmute is necessary because "some lifetime 
//     // above the current scope but otherwise unknown" is not a valid lifetime.
//     let callback_lifetime_erased: &'static mut dyn FnMut(RoomName) -> Value =
//         unsafe { mem::transmute(callback_type_erased) };

//     let res: ::stdweb::Reference = js!(
//         let cb = @{callback_lifetime_erased};
//         let res = PathFinder.search(pos_from_packed(@{origin.packed_repr()}), @{goal}, {
//             roomCallback: cb,
//             plainCost: @{plain_cost},
//             swampCost: @{swamp_cost},
//             flee: @{flee},
//             maxOps: @{max_ops},
//             maxRooms: @{max_rooms},
//             maxCost: @{max_cost},
//             heuristicWeight: @{heuristic_weight}
//         });
//         cb.drop();
//         return res;
//     )
//     .try_into()
//     .expect("expected reference from search");

//     SearchResults {
//         path: js_unwrap!(@{&res}.path),
//         ops: js_unwrap!(@{&res}.ops),
//         cost: js_unwrap!(@{&res}.cost),
//         incomplete: js_unwrap!(@{&res}.incomplete),
//     }
// }
