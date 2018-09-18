//! Interface with Screeps' `Memory` global variable
//!
//! Screeps' memory lives in the javascript `Memory` global variable and is
//! encoded as a javascript object. This object's reference is tracked within
//! rust as a `MemoryReference`. The [`root`] function gives access to a
//! reference to the `Memory` global object.
//!
//! # Typing
//! Contrary to accessing the memory in javascript, rust's strong type system,
//! requires that read values be assigned a type. To facilitate this, the
//! `MemoryReference` provides methods to read a part of the memory as a
//! certain type. If the value read cannot be transformed to the requested
//! type, the method return `None`.
//!
//! # Accessing the memory
//! Memory can be accessed in two ways:
//!  - via _keys_
//!  - via _paths_ (methods prefixed with `path_`)
//!
//! In both cases, if the value requested is `undefined`, `null`, or even just
//! of the wrong type, the method returns `None`.
//!
//! ## Accessing memory with a _key_
//! Since a `MemoryReference` represents a javascript object, its children can
//! be accessed using the `object["key"]` javascript syntax using type methods.
//! ```no_run
//! let mem = screeps::memory::root();
//! let cpu_used_last_tick = mem.int("cpu_used_last_tick").unwrap();
//! ```
//!
//! ## Accessing memory with a _path_
//! A quality of life improvement upon the key access is through full path. In
//! javascript, it is possible to query a value with a full path:
//! ```javascript
//! var creep_time = Memory.creeps.John.time;
//! ```
//!
//! To emulate this behavior in rust, you can write such a path to a string and
//! it will fetch the javascript object using
//! [lodash](https://lodash.com/docs/4.17.10#get) and convert the result
//! depending on the method used. For example,
//! ```no_run
//! let mem = screeps::memory::root();
//! let creep_time = mem.path_num("creeps.John.time").unwrap();
//! ```
//!
//! # Other methods that provide `MemoryReference`s
//! In addition to accessing the memory from the root, it is possible to
//! access the memory via creeps, spawns, rooms and flags. Accessing the memory
//! from those objects will also result in a `MemoryReference` which instead
//! points at the root of this object's memory.
//!

use std::fmt;
use stdweb::{JsSerialize, Reference, Value};

use {
    traits::{TryFrom, TryInto},
    ConversionError,
};

#[derive(Clone, Debug)]
pub struct UnexpectedTypeError;

impl fmt::Display for UnexpectedTypeError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // TODO: include &'static str references to the type names in this error...
        write!(f, "expected one memory type, found another")
    }
}

// TODO: do we even need this over just a raw 'Reference'?
/// A [`Reference`] to a screeps memory object
pub struct MemoryReference(Reference);

impl AsRef<Reference> for MemoryReference {
    fn as_ref(&self) -> &Reference {
        &self.0
    }
}

impl MemoryReference {
    pub fn new() -> Self {
        js_unwrap!({})
    }

    /// Creates a MemoryReference from some JavaScript reference.
    ///
    /// Warning: `MemoryReference` is only designed to work with "plain"
    /// JavaScript objects, and passing an array or a non-plain object
    /// into this method probably won't be what you want. `MemoryReference`
    /// also gives access to all properties, so if this is indeed a plain
    /// object, all of its values should also be plain objects.
    ///
    /// Passing a non-plain-object reference into this function won't
    /// invoke undefined behavior in and of itself, but other functions
    /// can rely on `MemoryReference` being "plain".
    pub unsafe fn from_reference_unchecked(reference: Reference) -> Self {
        MemoryReference(reference)
    }

    pub fn bool(&self, key: &str) -> bool {
        js_unwrap!(Boolean(@{self.as_ref()}[@{key}]))
    }

    pub fn path_bool(&self, path: &str) -> bool {
        js_unwrap!(Boolean(_.get(@{self.as_ref()}, @{path})))
    }

    pub fn f64(&self, key: &str) -> Result<Option<f64>, ConversionError> {
        (js! {
            return (@{self.as_ref()})[@{key}];
        }).try_into()
    }

    pub fn path_f64(&self, path: &str) -> Result<Option<f64>, ConversionError> {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
    }

    pub fn i32(&self, key: &str) -> Result<Option<i32>, ConversionError> {
        (js! {
            return (@{self.as_ref()})[@{key}];
        }).try_into()
    }

    pub fn path_i32(&self, path: &str) -> Result<Option<i32>, ConversionError> {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
    }

    pub fn string(&self, key: &str) -> Result<Option<String>, ConversionError> {
        (js! {
            return (@{self.as_ref()})[@{key}];
        }).try_into()
    }

    pub fn path_string(&self, path: &str) -> Result<Option<String>, ConversionError> {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
    }

    pub fn dict(&self, key: &str) -> Result<Option<MemoryReference>, ConversionError> {
        (js! {
            return (@{self.as_ref()})[@{key}];
        }).try_into()
    }

    pub fn path_dict(&self, path: &str) -> Result<Option<MemoryReference>, ConversionError> {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
    }

    /// Get a dictionary value or create it if it does not exist.
    ///
    /// If the value exists but is a different type, this will return `None`.
    pub fn dict_or_create(&self, key: &str) -> Result<MemoryReference, UnexpectedTypeError> {
        (js!{
            var map = (@{self.as_ref()});
            var key = (@{key});
            var value = map[key];
            if (value === null || value === undefined) {
                map[key] = value = {};
            }
            if ((typeof value) == "object" && !_.isArray(value)) {
                return value;
            } else {
                return null;
            }
        }).try_into()
        .map_err(|_| UnexpectedTypeError)
        .map(MemoryReference)
    }

    pub fn keys(&self) -> Vec<String> {
        js_unwrap!(Object.keys(@{self.as_ref()}))
    }

    pub fn del(&self, key: &str) {
        js! {
            (@{self.as_ref()})[@{key}] = undefined;
        }
    }

    pub fn path_del(&self, path: &str) {
        js! {
            _.set(@{self.as_ref()}, @{path}, undefined);
        }
    }

    pub fn set<T>(&self, key: &str, value: T)
    where
        T: JsSerialize,
    {
        js! {
            (@{self.as_ref()})[@{key}] = @{value};
        }
    }

    pub fn path_set<T>(&self, path: &str, value: T)
    where
        T: JsSerialize,
    {
        js! {
            _.set(@{self.as_ref()}, @{path}, @{value});
        }
    }

    pub fn arr<T>(&self, key: &str) -> Result<Option<Vec<T>>, ConversionError>
    where
        T: TryFrom<Value, Error = ConversionError>,
    {
        (js! {
            return (@{self.as_ref()})[@{key}];
        }).try_into()
    }

    pub fn path_arr<T>(&self, path: &str) -> Result<Option<Vec<T>>, ConversionError>
    where
        T: TryFrom<Value, Error = ConversionError>,
    {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
    }
}

impl TryFrom<Value> for MemoryReference {
    type Error = ConversionError;

    fn try_from(v: Value) -> Result<Self, Self::Error> {
        let r: Reference = v.try_into()?; // fail early.
        Ok(MemoryReference(
            (js! {
                var v = (@{r});
                if (_.isArray(v)) {
                    return null;
                } else {
                    return v;
                }
            }).try_into()?,
        ))
    }
}

/// Get a reference to the `Memory` global object
pub fn root() -> MemoryReference {
    js_unwrap!(Memory)
}
