use std::{
    error,
    fmt::{self, Write},
    ops,
    str::FromStr,
};

use arrayvec::ArrayString;

use super::{HALF_WORLD_SIZE, VALID_ROOM_NAME_COORDINATES};

/// A structure representing a room name.
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LocalRoomName {
    /// A bit-packed integer, containing, from highest-order to lowest:
    ///
    /// - 1 byte: (room_x) + 128
    /// - 1 byte: (room_y) + 128
    ///
    /// For `Wxx` rooms, `room_x = -xx - 1`. For `Exx` rooms, `room_x = xx`.
    ///
    /// For `Nyy` rooms, `room_y = -yy - 1`. For `Syy` rooms, `room_y = yy`.
    ///
    /// This is the same representation of the upper 16 bits of [`Position`]'s
    /// packed representation.
    packed: u16,
}

impl fmt::Display for LocalRoomName {
    /// Formats this room name into the format the game expects.
    ///
    /// Resulting string will be `(E|W)[0-9]+(N|S)[0-9]+`, and will result
    /// in the same LocalRoomName if passed into [`LocalRoomName::new`].
    ///
    /// [`LocalRoomName::new`]: struct.LocalRoomName.html#method.new
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let x_coord = self.x_coord();

        if x_coord >= 0 {
            write!(f, "E{}", x_coord)?;
        } else {
            write!(f, "W{}", -x_coord - 1)?;
        }

        let y_coord = self.y_coord();

        if y_coord >= 0 {
            write!(f, "S{}", y_coord)?;
        } else {
            write!(f, "N{}", -y_coord - 1)?;
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

    #[inline]
    pub(crate) fn from_packed(packed: u16) -> Self {
        LocalRoomName { packed }
    }

    /// Creates a new room name from room coords with direction implicit in
    /// sign.
    ///
    /// For `Wxx` rooms, `room_x = -xx - 1`. For `Exx` rooms, `room_x = xx`.
    ///
    /// For `Nyy` rooms, `room_y = -yy - 1`. For `Syy` rooms, `room_y = yy`.
    ///
    /// # Errors
    ///
    /// Returns an error if the coordinates are outside of the valid room name
    /// bounds.
    pub(super) fn from_coords(x_coord: i32, y_coord: i32) -> Result<Self, LocalRoomNameParseError> {
        if !VALID_ROOM_NAME_COORDINATES.contains(&x_coord)
            || !VALID_ROOM_NAME_COORDINATES.contains(&y_coord)
        {
            return Err(LocalRoomNameParseError::PositionOutOfBounds { x_coord, y_coord });
        }

        let room_x = (x_coord + HALF_WORLD_SIZE) as u16;
        let room_y = (y_coord + HALF_WORLD_SIZE) as u16;

        Ok(Self::from_packed((room_x << 8) | room_y))
    }

    /// Gets the x coordinate.
    ///
    /// For `Wxx` rooms, returns `-xx - 1`. For `Exx` rooms, returns `xx`.
    #[inline]
    pub(super) fn x_coord(&self) -> i32 {
        ((self.packed >> 8) & 0xFF) as i32 - HALF_WORLD_SIZE
    }

    /// Gets the y coordinate.
    ///
    /// For `Nyy` rooms, returns `-yy - 1`. For `Syy` rooms, returns `yy`.
    #[inline]
    pub(super) fn y_coord(&self) -> i32 {
        (self.packed & 0xFF) as i32 - HALF_WORLD_SIZE
    }

    #[inline]
    pub(super) fn packed_repr(&self) -> u16 {
        self.packed
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
    ///
    /// # Panics
    ///
    /// Will panic if the addition overflows the boundaries of LocalRoomName.
    #[inline]
    fn add(self, (x, y): (i32, i32)) -> Self {
        LocalRoomName::from_coords(self.x_coord() + x, self.y_coord() + y)
            .expect("expected addition to keep LocalRoomName in-bounds")
    }
}

impl ops::Sub<(i32, i32)> for LocalRoomName {
    type Output = Self;

    /// Offsets this room name in the opposite direction from the coordinates.
    ///
    /// See the implementation for `Add<(i32, i32)>`.
    ///
    /// # Panics
    ///
    /// Will panic if the subtraction overflows the boundaries of LocalRoomName.
    #[inline]
    fn sub(self, (x, y): (i32, i32)) -> Self {
        LocalRoomName::from_coords(self.x_coord() - x, self.y_coord() - y)
            .expect("expected addition to keep LocalRoomName in-bounds")
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
        (
            self.x_coord() - other.x_coord(),
            self.y_coord() - other.y_coord(),
        )
    }
}

impl FromStr for LocalRoomName {
    type Err = LocalRoomNameParseError;

    fn from_str(s: &str) -> Result<Self, LocalRoomNameParseError> {
        parse_to_coords(s)
            .map_err(|()| LocalRoomNameParseError::new(s))
            .and_then(|(x, y)| LocalRoomName::from_coords(x, y))
    }
}

fn parse_to_coords(s: &str) -> Result<(i32, i32), ()> {
    if s == "sim" {
        return Ok((0, 0));
    }

    let mut chars = s.char_indices();

    let east = match chars.next() {
        Some((_, 'E')) | Some((_, 'e')) => true,
        Some((_, 'W')) | Some((_, 'w')) => false,
        _ => return Err(()),
    };

    let (x_coord, south): (i32, bool) = {
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

    let y_coord: i32 = {
        let (start_index, _) = chars.next().ok_or(())?;

        s[start_index..s.len()].parse().map_err(|_| ())?
    };

    let room_x = if east { x_coord } else { -x_coord - 1 };
    let room_y = if south { y_coord } else { -y_coord - 1 };

    Ok((room_x, room_y))
}

/// An error representing when a string can't be parsed into a
/// [`LocalRoomName`].
///
/// [`LocalRoomName`]: struct.LocalRoomName.html
#[derive(Clone, Debug)]
pub enum LocalRoomNameParseError {
    TooLarge { length: usize },
    InvalidString { string: ArrayString<[u8; 8]> },
    PositionOutOfBounds { x_coord: i32, y_coord: i32 },
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
            LocalRoomNameParseError::PositionOutOfBounds { x_coord, y_coord } => write!(
                f,
                "expected room name with coords within -128..+128, found {}, {}",
                x_coord, y_coord,
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

    use super::LocalRoomName;
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
            formatter.write_str(
                "room name formatted `(E|W)[0-9]+(N|S)[0-9]+` with both numbers within -128..128",
            )
        }

        fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
        where
            E: Error,
        {
            v.parse()
                .map_err(|_| E::invalid_value(Unexpected::Str(v), &self))
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
