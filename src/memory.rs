//! Interface with Screeps' `Memory` global variable
//!
//! If you wish to access the `Memory` object stored in the javascript heap
//! which has its encoding, storage, and decoding from JSON handled by the game,
//! this allows accessing a reference to the [`ROOT`] of Memory object. Game
//! objects which have an automatic memory accessor can access references to
//! their respective parts of the object, eg.
//! [`Creep::memory`]/[`StructureSpawn::memory`]. You can work with these
//! objects using [`js_sys::Reflect`], or by converting the value into a
//! wasm_bindgen compatible type with the properly access functions you need via
//! [`wasm_bindgen::JsCast`].
//!
//! [`ROOT`]: crate::memory::ROOT
//! [`Creep::memory`]: crate::objects::Creep::memory
//! [`StructureSpawn::memory`]: crate::objects::StructureSpawn::memory
// //! # Typing
// //! Contrary to accessing the memory in javascript, rust's strong type
// system, //! requires that read values be assigned a type. To facilitate this,
// the //! `MemoryReference` provides methods to read a part of the memory as a
// //! certain type. If the value read cannot be transformed to the requested
// //! type, the method return `None`.
// //!
// //! # Accessing the memory
// //! Memory can be accessed in two ways:
// //!  - via _keys_
// //!  - via _paths_ (methods prefixed with `path_`)
// //!
// //! In both cases, if the value requested is `undefined`, `null`, or even
// just //! of the wrong type, the method returns `None`.
// //!
// //! ## Accessing memory with a _key_
// //! Since a `MemoryReference` represents a javascript object, its children
// can //! be accessed using the `object["key"]` javascript syntax using type
// methods. //! ```no_run
// //! let mem = screeps::memory::root();
// //! let cpu_used_last_tick = mem.i32("cpu_used_last_tick").unwrap();
// //! ```
// //!
// //! ## Accessing memory with a _path_
// //! A quality of life improvement upon the key access is through full path.
// In //! javascript, it is possible to query a value with a full path:
// //! ```javascript
// //! var creep_time = Memory.creeps.John.time;
// //! ```
// //!
// //! To emulate this behavior in rust, you can write such a path to a string
// and //! it will fetch the javascript object using
// //! [lodash](https://lodash.com/docs/4.17.10#get) and convert the result
// //! depending on the method used. For example,
// //! ```no_run
// //! let mem = screeps::memory::root();
// //! let creep_time = mem.path_i32("creeps.John.time").unwrap();
// //! ```
// //!
// //! # Other methods that provide `MemoryReference`s
// //! In addition to accessing the memory from the root, it is possible to
// //! access the memory via creeps, spawns, rooms and flags. Accessing the
// memory //! from those objects will also result in a `MemoryReference` which
// instead //! points at the root of this object's memory.

// use std::fmt;

// use stdweb::{JsSerialize, Reference, Value};
use js_sys::Object;
use wasm_bindgen::prelude::*;

// use crate::{
//     traits::{TryFrom, TryInto},
//     ConversionError,
// };

// #[derive(Clone, Debug)]
// pub struct UnexpectedTypeError;

// impl fmt::Display for UnexpectedTypeError {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // TODO: include &'static str references to the type names in this
// error...         write!(f, "expected one memory type, found another")
//     }
// }

// // TODO: do we even need this over just a raw 'Reference'?
// /// A [`Reference`] to a screeps memory object
// ///
// /// [`Reference`]: stdweb::Reference
// pub struct MemoryReference(Reference);

// impl AsRef<Reference> for MemoryReference {
//     fn as_ref(&self) -> &Reference {
//         &self.0
//     }
// }

// impl Default for MemoryReference {
//     fn default() -> Self {
//         Self::new()
//     }
// }

// impl MemoryReference {
//     pub fn new() -> Self {
//         js_unwrap!({})
//     }

//     /// Creates a MemoryReference from some JavaScript reference.
//     ///
//     /// Warning: `MemoryReference` is only designed to work with "plain"
//     /// JavaScript objects, and passing an array or a non-plain object
//     /// into this method probably won't be what you want. `MemoryReference`
//     /// also gives access to all properties, so if this is indeed a plain
//     /// object, all of its values should also be plain objects.
//     ///
//     /// Passing a non-plain-object reference into this function won't
//     /// invoke undefined behavior in and of itself, but other functions
//     /// can rely on `MemoryReference` being "plain".
//     pub unsafe fn from_reference_unchecked(reference: Reference) -> Self {
//         MemoryReference(reference)
//     }

//     pub fn bool(&self, key: &str) -> bool {
//         js_unwrap!(Boolean(@{self.as_ref()}[@{key}]))
//     }

//     pub fn path_bool(&self, path: &str) -> bool {
//         js_unwrap!(Boolean(_.get(@{self.as_ref()}, @{path})))
//     }

//     pub fn f64(&self, key: &str) -> Result<Option<f64>, ConversionError> {
//         (js! {
//             return (@{self.as_ref()})[@{key}];
//         })
//         .try_into()
//     }

//     pub fn path_f64(&self, path: &str) -> Result<Option<f64>,
// ConversionError> {         (js! {
//             return _.get(@{self.as_ref()}, @{path});
//         })
//         .try_into()
//     }

//     pub fn i32(&self, key: &str) -> Result<Option<i32>, ConversionError> {
//         (js! {
//             return (@{self.as_ref()})[@{key}];
//         })
//         .try_into()
//     }

//     pub fn path_i32(&self, path: &str) -> Result<Option<i32>,
// ConversionError> {         (js! {
//             return _.get(@{self.as_ref()}, @{path});
//         })
//         .try_into()
//     }

//     pub fn string(&self, key: &str) -> Result<Option<String>,
// ConversionError> {         (js! {
//             return (@{self.as_ref()})[@{key}];
//         })
//         .try_into()
//     }

//     pub fn path_string(&self, path: &str) -> Result<Option<String>,
// ConversionError> {         (js! {
//             return _.get(@{self.as_ref()}, @{path});
//         })
//         .try_into()
//     }

//     pub fn dict(&self, key: &str) -> Result<Option<MemoryReference>,
// ConversionError> {         (js! {
//             return (@{self.as_ref()})[@{key}];
//         })
//         .try_into()
//     }

//     pub fn path_dict(&self, path: &str) -> Result<Option<MemoryReference>,
// ConversionError> {         (js! {
//             return _.get(@{self.as_ref()}, @{path});
//         })
//         .try_into()
//     }

//     /// Get a dictionary value or create it if it does not exist.
//     ///
//     /// If the value exists but is a different type, this will return `None`.
//     pub fn dict_or_create(&self, key: &str) -> Result<MemoryReference,
// UnexpectedTypeError> {         (js! {
//             var map = (@{self.as_ref()});
//             var key = (@{key});
//             var value = map[key];
//             if (value === null || value === undefined) {
//                 map[key] = value = {};
//             }
//             if ((typeof value) == "object" && !_.isArray(value)) {
//                 return value;
//             } else {
//                 return null;
//             }
//         })
//         .try_into()
//         .map_err(|_| UnexpectedTypeError)
//         .map(MemoryReference)
//     }

//     pub fn keys(&self) -> Vec<String> {
//         js_unwrap!(Object.keys(@{self.as_ref()}))
//     }

//     pub fn del(&self, key: &str) {
//         js! { @(no_return)
//             (@{self.as_ref()})[@{key}] = undefined;
//         }
//     }

//     pub fn path_del(&self, path: &str) {
//         js! {
//             _.set(@{self.as_ref()}, @{path}, undefined);
//         }
//     }

//     /// Gets a custom type. Will return `None` if `null` or `undefined`, and
//     /// `Err` if incorrect type.
//     ///
//     /// # Example
//     ///
//     /// ```no_run
//     /// use log::info;
//     /// use screeps::{prelude::*, Position};
//     ///
//     /// let creep = screeps::game::creeps::get("mycreepname").unwrap();
//     /// let mem = creep.memory();
//     /// let pos = mem.get::<Position>("saved_pos").unwrap();
//     /// let pos = match pos {
//     ///     Some(pos) => {
//     ///         info!("found position: {}", pos);
//     ///         pos
//     ///     }
//     ///     None => {
//     ///         info!("no position. saving new one!");
//     ///         let pos = creep.pos();
//     ///         mem.set("saved_pos", pos);
//     ///         pos
//     ///     }
//     /// };
//     /// info!("final position: {}", pos);
//     /// creep.move_to(&pos);
//     /// ```
//     pub fn get<T>(&self, key: &str) -> Result<Option<T>, <T as
// TryFrom<Value>>::Error>     where
//         T: TryFrom<Value>,
//     {
//         let val = js! {
//             return (@{self.as_ref()})[@{key}];
//         };
//         if val == Value::Null || val == Value::Undefined {
//             Ok(None)
//         } else {
//             Some(val.try_into()).transpose()
//         }
//     }

//     /// Gets a custom type at a memory path. Will return `None` if `null` or
//     /// `undefined`, and `Err` if incorrect type.
//     ///
//     /// Uses lodash in JavaScript to evaluate the path. See https://lodash.com/docs/#get.
//     pub fn get_path<T>(&self, path: &str) -> Result<Option<T>, <T as
// TryFrom<Value>>::Error>     where
//         T: TryFrom<Value>,
//     {
//         let val = js! {
//             return _.get(@{self.as_ref()}, @{path});
//         };
//         if val == Value::Null || val == Value::Undefined {
//             Ok(None)
//         } else {
//             Some(val.try_into()).transpose()
//         }
//     }

//     pub fn set<T>(&self, key: &str, value: T)
//     where
//         T: JsSerialize,
//     {
//         js! { @(no_return)
//             (@{self.as_ref()})[@{key}] = @{value};
//         }
//     }

//     pub fn path_set<T>(&self, path: &str, value: T)
//     where
//         T: JsSerialize,
//     {
//         js! { @(no_return)
//             _.set(@{self.as_ref()}, @{path}, @{value});
//         }
//     }

//     pub fn arr<T>(&self, key: &str) -> Result<Option<Vec<T>>,
// ConversionError>     where
//         T: TryFrom<Value, Error = ConversionError>,
//     {
//         (js! {
//             return (@{self.as_ref()})[@{key}];
//         })
//         .try_into()
//     }

//     pub fn path_arr<T>(&self, path: &str) -> Result<Option<Vec<T>>,
// ConversionError>     where
//         T: TryFrom<Value, Error = ConversionError>,
//     {
//         (js! {
//             return _.get(@{self.as_ref()}, @{path});
//         })
//         .try_into()
//     }
// }

// impl TryFrom<Value> for MemoryReference {
//     type Error = ConversionError;

//     fn try_from(v: Value) -> Result<Self, Self::Error> {
//         let r: Reference = v.try_into()?; // fail early.
//         Ok(MemoryReference(
//             (js! {
//                 var v = (@{r});
//                 if (_.isArray(v)) {
//                     return null;
//                 } else {
//                     return v;
//                 }
//             })
//             .try_into()?,
//         ))
//     }
// }

#[wasm_bindgen]
extern "C" {
    /// Get a reference to the `Memory` global object. Note that this object
    /// gets recreated each tick by the Screeps engine, so references from it
    /// should not be held beyond the current tick.
    #[wasm_bindgen(js_name = Memory)]
    pub static ROOT: Object;

}
