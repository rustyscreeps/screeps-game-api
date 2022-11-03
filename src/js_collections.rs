use std::marker::PhantomData;

use js_sys::{Array, JsString, Object};

use wasm_bindgen::{prelude::*, JsCast};

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

pub trait JsContainerIntoValue {
    fn into_value(self) -> JsValue;
}

pub trait JsContainerFromValue {
    fn from_value(val: JsValue) -> Self;
}

pub struct JsHashMap<K, V> {
    map: Object,
    _phantom: PhantomData<(K, V)>,
}

impl<K, V> JsHashMap<K, V>
where
    K: JsContainerFromValue,
{
    pub fn keys(&self) -> impl Iterator<Item = K> {
        let array = Object::keys(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }
}

impl<K, V> JsHashMap<K, V>
where
    V: JsContainerFromValue,
{
    pub fn values(&self) -> impl Iterator<Item = V> {
        let array = Object::values(self.map.unchecked_ref());

        OwnedArrayIter::new(array)
    }
}

impl<K, V> JsHashMap<K, V>
where
    K: JsContainerIntoValue,
    V: JsContainerFromValue,
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
    K: JsContainerIntoValue,
    V: JsContainerIntoValue,
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
    T: JsContainerFromValue,
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
    T: JsContainerFromValue,
{
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
            val.parse::<u8>().expect("expected parseable u8 string")
        } else {
            val.as_f64().expect("expected number value") as u8
        }
    }
}
