use std::{
    cmp::Ordering,
    fmt,
    hash::{Hash, Hasher},
    marker::PhantomData,
    str::FromStr,
};

use arrayvec::ArrayString;
use js_sys::JsString;
use serde::{Deserialize, Serialize};
use wasm_bindgen::{JsCast, JsValue};

use crate::{game, js_collections::JsCollectionFromValue, objects::RoomObject, traits::MaybeHasId};

mod errors;
mod raw;

pub use errors::*;
pub use raw::*;

/// Represents an Object ID and a type that the ID points to, stored in Rust
/// memory.
///
/// Use [`JsObjectId`] if a reference stored in JavaScript memory is preferred.
///
/// Each object ID in Screeps: World is represented by an ID of up to 24
/// hexidemical characters, which cannot change. This implementation takes
/// advantage of that by storing a packed representation in a `u128`, using 96
/// bits for the ID and 32 bits for tracking the length of the ID string for
/// reconstruction in JS.
///
/// # Conversion
///
/// Use `into` to convert between `ObjectId<T>` and [`RawObjectId`], and
/// [`ObjectId::into_type`] to change the type this `ObjectId` points to freely.
///
/// # Ordering
///
/// To facilitate use as a key in a [`BTreeMap`] or other similar data
/// structures, `ObjectId` implements [`PartialOrd`] and [`Ord`].
///
/// `ObjectId`'s are ordered by the corresponding order of their underlying
/// byte values. This agrees with:
///
/// - lexicographical ordering of the object id strings
/// - JavaScript's ordering of object id strings
/// - ordering of [`RawObjectId`]s
///
/// **Note:** when running on the official screeps server, or on a private
/// server backed by a MongoDB database, this ordering roughly corresponds to
/// creation order. The first four bytes of a MongoDB-created `ObjectId` [are
/// seconds since the epoch when the id was created][1], so up to a second
/// accuracy, these ids will be sorted by object creation time.
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`JsObjectId`]: crate::js_collections::JsObjectId
/// [1]: https://docs.mongodb.com/manual/reference/method/ObjectId/
// Copy, Clone, Debug, PartialEq, Eq, Hash, PartialEq, Eq implemented manually below
#[derive(Serialize, Deserialize)]
#[serde(transparent, bound = "")]
pub struct ObjectId<T> {
    raw: RawObjectId,

    // Needed to consider the `T` as "used" even though we mostly use it as a marker. Because of
    // auto traits, `PhantomData<fn() -> T>` is used instead: this struct doesn't *hold* a `T`, it
    // *produces* one.
    #[serde(skip)]
    phantom: PhantomData<fn() -> T>,
}

// traits implemented manually so they don't depend on `T` implementing them.
impl<T> Copy for ObjectId<T> {}
impl<T> Clone for ObjectId<T> {
    fn clone(&self) -> ObjectId<T> {
        *self
    }
}
impl<T> fmt::Debug for ObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}
impl<T> PartialEq for ObjectId<T> {
    fn eq(&self, o: &ObjectId<T>) -> bool {
        self.raw.eq(&o.raw)
    }
}
impl<T> Eq for ObjectId<T> {}
impl<T> Hash for ObjectId<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.raw.hash(state)
    }
}
impl<T> PartialOrd<ObjectId<T>> for ObjectId<T> {
    #[inline]
    fn partial_cmp(&self, other: &ObjectId<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<T> Ord for ObjectId<T> {
    #[inline]
    fn cmp(&self, other: &Self) -> Ordering {
        self.raw.cmp(&other.raw)
    }
}

impl<T> FromStr for ObjectId<T> {
    type Err = RawObjectIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let raw: RawObjectId = s.parse()?;

        Ok(raw.into())
    }
}

impl<T> fmt::Display for ObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}

impl<T> ObjectId<T> {
    /// Changes the type this [`ObjectId`] points to, unchecked.
    ///
    /// This will allow changing to any type - `ObjectId` makes no guarantees
    /// about its ID matching the type of any object in the game that it
    /// actually points to.
    pub fn into_type<U>(self) -> ObjectId<U> {
        RawObjectId::from(self).into()
    }

    /// Creates an object ID from its packed representation.
    ///
    /// The input to this function is the bytes representing the up-to-24 hex
    /// digits in the object id.
    ///
    /// See also [`RawObjectId::from_packed`].
    pub fn from_packed(packed: u128) -> Self {
        RawObjectId::from_packed(packed).into()
    }

    /// Converts this object ID to a `u128` number.
    ///
    /// The returned number, when formatted as hex, will produce a string
    /// parseable into this object id.
    ///
    /// The returned number will be less than or equal to `2^96 - 1`, as that's
    /// the maximum value that `RawObjectId` can hold.
    pub fn to_u128(self) -> u128 {
        self.raw.into()
    }

    /// Formats this object ID as a string on the stack.
    ///
    /// This is equivalent to [`ToString::to_string`], but involves no
    /// allocation.
    ///
    /// See also [`RawObjectId::to_array_string`].
    pub fn to_array_string(&self) -> ArrayString<24> {
        self.raw.to_array_string()
    }

    /// Resolves this object ID into an object, verifying that the returned
    /// object matches the expected type.
    ///
    /// # Errors
    ///
    /// Will return an error if this ID's type does not match the object it
    /// points to with the resolved [`RoomObject`] with an unknown type.
    ///
    /// Will return `Ok(None)` if the object no longer exists, or is in a room
    /// we don't have vision for.
    pub fn try_resolve(self) -> Result<Option<T>, RoomObject>
    where
        T: MaybeHasId + JsCast,
    {
        match game::get_object_by_id_erased(&self.raw) {
            Some(v) => v.dyn_into().map(|v| Some(v)),
            None => Ok(None),
        }
    }

    /// Resolves this ID into an object, assuming the type `T` is the correct
    /// type of object that this ID refers to. If the ID has been converted to
    /// an invalid type, using the returned object in a way not valid for its
    /// type will cause a panic.
    ///
    /// Will return `None` if this object no longer exists, or is in a room we
    /// don't have vision for.
    pub fn resolve(self) -> Option<T>
    where
        T: MaybeHasId + JsCast,
    {
        game::get_object_by_id_typed(&self)
    }
}

impl<T> PartialEq<RawObjectId> for ObjectId<T> {
    #[inline]
    fn eq(&self, other: &RawObjectId) -> bool {
        self.raw == *other
    }
}

impl<T> PartialEq<ObjectId<T>> for RawObjectId {
    #[inline]
    fn eq(&self, other: &ObjectId<T>) -> bool {
        *self == other.raw
    }
}

impl<T> PartialOrd<RawObjectId> for ObjectId<T> {
    #[inline]
    fn partial_cmp(&self, other: &RawObjectId) -> Option<Ordering> {
        Some(self.raw.cmp(other))
    }
}

impl<T> PartialOrd<ObjectId<T>> for RawObjectId {
    #[inline]
    fn partial_cmp(&self, other: &ObjectId<T>) -> Option<Ordering> {
        Some(self.cmp(&other.raw))
    }
}

impl<T> From<RawObjectId> for ObjectId<T> {
    fn from(raw: RawObjectId) -> Self {
        ObjectId {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> From<ObjectId<T>> for RawObjectId {
    fn from(id: ObjectId<T>) -> Self {
        id.raw
    }
}

impl<T> From<ObjectId<T>> for ArrayString<24> {
    fn from(id: ObjectId<T>) -> Self {
        id.to_array_string()
    }
}

impl<T> From<ObjectId<T>> for String {
    fn from(id: ObjectId<T>) -> Self {
        id.to_string()
    }
}

impl<T> From<ObjectId<T>> for u128 {
    fn from(id: ObjectId<T>) -> Self {
        id.raw.into()
    }
}

impl<T> From<u128> for ObjectId<T> {
    fn from(packed: u128) -> Self {
        Self::from_packed(packed)
    }
}

impl<T> JsCollectionFromValue for ObjectId<T> {
    fn from_value(val: JsValue) -> Self {
        let val: JsString = val.unchecked_into();
        let val: String = val.into();
        val.parse().expect("valid id string")
    }
}
