use std::marker::PhantomData;

use stdweb::{Array, Object, Reference};

use traits::{ConversionError, FromExpectedType, TryInto};

//   - InstanceOf
//   - AsRef<Reference>
//   - ReferenceType
//   - Into<Reference>
//   - TryInto<Reference>
//   - TryFrom<Reference>
//   - TryFrom<&Reference>
//   - TryFrom<Value>
//   - TryFrom<&Value>

struct JsVec<T> {
    inner: Array,
    phantom: PhantomData<Vec<T>>,
}

impl<T> JsVec<T> {
    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn local(&self) -> Result<Vec<T>, Self::Error> {
        self.try_into()
    }
}

impl<T> AsRef<Array> for JsVec<T> {
    pub fn as_ref(&self) -> &Array {
        &self.inner
    }
}

impl<T> From<JsVec<T>> for Array {
    pub fn from(jsv: JsVec<T>) -> Array {
        jsv.inner
    }
}

impl<T> TryFrom<JsVec<T>> for Array {
    type Error = ConversionError;

    pub fn try_from(jsv: JsVec<T>) ->  Result<Array, Self::Error> {
        Ok(jsv.inner)
    }
}

impl<T> From<JsVec<T>> for Reference {
    pub fn from(jsv: JsVec<T>) -> Reference {
        jsv.inner.into()
    }
}

impl<T> TryFrom<JsVec<T>> for Reference {
    type Error = ConversionError;

    pub fn try_from(jsv: JsVec<T>) -> Result<Reference, Self::Error> {
        Ok(jsv.inner.into())
    }
}

impl<T> TryFrom<Array> for JsVec<T>
where
    T: TryFrom<Value>,
{
    type Error = ConversionError;

    pub fn try_from(arr: Array) -> Result<JsVec<T>, Self::Error> {
        if arr.len() == 0 {
            return Ok(JsVec{inner: arr, phantom: PhantomData})
        }

        // Type check the first array element
        let v: Result<T, Self::Error> = js!{return @{arr}[0];}.try_into();
        v.map(|| Ok(JsVec{inner: arr, phantom: PhantomData}))
    }
}

impl<T> TryFrom<Reference> for JsVec<T>
where
    T: TryFrom<Value>,
{
    type Error = ConversionError;

    pub fn try_from(r: Reference) -> Result<JsVec<T>, Self::Error> {
        let arr = r.try_into()?;
        arr.try_into()
    }
}

impl<T> TryFrom<JsVec<T>> for Vec<T>
where
    T: TryFrom<Value>,
{
    type Error = ConversionError;

    pub fn try_from(jsv: JsVec<T>) -> Result<Vec<T>, Self::Error> {
        jsv.inner.try_into()
    }
}

impl<T> FromExpectedType<Array> for JsVec<T>
where
    T: TryFrom<Value>,
{
    fn from_expected_type(arr: Array) -> Result<Self, ConversionError> {
        #[cfg(feature = "check-all-casts")]
        {
            arr.try_into()
        }
        #[cfg(not(feature = "check-all-casts"))]
        {
            Ok(JsVec{inner: arr, phantom: PhantomData})
        }
    }
}

impl<T> FromExpectedType<Reference> for JsVec<T>
where
    T: TryFrom<Value>,
{
    fn from_expected_type(r: Reference) -> Result<Self, ConversionError> {
        #[cfg(feature = "check-all-casts")]
        {
            r.try_into()
        }
        #[cfg(not(feature = "check-all-casts"))]
        {
            let arr = Array::from_reference_unchecked(r);
            Ok(JsVec{inner: arr, phantom: PhantomData})
        }
    }
}














struct JsMap<K, T> {
    inner: Object,
    phantom: PhantomData<HashMap<K, V>>,
}

// impl Index<Nucleotide> for NucleotideCount {
//     type Output = usize;

//     fn index(&self, nucleotide: Nucleotide) -> &usize {
//         match nucleotide {
//             Nucleotide::A => &self.a,
//             Nucleotide::C => &self.c,
//             Nucleotide::G => &self.g,
//             Nucleotide::T => &self.t,
//         }
//     }
// }
