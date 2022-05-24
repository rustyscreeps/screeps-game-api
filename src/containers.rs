//! The main interface to objects in the Screeps game world.
//!
//! This contains all functionality from the [`Game`] object in Screeps. That
//! generally means all state which is true this tick throughout the world.
//!
//! [Screeps documentation](http://docs.screeps.com/api/#Game)

use std::marker::PhantomData;

use js_sys::{Array, JsString, Object};

use wasm_bindgen::{
    JsCast,
    prelude::*
};

pub trait JsContainerIntoValue {
    fn into_value(self) -> JsValue;
}

pub trait JsContainerFromValue {
    fn from_value(val: JsValue) -> Self;
}

pub struct JsHashMap<K, V> {
    map: Object,
    _phantom: PhantomData<(K, V)>
}

impl<K, V> JsHashMap<K, V> where K: JsContainerFromValue {
    pub fn keys(&self) -> impl Iterator<Item = K> {
        let array = Object::keys(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }  
}

impl<K, V> JsHashMap<K, V> where V: JsContainerFromValue {
    pub fn values(&self) -> impl Iterator<Item = V> {
        let array = Object::values(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }  
}

impl<K, V> JsHashMap<K, V> where K: JsContainerIntoValue, V: JsContainerFromValue {
    pub fn get(&self, key: K) -> Option<V> {
        let key = key.into_value();
        let val = js_sys::Reflect::get(&self.map, &key).ok()?;
        if val.is_null() || val.is_undefined() {
            return None;
        }
        let val = V::from_value(val);

        Some(val)
    }    
}

impl<K, V> JsHashMap<K, V> where K: JsContainerIntoValue, V: JsContainerIntoValue {
    pub fn set(&self, key: K, value: V) {
        let key = key.into_value();
        let value = value.into_value();
        js_sys::Reflect::set(&self.map, &key, &value).expect("expected to set js value");
    }
}

impl<K, V> From<Object> for JsHashMap<K, V> {
    fn from(map: Object) -> Self {
        Self {
            map,
            _phantom: Default::default()
        }
    }
}

impl<K, V> From<JsValue> for JsHashMap<K, V> {
    fn from(val: JsValue) -> Self {
        Self {
            map: val.into(),
            _phantom: Default::default()
        }
    }
}

#[derive(Debug, Clone)]
pub struct OwnedArrayIter<T> {
    range: std::ops::Range<u32>,
    array: Array,
    _phantom: PhantomData<T>
}

impl<T> OwnedArrayIter<T> {
    pub fn new(array: Array) -> Self {
        OwnedArrayIter {
            range: 0..array.length(),
            array: array,
            _phantom: Default::default()
        }
    }
}

impl<T> std::iter::Iterator for OwnedArrayIter<T> where T: JsContainerFromValue {
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

impl<T> std::iter::DoubleEndedIterator for OwnedArrayIter<T> where T: JsContainerFromValue {
    fn next_back(&mut self) -> Option<Self::Item> {
        let index = self.range.next_back()?;
        let val = self.array.get(index);
        let val = T::from_value(val);
        Some(val)
    }
}

impl<T> std::iter::FusedIterator for OwnedArrayIter<T> where T: JsContainerFromValue {}

impl<T> std::iter::ExactSizeIterator for OwnedArrayIter<T> where T: JsContainerFromValue {}

//
// Utility conversions for containers.
//
impl JsContainerIntoValue for JsString {
    fn into_value(self) -> JsValue {
        self.unchecked_into()
    }
}

impl JsContainerFromValue for JsString {
    fn from_value(val: JsValue) -> JsString {
        val.unchecked_into()
    }
}

impl JsContainerIntoValue for String {
    fn into_value(self) -> JsValue {
        self.into()
    }
}

impl JsContainerFromValue for String {
    fn from_value(val: JsValue) -> String {
        let val: JsString = val.unchecked_into();
        
        val.into()
    }
}

impl JsContainerIntoValue for u8 {
    fn into_value(self) -> JsValue {
        JsValue::from_f64(self as f64)
    }
}

impl JsContainerFromValue for u8 {
    fn from_value(val: JsValue) -> u8 {
        if let Some(val) = val.as_string() {
            u8::from_str_radix(&val, 10).expect("expected parseable u8 string")
        } else {
            val.as_f64().expect("expected number value") as u8
        }
    }
}