//! Useful traits for interacting with JavaScript above what [`stdweb`]
//! provides.

pub use stdweb::unstable::{TryFrom, TryInto};
use stdweb::{Reference, Value};

use crate::ConversionError;

/// See [`IntoExpectedType`]
pub trait FromExpectedType<T>: Sized {
    fn from_expected_type(v: T) -> Result<Self, ConversionError>;
}

/// Trait for casting api results which we expect to be the right thing as long
/// as all JS code is behaving as expected.
///
/// This trait allows us to switch between checked and unchecked casts at
/// compile time with the `"check-all-casts"` feature flag.
pub trait IntoExpectedType<T> {
    /// Casts this value as the target type, making the assumption that the
    /// types are correct.
    ///
    /// # Error conditions
    ///
    /// If the types don't match up, and `"check-all-casts"` is enabled, this
    /// will return an error.
    ///
    /// If this is a non-`Reference` `Value`, this will return an error.
    fn into_expected_type(self) -> Result<T, ConversionError>;
}

impl<T> FromExpectedType<Value> for T
where
    T: FromExpectedType<Reference>,
{
    fn from_expected_type(v: Value) -> Result<Self, ConversionError> {
        Reference::try_from(v).and_then(|reference| reference.into_expected_type())
    }
}

impl<T> FromExpectedType<Value> for Option<T>
where
    T: FromExpectedType<Reference>,
{
    fn from_expected_type(v: Value) -> Result<Self, ConversionError> {
        <Option<Reference>>::try_from(v).and_then(|opt_reference| {
            opt_reference
                .map(|reference| reference.into_expected_type().map(Some))
                .unwrap_or(Ok(None))
        })
    }
}

// TODO: this is inefficient
impl<T> FromExpectedType<Value> for Vec<T>
where
    T: FromExpectedType<Reference>,
{
    fn from_expected_type(v: Value) -> Result<Self, ConversionError> {
        <Vec<Reference>>::try_from(v).and_then(|ref_vec| {
            ref_vec
                .into_iter()
                .map(|reference| reference.into_expected_type())
                .collect()
        })
    }
}

impl<T, U> IntoExpectedType<U> for T
where
    U: FromExpectedType<T>,
{
    fn into_expected_type(self) -> Result<U, ConversionError> {
        U::from_expected_type(self)
    }
}
