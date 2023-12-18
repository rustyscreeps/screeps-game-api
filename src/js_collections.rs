//! Typed JavaScript collection wrappers.
use std::{
    cmp::{Eq, PartialEq},
    fmt,
    marker::PhantomData,
    str::FromStr,
};

use js_sys::{Array, JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

use crate::{game, local::RawObjectIdParseError, prelude::*};

#[wasm_bindgen]
extern "C" {
    #[derive(Clone)]
    #[wasm_bindgen(extends = Object)]
    pub(crate) type ObjectExt;

    #[wasm_bindgen(method, structural, indexing_setter)]
    pub(crate) fn set(this: &ObjectExt, prop: &str, val: &JsValue);

    #[wasm_bindgen(method, structural, indexing_setter)]
    pub(crate) fn set_value(this: &ObjectExt, prop: &JsValue, val: &JsValue);

    #[wasm_bindgen(method, structural, indexing_getter)]
    pub(crate) fn get_value(this: &ObjectExt, prop: &JsValue) -> JsValue;
}

pub trait JsCollectionIntoValue {
    fn into_value(self) -> JsValue;
}

pub trait JsCollectionFromValue {
    fn from_value(val: JsValue) -> Self;
}

/// Container holding a reference to an [`Object`] in JavaScript as well as
/// expected types for both the keys and values.
pub struct JsHashMap<K, V> {
    map: Object,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> JsHashMap<K, V>
where
    K: JsCollectionFromValue,
{
    pub fn keys(&self) -> impl Iterator<Item = K> {
        let array = Object::keys(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }
}

impl<K, V> JsHashMap<K, V>
where
    V: JsCollectionFromValue,
{
    pub fn values(&self) -> impl Iterator<Item = V> {
        let array = Object::values(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }
}

impl<K, V> JsHashMap<K, V>
where
    K: JsCollectionIntoValue,
    V: JsCollectionFromValue,
{
    pub fn get(&self, key: K) -> Option<V> {
        let key = key.into_value();
        let val = JsCast::unchecked_ref::<ObjectExt>(&self.map).get_value(&key);
        if val.is_null() || val.is_undefined() {
            return None;
        }
        let val = V::from_value(val);

        Some(val)
    }
}

impl<K, V> JsHashMap<K, V>
where
    K: JsCollectionIntoValue,
    V: JsCollectionIntoValue,
{
    pub fn set(&self, key: K, value: V) {
        let key = key.into_value();
        let value = value.into_value();
        JsCast::unchecked_ref::<ObjectExt>(&self.map).set_value(&key, &value);
    }
}

impl<K, V> From<Object> for JsHashMap<K, V> {
    fn from(map: Object) -> Self {
        Self {
            map,
            _phantom: Default::default(),
        }
    }
}

impl<K, V> From<JsValue> for JsHashMap<K, V> {
    fn from(val: JsValue) -> Self {
        Self {
            map: val.into(),
            _phantom: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedArrayIter<T> {
    range: std::ops::Range<u32>,
    array: Array,
    _phantom: PhantomData<T>,
}

impl<T> OwnedArrayIter<T> {
    pub fn new(array: Array) -> Self {
        OwnedArrayIter {
            range: 0..array.length(),
            array,
            _phantom: Default::default(),
        }
    }
}

impl<T> std::iter::Iterator for OwnedArrayIter<T>
where
    T: JsCollectionFromValue,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let index = self.range.next()?;
        let val = self.array.get(index);
        let val = T::from_value(val);
        Some(val)
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.range.size_hint()
    }
}

impl<T> std::iter::DoubleEndedIterator for OwnedArrayIter<T>
where
    T: JsCollectionFromValue,
{
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        let val = self.array.get(index);
        let val = T::from_value(val);
        Some(val)
    }
}

impl<T> std::iter::FusedIterator for OwnedArrayIter<T> where T: JsCollectionFromValue {}

impl<T> std::iter::ExactSizeIterator for OwnedArrayIter<T> where T: JsCollectionFromValue {}

/// Represents a reference to an Object ID string in JavaScript memory, typed
/// according to the object type Rust expects for the object after resolving.
///
/// Use [`ObjectId`] if a value stored in Rust memory is preferred; the
/// JavaScript representation can be harder to work with in Rust code due to
/// lack of visibility on the underlying string and lack of most trait
/// implementations, and consumes more memory, but is faster to resolve and may
/// be useful with objects you plan to resolve frequently.
///
/// This object ID is typed, but not strictly, and can be converted into
/// referring into another type of object with [`JsObjectId::into_type`].
///
/// [`ObjectId`]: crate::local::ObjectId
// Copy, Clone, Debug, PartialEq, Eq, Hash, PartialEq, Eq implemented manually
// below
pub struct JsObjectId<T> {
    pub raw: JsString,
    phantom: PhantomData<T>,
}

// traits implemented manually so they don't depend on `T` implementing them.
impl<T> Clone for JsObjectId<T> {
    fn clone(&self) -> JsObjectId<T> {
        JsObjectId {
            raw: self.raw.clone(),
            phantom: PhantomData,
        }
    }
}
impl<T> fmt::Debug for JsObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.raw.fmt(f)
    }
}
impl<T> PartialEq for JsObjectId<T> {
    fn eq(&self, o: &JsObjectId<T>) -> bool {
        self.raw.eq(&o.raw)
    }
}
impl<T> Eq for JsObjectId<T> {}

impl<T> FromStr for JsObjectId<T> {
    type Err = RawObjectIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let js_string: JsString = s.into();
        Ok(js_string.into())
    }
}

impl<T> fmt::Display for JsObjectId<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        String::from(&self.raw).fmt(f)
    }
}

impl<T> JsObjectId<T> {
    /// Changes the type this [`JsObjectId`] points to, unchecked.
    ///
    /// This will allow changing to any type - `JsObjectId` makes no guarantees
    /// about its ID matching the type of any object in the game that it
    /// actually points to.
    pub fn into_type<U>(self) -> JsObjectId<U> {
        JsObjectId {
            raw: self.raw,
            phantom: PhantomData,
        }
    }

    /// Resolves this ID into an object, assuming the type `T` is the correct
    /// type of object that this ID refers to. If the ID has been converted to
    /// an invalid type, using the returned object in a way not valid for its
    /// type will cause a panic.
    ///
    /// Will return `None` if this object no longer exists, or is in a room we
    /// don't have vision for.
    pub fn resolve(&self) -> Option<T>
    where
        T: Resolvable,
    {
        game::get_object_by_js_id_typed(self)
    }
}

impl<T> From<JsString> for JsObjectId<T> {
    fn from(raw: JsString) -> Self {
        JsObjectId {
            raw,
            phantom: PhantomData,
        }
    }
}

impl<T> From<JsObjectId<T>> for JsString {
    fn from(id: JsObjectId<T>) -> Self {
        id.raw
    }
}

impl<T> From<JsObjectId<T>> for String {
    fn from(id: JsObjectId<T>) -> Self {
        id.to_string()
    }
}

//
// Utility conversions for collections.
//
impl JsCollectionIntoValue for JsString {
    fn into_value(self) -> JsValue {
        self.unchecked_into()
    }
}

impl JsCollectionFromValue for JsString {
    fn from_value(val: JsValue) -> JsString {
        val.unchecked_into()
    }
}

impl JsCollectionIntoValue for String {
    fn into_value(self) -> JsValue {
        self.into()
    }
}

impl JsCollectionFromValue for String {
    fn from_value(val: JsValue) -> String {
        let val: JsString = val.unchecked_into();

        val.into()
    }
}

impl JsCollectionIntoValue for u8 {
    fn into_value(self) -> JsValue {
        JsValue::from_f64(self as f64)
    }
}

impl JsCollectionFromValue for u8 {
    fn from_value(val: JsValue) -> u8 {
        if let Some(val) = val.as_string() {
            val.parse::<u8>().expect("expected parseable u8 string")
        } else {
            val.as_f64().expect("expected number value") as u8
        }
    }
}
