use stdweb::unstable::{TryFrom, TryInto};
use stdweb::{Array, Reference, ReferenceType, Value};

/// since find returns not "Array" but array from outside the container,
/// we need to do an unsafe cast to get stdweb to treat it like an array.
pub unsafe fn game_array_to_vec<T>(
    v: Reference,
) -> Result<Vec<T>, <Reference as TryFrom<Value>>::Error>
where
    T: TryFrom<Value, Error = <Reference as TryFrom<Value>>::Error>,
{
    Array::from_reference_unchecked(v).try_into()
}
