use std::{fmt, fmt::Write, str::FromStr};

use arrayvec::ArrayString;
use js_sys::JsString;
use serde::{
    de::{Error, Visitor},
    Deserialize, Serialize,
};

use super::errors::RawObjectIdParseError;

const MAX_PACKED_VAL: u128 = (1 << (32 * 3)) - 1;

/// Represents an Object ID using a packed 12-byte representation
///
/// Each object id in screeps is represented by a Mongo GUID, which,
/// while not guaranteed, is unlikely to change. This takes advantage of that by
/// storing a packed representation.
///
/// # Ordering
///
/// To facilitate use as a key in a [`BTreeMap`] or other similar data
/// structures, `ObjectId` implements [`PartialOrd`] and [`Ord`].
///
/// `RawObjectId`'s are ordered by the corresponding order of their underlying
/// byte values. See [`ObjectId`] documentation for more information.
///
/// [`BTreeMap`]: std::collections::BTreeMap
/// [`Ord`]: std::cmp::Ord
/// [`PartialOrd`]: std::cmp::PartialOrd
/// [`ObjectId`]: super::ObjectId
#[derive(Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct RawObjectId {
    packed: u128,
}

impl fmt::Debug for RawObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawObjectId")
            .field("packed", &self.packed)
            .field("real", &self.to_string())
            .finish()
    }
}

impl fmt::Display for RawObjectId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:0width$x}",
            self.packed >> 32,
            width = (self.packed as u32) as usize
        )
    }
}

impl Serialize for RawObjectId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        if serializer.is_human_readable() {
            serializer.collect_str(&self.to_array_string())
        } else {
            serializer.serialize_bytes(&self.packed.to_be_bytes())
        }
    }
}

struct RawObjectIdVisitor;

impl<'de> Visitor<'de> for RawObjectIdVisitor {
    type Value = RawObjectId;

    fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        formatter.write_str("a string or bytes representing an object id")
    }

    fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
    where
        E: Error,
    {
        RawObjectId::from_str(v).map_err(|e| E::custom(format!("Could not parse object id: {e}")))
    }

    fn visit_bytes<E>(self, v: &[u8]) -> Result<Self::Value, E>
    where
        E: Error,
    {
        v.try_into()
            .map(u128::from_be_bytes)
            .map(RawObjectId::from_packed)
            .map_err(|e| E::custom(format!("Could not parse object id: {e}")))
    }
}

impl<'de> Deserialize<'de> for RawObjectId {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        if deserializer.is_human_readable() {
            deserializer.deserialize_str(RawObjectIdVisitor)
        } else {
            deserializer.deserialize_bytes(RawObjectIdVisitor)
        }
    }
}

impl FromStr for RawObjectId {
    type Err = RawObjectIdParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        // get the actual integer value of the id, which we'll store in the most
        // significant 96 bits of the u128
        let u128_id = u128::from_str_radix(s, 16)?;
        // and the length, which we know can't be greater than 24 without going over
        // MAX_PACKED_VAL
        let pad_length = s.len() as u128;

        if u128_id > MAX_PACKED_VAL || pad_length > 24 {
            return Err(RawObjectIdParseError::value_too_large(u128_id));
        }

        Ok(Self::from(u128_id << 32 | pad_length))
    }
}

impl TryFrom<JsString> for RawObjectId {
    type Error = RawObjectIdParseError;

    fn try_from(js_id: JsString) -> Result<Self, Self::Error> {
        let id: String = js_id.into();
        RawObjectId::from_str(&id)
    }
}

impl RawObjectId {
    /// Creates an object ID from its packed representation.
    ///
    /// The input to this function is the bytes representing the up-to-24 hex
    /// digits in the object id.
    pub const fn from_packed(packed: u128) -> Self {
        RawObjectId { packed }
    }

    /// Formats this object ID as a string on the stack.
    ///
    /// This is equivalent to [`ToString::to_string`], but involves no
    /// allocation.
    pub fn to_array_string(&self) -> ArrayString<24> {
        let mut res = ArrayString::new();
        write!(res, "{self}").expect("expected formatting into a fixed-sized buffer to succeed");
        res
    }
}

impl From<RawObjectId> for ArrayString<24> {
    fn from(id: RawObjectId) -> Self {
        id.to_array_string()
    }
}

impl From<RawObjectId> for String {
    fn from(id: RawObjectId) -> Self {
        id.to_string()
    }
}

impl From<RawObjectId> for u128 {
    fn from(id: RawObjectId) -> Self {
        id.packed
    }
}

impl From<u128> for RawObjectId {
    fn from(packed: u128) -> Self {
        Self::from_packed(packed)
    }
}

#[cfg(test)]
mod test {
    use super::RawObjectId;

    const TEST_IDS: &[&str] = &[
        "0",
        "1",
        // max allowed ID
        "ffffffffffffffffffffffff",
        // 24 char, leading 0 (#278)
        "06aebab343040c9baaa22322",
        "000000000000000000000001",
        "100000000000000000000000",
        // potential bad u128 from float?
        "5bbcab1d9099fc012e632dbc",
        "5bbcab1d9099fc012e632dbd",
        "10000000000000000",
        "1000000000000000",
        "bc03381d32f6790",
        // 15 char, leading 0 (#244)
        "0df4aea318bd552",
        "000000000000f00",
        "100000000",
        "10000000",
    ];

    #[test]
    fn rust_display_rust_fromstr_roundtrip() {
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            assert_eq!(&*parsed.to_string(), *id);
        }
    }

    #[test]
    fn rust_to_array_string_rust_fromstr_roundtrip() {
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            assert_eq!(&*parsed.to_array_string(), *id);
        }
    }

    #[test]
    fn rust_to_u128_from_u128_roundtrip() {
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            let int = u128::from(parsed);
            let reparsed: RawObjectId = int.into();
            assert_eq!(parsed, reparsed);
            assert_eq!(reparsed.to_string(), *id);
        }
    }

    #[test]
    fn rust_to_serde_json_from_serde_json_roundtrip() {
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            let serialized = serde_json::to_string(&parsed).unwrap();
            let reparsed: RawObjectId = serde_json::from_str(&serialized).unwrap();
            assert_eq!(parsed, reparsed);
            assert_eq!(reparsed.to_string(), *id);
        }
    }

    #[test]
    fn rust_vec_to_serde_json_from_serde_json_roundtrip() {
        let mut ids_parsed = vec![];
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            ids_parsed.push(parsed);
        }
        let serialized = serde_json::to_string(&ids_parsed).unwrap();
        let ids_reparsed: Vec<RawObjectId> = serde_json::from_str(&serialized).unwrap();
        assert_eq!(ids_parsed, ids_reparsed);
    }

    #[test]
    fn rust_to_serde_bincode_from_serde_bincode_roundtrip() {
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            let serialized = bincode::serialize(&parsed).unwrap();
            let reparsed: RawObjectId = bincode::deserialize(&serialized).unwrap();
            assert_eq!(parsed, reparsed);
            assert_eq!(reparsed.to_string(), *id);
        }
    }

    #[test]
    fn rust_vec_to_serde_bincode_from_serde_bincode_roundtrip() {
        let mut ids_parsed = vec![];
        for id in TEST_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            ids_parsed.push(parsed);
        }
        let serialized = bincode::serialize(&ids_parsed).unwrap();
        let ids_reparsed: Vec<RawObjectId> = bincode::deserialize(&serialized).unwrap();
        assert_eq!(ids_parsed, ids_reparsed);
    }

    // sorted first by id value (lowest first), then by the length of the string
    // (shortest first)
    #[rustfmt::skip]
    const SORTED_IDS: &[&str] = &[
        "0",
        "00",
        "000",
        "0000",
        "1",
        "01",
        "001",
        "0001",
        "2",
        "a",
        "f",
        "00f",
        "10",
        "010",
        "0010",
        "100",
        "0100",
        "f00",
        "0f00",
        "1000",
    ];

    #[test]
    fn sort_order() {
        let mut ids_parsed = vec![];
        for id in SORTED_IDS {
            let parsed: RawObjectId = id.parse().unwrap();
            ids_parsed.push(parsed);
        }
        let mut ids_sort: Vec<_> = ids_parsed.clone();
        ids_sort.sort();
        assert_eq!(ids_sort, ids_parsed);
    }

    const INVALID_IDS: &[&str] = &[
        // empty string
        "",
        // negative number
        "-1",
        "-0",
        // longer than 24 characters
        "1000000000000000000000000",
        // valid number but padded beyond what we can store in 96 bits
        "000000000000000000000000f",
        // u128::MAX
        "340282366920938463463374607431768211455",
        // even longer
        "000000000000000000000000000000000000000999000340282366920938463463374607431768211455",
        // bad characters
        "g",
        "💣",
        "\n",
    ];

    #[test]
    fn invalid_values_do_not_parse() {
        for id in INVALID_IDS {
            let res: Result<RawObjectId, _> = id.parse();
            assert!(res.is_err());
        }
    }
}
