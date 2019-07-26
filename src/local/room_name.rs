use std::{
    borrow::Cow,
    error,
    fmt::{self, Write},
    ops,
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
    pub x_coord: i32,
    /// Inner y coordinate representation.
    ///
    /// 0 represents S0, positive numbers represent S(y)
    ///
    /// -1 represents N0, negative numbers represent N((-y) - 1)
    pub y_coord: i32,
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
    /// Creates a new room name from the given input.
    ///
    /// This will parse the input, and return an error if it is in an invalid
    /// format.
    #[inline]
    pub fn new<T>(x: &T) -> Result<Self, LocalRoomNameParseError<'_>>
    where
        T: AsRef<str> + ?Sized,
    {
        let s = x.as_ref();
        parse_or_cheap_failure(s).map_err(|()| LocalRoomNameParseError::new(s.to_owned()))
    }

    /// Creates a new room name from the given position parameters.
    #[inline]
    pub fn from_coords(east: bool, south: bool, x_pos: i32, y_pos: i32) -> Self {
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

    /// Adds an (x, y) coordinate pair to this room name.
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

    /// Subtracts an (x, y) coordinate pair to this room name.
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

    /// Subtracts an (x, y) coordinate pair to this room name.
    #[inline]
    fn sub(self, other: LocalRoomName) -> (i32, i32) {
        (self.x_coord - other.x_coord, self.y_coord - other.y_coord)
    }
}

/// Something that can be turned into a room name.
pub trait IntoLocalRoomName {
    /// Turns this data into a room name, erroring if the format is not as
    /// expected.
    fn into_room_name(&self) -> Result<LocalRoomName, LocalRoomNameParseError<'_>>;
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
pub struct LocalRoomNameParseError<'a>(Cow<'a, str>);

impl<'a> LocalRoomNameParseError<'a> {
    /// Private method to construct a `LocalRoomNameParseError`.
    fn new<T: Into<Cow<'a, str>>>(failed_room_name: T) -> Self {
        LocalRoomNameParseError(failed_room_name.into())
    }

    /// Turns this error into a 'static error, cloning any inner data that
    /// represents what failed.
    pub fn into_owned(self) -> LocalRoomNameParseError<'static> {
        let LocalRoomNameParseError(cow) = self;
        LocalRoomNameParseError(cow.into_owned().into())
    }

    /// Retrieves the room name that failed to parse into a [`LocalRoomName`].
    ///
    /// [`LocalRoomName`]: struct.LocalRoomName.html
    pub fn get_failed_str(&self) -> &str {
        self.0.as_ref()
    }
}

impl<'a> error::Error for LocalRoomNameParseError<'a> {
    fn description(&self) -> &str {
        "string failed to parse into room name"
    }
}

impl<'a> fmt::Display for LocalRoomNameParseError<'a> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "expected room name formatted `(E|W)[0-9]+(N|S)[0-9]+`, found `{}`",
            self.0.as_ref()
        )
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

#[cfg(test)]
mod local_room_name_tests {
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
