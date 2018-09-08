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
//! Memory is accessed via _paths_. Those are directly transcribed into a 
//! javascript object path using [lodash](https://lodash.com/docs/4.17.10#get).
//! For example, to access a number recorded at 
//! `Memory.last_tick.cpu_usage`,
//! you need
//! ```
//! let last_tick_cpu_usage = root().num("last_tick.cpu_usage").unwrap();
//! ```
//! 
//! In addition to accessing the memory from the root, it is possible to
//! access the memory via creeps, spawns, rooms and flags. Accessing the memory
//! from those objects will also result in a `MemoryReference` which instead
//! points at the root of this object's memory.
//! 

use std::fmt;
use stdweb::unstable::{TryFrom, TryInto};
use stdweb::{Array, JsSerialize, Reference, Value};

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

    pub fn bool(&self, path: &str) -> bool {
        js_unwrap!(Boolean(_.get(@{self.as_ref()}, @{path})))
    }

    pub fn num(&self, path: &str) -> Option<f64> {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
            .map(Some)
            .unwrap_or_default()
    }

    pub fn int(&self, path: &str) -> Option<i32> {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
            .map(Some)
            .unwrap_or_default()
    }

    pub fn string(&self, path: &str) -> Option<String> {
        (js! {
            return _.get(@{self.as_ref()}, @{path});
        }).try_into()
            .map(Some)
            .unwrap_or_default()
    }

    pub fn dict(&self, path: &str) -> Option<MemoryReference> {
        (js! {
            var v = _.get(@{self.as_ref()}, @{path});
            if (_.isArray(v)) {
                return null;
            } else {
                return v || null;
            }
        }).try_into()
            .map(Some)
            .unwrap_or_default()
            .map(MemoryReference)
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
        })
            .try_into()
            .map_err(|_| UnexpectedTypeError)
            .map(MemoryReference)
    }

    pub fn keys(&self) -> Vec<String> {
        js_unwrap!(Object.keys(@{self.as_ref()}))
    }

    pub fn del(&self, path: &str) {
        js! {
            _.set(@{self.as_ref()}, @{path}, undefined);
        }
    }

    pub fn set<T>(&self, path: &str, value: T)
    where
        T: JsSerialize,
    {
        js! {
            _.set(@{self.as_ref()}, @{path}, @{value});
        }
    }

    pub fn arr<T>(&self, path: &str) -> Option<Vec<T>>
    where
        T: TryFrom<Value, Error = <Reference as TryFrom<Value>>::Error>,
    {
        let x: Reference = (js! {
            var v = _.get(@{self.as_ref()}, @{path});
            if (!_.isArray(v)) {
                return null;
            } else {
                return v || null;
            }
        }).try_into()
            .ok()?;

        // Memory arrays don't have the regular Array as their prototype - they
        // have the 'outside' type.
        let as_arr: Array = unsafe {
            use stdweb::ReferenceType;
            Array::from_reference_unchecked(x)
        };

        as_arr.try_into().ok()
    }
}

impl TryFrom<Value> for MemoryReference {
    type Error = <Reference as TryFrom<Value>>::Error;

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
