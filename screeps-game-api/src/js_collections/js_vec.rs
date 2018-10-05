//! [`JsVec`]
use std::marker::PhantomData;

use stdweb::{Array, InstanceOf, Reference, ReferenceType, Value};

use {
    traits::{FromExpectedType, TryFrom, TryInto},
    ConversionError,
};

//   - InstanceOf
//   - AsRef<Reference>
//   - ReferenceType
//   - Into<Reference>
//   - TryInto<Reference>
//   - TryFrom<Reference>
//   - TryFrom<&Reference>
//   - TryFrom<Value>
//   - TryFrom<&Value>

pub struct JsVec<T> {
    inner: Array,
    phantom: PhantomData<Vec<T>>,
}

impl<T> JsVec<T> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }
}
impl<T> JsVec<T>
where
    T: TryFrom<Value, Error = ConversionError>,
{
    pub fn get(&self, idx: usize) -> Result<Option<T>, ConversionError> {
        if idx >= self.len() || (idx > (u32::max_value() as usize)) {
            Ok(None)
        } else {
            (js!{
                return @{self.inner.as_ref()}[@{idx as u32}];
            })
            .try_into()
            .map(Some)
        }
    }

    pub fn local(self) -> Result<Vec<T>, ConversionError>
    where
        T: TryFrom<Value, Error = ConversionError>,
    {
        self.try_into()
    }
}

pub struct IntoIter<T> {
    index: u32,
    inner: JsVec<T>,
}

pub struct Iter<'a, T> {
    index: u32,
    inner: &'a JsVec<T>,
}

impl_js_vec_iterators!(Iter<'a>, IntoIter);

impl<T> IntoIterator for JsVec<T>
where
    T: TryFrom<Value, Error = ConversionError>,
{
    type Item = Result<T, ConversionError>;
    type IntoIter = IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        IntoIter {
            index: 0,
            inner: self,
        }
    }
}

impl<'a, T> IntoIterator for &'a JsVec<T>
where
    T: TryFrom<Value, Error = ConversionError>,
{
    type Item = Result<T, ConversionError>;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        Iter {
            index: 0,
            inner: self,
        }
    }
}

impl<T> AsRef<Array> for JsVec<T> {
    fn as_ref(&self) -> &Array {
        &self.inner
    }
}

impl<T> From<JsVec<T>> for Array {
    fn from(jsv: JsVec<T>) -> Array {
        jsv.inner
    }
}

impl<T> TryFrom<JsVec<T>> for Array {
    type Error = ConversionError;

    fn try_from(jsv: JsVec<T>) -> Result<Array, Self::Error> {
        Ok(jsv.inner)
    }
}

impl<T> From<JsVec<T>> for Reference {
    fn from(jsv: JsVec<T>) -> Reference {
        jsv.inner.into()
    }
}

impl<T> TryFrom<JsVec<T>> for Reference {
    type Error = ConversionError;

    fn try_from(jsv: JsVec<T>) -> Result<Reference, Self::Error> {
        Ok(jsv.inner.into())
    }
}

impl<T> InstanceOf for JsVec<T>
where
    T: InstanceOf,
{
    fn instance_of(reference: &Reference) -> bool {
        (js!{
            let arr = @{reference};
            if (!(arr instanceof Array)) {
                return false;
            }
            for (let item of arr) {
                if (!(@{|r: Reference| T::instance_of(&r)}(item))) {
                    return false;
                }
            }
            return true;
        })
        .try_into()
        .expect("expected JsVec instance_of js code returning a bool to return a bool")
    }
}

impl<T> TryFrom<Array> for JsVec<T>
where
    T: InstanceOf,
{
    type Error = ConversionError;

    fn try_from(arr: Array) -> Result<JsVec<T>, Self::Error> {
        if arr.len() == 0 {
            return Ok(JsVec {
                inner: arr,
                phantom: PhantomData,
            });
        }

        // Type check array elements
        if !Self::instance_of(arr.as_ref()) {
            return Err(ConversionError::Custom(
                "reference is of a different type".into(),
            ));
        }
        Ok(JsVec {
            inner: arr,
            phantom: PhantomData,
        })
    }
}

impl<T> TryFrom<Reference> for JsVec<T>
where
    T: InstanceOf,
{
    type Error = ConversionError;

    fn try_from(r: Reference) -> Result<JsVec<T>, Self::Error> {
        let arr: Array = r.try_into()?;
        arr.try_into()
    }
}

impl<T> TryFrom<Value> for JsVec<T>
where
    T: InstanceOf,
{
    type Error = ConversionError;

    fn try_from(r: Value) -> Result<JsVec<T>, Self::Error> {
        let arr: Array = r.try_into()?;
        arr.try_into()
    }
}

impl<T> TryInto<Vec<T>> for JsVec<T>
where
    T: TryFrom<Value, Error = ConversionError>,
{
    type Error = ConversionError;

    fn try_into(self) -> Result<Vec<T>, Self::Error> {
        self.inner.try_into()
    }
}

impl<T> AsRef<Reference> for JsVec<T> {
    fn as_ref(&self) -> &Reference {
        self.inner.as_ref()
    }
}

impl<T> FromExpectedType<Array> for JsVec<T>
where
    T: InstanceOf,
{
    fn from_expected_type(arr: Array) -> Result<Self, ConversionError> {
        #[cfg(feature = "check-all-casts")]
        {
            arr.try_into()
        }
        #[cfg(not(feature = "check-all-casts"))]
        {
            Ok(JsVec {
                inner: arr,
                phantom: PhantomData,
            })
        }
    }
}

impl<T> ReferenceType for JsVec<T>
where
    T: InstanceOf,
{
    unsafe fn from_reference_unchecked(reference: Reference) -> Self {
        JsVec {
            inner: Array::from_reference_unchecked(reference),
            phantom: PhantomData,
        }
    }
}

impl<T> FromExpectedType<Reference> for JsVec<T>
where
    T: InstanceOf,
{
    fn from_expected_type(r: Reference) -> Result<Self, ConversionError> {
        #[cfg(feature = "check-all-casts")]
        {
            r.try_into()
        }
        #[cfg(not(feature = "check-all-casts"))]
        {
            Ok(unsafe { Self::from_reference_unchecked(r) })
        }
    }
}
