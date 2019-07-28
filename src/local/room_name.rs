use std::{
    error,
    fmt::{self, Write},
    ops,
    str::FromStr,
};

use arrayvec::ArrayString;

/// A structure representing a room name.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LocalRoomName {
    /// Inner x coordinate representation.
    ///
    /// 0 represents E0, positive numbers represent E(x)
    ///
    /// -1 represents W0, negative numbers represent W((-x) - 1)
    pub(super) x_coord: i32,
    /// Inner y coordinate representation.
    ///
    /// 0 represents S0, positive numbers represent S(y)
    ///
    /// -1 represents N0, negative numbers represent N((-y) - 1)
    pub(super) y_coord: i32,
}

impl fmt::Display for LocalRoomName {
    /// Formats this room name into the format the game expects.
    ///
    /// Resulting string will be `(E|W)[0-9]+(N|S)[0-9]+`, and will result
    /// in the same LocalRoomName if passed into [`LocalRoomName::new`].
    ///
    /// [`LocalRoomName::new`]: struct.LocalRoomName.html#method.new
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.x_coord >= 0 {
            write!(f, "E{}", self.x_coord)?;
        } else {
            write!(f, "W{}", (-self.x_coord) - 1)?;
        }

        if self.y_coord >= 0 {
            write!(f, "S{}", self.y_coord)?;
        } else {
            write!(f, "N{}", (-self.y_coord) - 1)?;
        }

        Ok(())
    }
}

impl LocalRoomName {
    /// Parses a room name from a string.
    ///
    /// This will parse the input string, returning an error if it is in an
    /// invalid room name.
    ///
    /// The expected format can be represented by the regex
    /// `[ewEW][0-9]+[nsNS][0-9]+`.
    #[inline]
    pub fn new<T>(x: &T) -> Result<Self, LocalRoomNameParseError>
    where
        T: AsRef<str> + ?Sized,
    {
        x.as_ref().parse()
    }

    /// Creates a new room name from the given position parameters.
    #[inline]
    pub(crate) fn from_coords(east: bool, south: bool, x_pos: i32, y_pos: i32) -> Self {
        LocalRoomName {
            x_coord: if east { x_pos } else { -x_pos - 1 },
            y_coord: if south { y_pos } else { -y_pos - 1 },
        }
    }

    /// Converts this LocalRoomName into an efficient, stack-based string.
    ///
    /// This is equivalent to [`ToString::to_string`], but involves no
    /// allocation.
    pub fn to_array_string(&self) -> ArrayString<[u8; 8]> {
        let mut res = ArrayString::new();
        write!(res, "{}", self).expect("expected ArrayString write to be unfallible");
        res
    }
}

impl ops::Add<(i32, i32)> for LocalRoomName {
    type Output = Self;

    /// Offsets this room name by a given horizontal and vertical (x, y) pair.
    ///
    /// The first number offsets to the west when negative and to the east when
    /// positive. The first number offsets to the north when negative and to
    /// the south when positive.
    #[inline]
    fn add(self, (x, y): (i32, i32)) -> Self {
        LocalRoomName {
            x_coord: self.x_coord + x,
            y_coord: self.y_coord + y,
        }
    }
}

impl ops::Sub<(i32, i32)> for LocalRoomName {
    type Output = Self;

    /// Offsets this room name in the opposite direction from the coordinates.
    ///
    /// See the implementation for `Add<(i32, i32)>`.
    #[inline]
    fn sub(self, (x, y): (i32, i32)) -> Self {
        LocalRoomName {
            x_coord: self.x_coord - x,
            y_coord: self.y_coord - y,
        }
    }
}

impl ops::Sub<LocalRoomName> for LocalRoomName {
    type Output = (i32, i32);

    /// Subtracts one room name from the other, extracting the difference.
    ///
    /// The first return value represents east/west offset, with 'more east'
    /// being positive and 'more west' being negative.
    ///
    /// The second return value represents north/south offset, with 'more south'
    /// being positive and 'more north' being negative.
    ///
    /// This coordinate system agrees with the implementations `Add<(i32, i32)>
    /// for LocalRoomName` and `Sub<(i32, i32)> for LocalRoomName`.
    #[inline]
    fn sub(self, other: LocalRoomName) -> (i32, i32) {
        (self.x_coord - other.x_coord, self.y_coord - other.y_coord)
    }
}

impl FromStr for LocalRoomName {
    type Err = LocalRoomNameParseError;

    fn from_str(s: &str) -> Result<Self, LocalRoomNameParseError> {
        parse_or_cheap_failure(s).map_err(|()| LocalRoomNameParseError::new(s))
    }
}

fn parse_or_cheap_failure(s: &str) -> Result<LocalRoomName, ()> {
    if s == "sim" {
        return Ok(LocalRoomName {
            x_coord: 0,
            y_coord: 0,
        });
    }

    let mut chars = s.char_indices();

    let east = match chars.next() {
        Some((_, 'E')) | Some((_, 'e')) => true,
        Some((_, 'W')) | Some((_, 'w')) => false,
        _ => return Err(()),
    };

    let (x_coord, south) = {
        // we assume there's at least one number character. If there isn't,
        // we'll catch it when we try to parse this substr.
        let (start_index, _) = chars.next().ok_or(())?;
        let end_index;
        let south;
        loop {
            match chars.next().ok_or(())? {
                (i, 'N') | (i, 'n') => {
                    end_index = i;
                    south = false;
                    break;
                }
                (i, 'S') | (i, 's') => {
                    end_index = i;
                    south = true;
                    break;
                }
                _ => continue,
            }
        }

        let x_coord = s[start_index..end_index].parse().map_err(|_| ())?;

        (x_coord, south)
    };

    let y_coord = {
        let (start_index, _) = chars.next().ok_or(())?;

        s[start_index..s.len()].parse().map_err(|_| ())?
    };

    Ok(LocalRoomName::from_coords(east, south, x_coord, y_coord))
}

/// An error representing when a string can't be parsed into a
/// [`LocalRoomName`].
///
/// [`LocalRoomName`]: struct.LocalRoomName.html
#[derive(Clone, Debug)]
pub enum LocalRoomNameParseError {
    TooLarge { length: usize },
    InvalidString { string: ArrayString<[u8; 8]> },
}

impl LocalRoomNameParseError {
    /// Private method to construct a `LocalRoomNameParseError`.
    fn new(failed_room_name: &str) -> Self {
        match ArrayString::from(failed_room_name) {
            Ok(string) => LocalRoomNameParseError::InvalidString { string },
            Err(_) => LocalRoomNameParseError::TooLarge {
                length: failed_room_name.len(),
            },
        }
    }
}

impl error::Error for LocalRoomNameParseError {}

impl fmt::Display for LocalRoomNameParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LocalRoomNameParseError::TooLarge { length } => write!(
                f,
                "got invalid room name, too large to stick in error. \
                 expected length 8 or less, got length {}",
                length
            ),
            LocalRoomNameParseError::InvalidString { string } => write!(
                f,
                "expected room name formatted `[ewEW][0-9]+[nsNS][0-9]+`, found `{}`",
                string
            ),
        }
    }
}

impl PartialEq<str> for LocalRoomName {
    fn eq(&self, other: &str) -> bool {
        let s = self.to_array_string();
        s.eq_ignore_ascii_case(other)
    }
}
impl PartialEq<LocalRoomName> for str {
    #[inline]
    fn eq(&self, other: &LocalRoomName) -> bool {
        // Explicitly call the impl for `PartialEq<str>` so that we don't end up
        // accidentally calling one of the other implementations and ending up in an
        // infinite loop.
        //
        // This one in particular would probably be OK, but I've written it this way to
        // be consistent with the others, and to ensure that if this code changes in
        // this future it'll stay working.
        <LocalRoomName as PartialEq<str>>::eq(other, self)
    }
}

impl PartialEq<&str> for LocalRoomName {
    #[inline]
    fn eq(&self, other: &&str) -> bool {
        <LocalRoomName as PartialEq<str>>::eq(self, other)
    }
}

impl PartialEq<LocalRoomName> for &str {
    #[inline]
    fn eq(&self, other: &LocalRoomName) -> bool {
        <LocalRoomName as PartialEq<str>>::eq(other, self)
    }
}

impl PartialEq<String> for LocalRoomName {
    #[inline]
    fn eq(&self, other: &String) -> bool {
        <LocalRoomName as PartialEq<str>>::eq(self, &other)
    }
}

impl PartialEq<LocalRoomName> for String {
    #[inline]
    fn eq(&self, other: &LocalRoomName) -> bool {
        <LocalRoomName as PartialEq<str>>::eq(other, self)
    }
}

impl PartialEq<&String> for LocalRoomName {
    #[inline]
    fn eq(&self, other: &&String) -> bool {
        <LocalRoomName as PartialEq<str>>::eq(self, other)
    }
}

impl PartialEq<LocalRoomName> for &String {
    #[inline]
    fn eq(&self, other: &LocalRoomName) -> bool {
        <LocalRoomName as PartialEq<str>>::eq(other, self)
    }
}

mod serde {
    use std::fmt;

    use serde::{
        de::{Error, Unexpected, Visitor},
        Deserialize, Deserializer, Serialize, Serializer,
    };

    use super::{parse_or_cheap_failure, LocalRoomName};
    use crate::macros::*;

    impl Serialize for LocalRoomName {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            serializer.serialize_str(&self.to_array_string())
        }
    }

    struct LocalRoomNameVisitor;

    impl<'de> Visitor<'de> for LocalRoomNameVisitor {
        type Value = LocalRoomName;

        fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
            formatter.write_str("room name formatted `(E|W)[0-9]+(N|S)[0-9]+`")
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            parse_or_cheap_failure(v).map_err(|()| E::invalid_value(Unexpected::Str(v), &self))
        }
    }

    impl<'de> Deserialize<'de> for LocalRoomName {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            deserializer.deserialize_str(LocalRoomNameVisitor)
        }
    }

    js_deserializable!(LocalRoomName);
    js_serializable!(LocalRoomName);
}

#[cfg(test)]
mod test {
    #[test]
    fn test_string_equality() {
        use super::LocalRoomName;
        let room_names = vec!["E21N4", "w6S42", "W17s5", "e2n5"];
        for room_name in room_names {
            assert_eq!(room_name, LocalRoomName::new(room_name).unwrap());
            assert_eq!(LocalRoomName::new(room_name).unwrap(), room_name);
            assert_eq!(
                LocalRoomName::new(room_name).unwrap(),
                &room_name.to_string()
            );
            assert_eq!(
                &room_name.to_string(),
                LocalRoomName::new(room_name).unwrap()
            );
        }
    }
}
