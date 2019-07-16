//! Structures relating to room name parsing.

use std::{borrow::Cow, error, fmt, ops};

use crate::objects::{HasPosition, RoomPosition};

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
    /// 0 represents N0, positive numbers represent N(y)
    ///
    /// -1 represents S0, negative numbers represent S((-y) - 1)
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
            write!(f, "N{}", self.y_coord)?;
        } else {
            write!(f, "S{}", (-self.y_coord) - 1)?;
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
    pub fn from_coords(east: bool, north: bool, x_pos: i32, y_pos: i32) -> Self {
        LocalRoomName {
            x_coord: if east { x_pos } else { -x_pos - 1 },
            y_coord: if north { y_pos } else { -y_pos - 1 },
        }
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

    let (x_coord, north) = {
        // we assume there's at least one number character. If there isn't,
        // we'll catch it when we try to parse this substr.
        let (start_index, _) = chars.next().ok_or(())?;
        let end_index;
        let north;
        loop {
            match chars.next().ok_or(())? {
                (i, 'N') | (i, 'n') => {
                    end_index = i;
                    north = true;
                    break;
                }
                (i, 'S') | (i, 's') => {
                    end_index = i;
                    north = false;
                    break;
                }
                _ => continue,
            }
        }

        let x_coord = s[start_index..end_index].parse().map_err(|_| ())?;

        (x_coord, north)
    };

    let y_coord = {
        let (start_index, _) = chars.next().ok_or(())?;

        s[start_index..s.len()].parse().map_err(|_| ())?
    };

    Ok(LocalRoomName::from_coords(east, north, x_coord, y_coord))
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

impl PartialEq<&str> for LocalRoomName {
    fn eq(&self, other: &&str) -> bool {
        let s = format!("{}", &self);
        s.eq_ignore_ascii_case(other)
    }
}
impl PartialEq<LocalRoomName> for &str {
    fn eq(&self, other: &LocalRoomName) -> bool {
        other.eq(self)
    }
}

impl PartialEq<String> for LocalRoomName {
    fn eq(&self, other: &String) -> bool {
        let s = format!("{}", &self);
        s.eq_ignore_ascii_case(other.as_str())
    }
}
impl PartialEq<LocalRoomName> for String {
    fn eq(&self, other: &LocalRoomName) -> bool {
        other.eq(&self.as_str())
    }
}

impl PartialEq<&String> for LocalRoomName {
    fn eq(&self, other: &&String) -> bool {
        let s = format!("{}", &self);
        s.eq_ignore_ascii_case(other.as_str())
    }
}
impl PartialEq<LocalRoomName> for &String {
    fn eq(&self, other: &LocalRoomName) -> bool {
        other.eq(&self.as_str())
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
            serializer.collect_str(self)
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

/// This is a room position located in Rust memory.
///
/// It's "local" in the sense that while `RoomPosition` always references a room
/// position allocated by and managed by the JavaScript VM, this is a
/// self-contained plain-data struct in Rust memory.
///
/// # Using LocalRoomPosition
///
/// A `LocalRoomPosition` can be retrieved at any point by using
/// [`RoomPosition::local`]. It can then be copied around freely, and have its
/// values modified.
///
/// `&LocalRoomPosition` can be passed into any game method taking an object,
/// and will be automatically uploaded to JavaScript as a `RoomPosition`.
///
/// If you need to manually create a `RoomPosition` from a `LocalRoomPosition`,
/// use [`LocalRoomPosition::remote`].
///
/// # Serialization
///
/// `LocalRoomPosition` implements both `serde::Serialize` and
/// `serde::Deserialize`.
///
/// When serializing, it will use the obvious format of `{roomName: String, x:
/// u32, y: u32}` in "human readable" formats like JSON, and a less obvious
/// format `{room_x: u32, room_y: u32, x: u32, y: u32}` in "non-human readable"
/// formats like [`bincode`].
///
/// You can also pass `LocalRoomPosition` into JavaScript using the `js!{}`
/// macro provided by `stdweb`, or helper methods using the same code like
/// [`MemoryReference::set`][crate::memory::MemoryReference::set].  It will be
/// serialized the same as in JSON, as an object with `roomName`, `x` and `y`
/// properties.
///
/// *Note:* serializing using `js!{}` or `MemoryReference::set` will _not_
/// create a `RoomPosition`, only something with the same properties. Use
/// `.remote()` if you need a `RoomPosition`.
///
/// [`bincode`]: https://github.com/servo/bincode
#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug)]
pub struct LocalRoomPosition {
    pub room_name: LocalRoomName,
    pub x: u32,
    pub y: u32,
}

impl fmt::Display for LocalRoomPosition {
    /// Formats this into a nice looking string mimicking `RoomPosition`'s
    /// `toString`.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[room {} pos {},{}]", self.room_name, self.x, self.y)
    }
}

impl LocalRoomPosition {
    pub fn remote(&self) -> RoomPosition {
        RoomPosition::new(self.x, self.y, &self.room_name.to_string())
    }
}

impl HasPosition for LocalRoomPosition {
    fn pos(&self) -> RoomPosition {
        self.remote()
    }
}

mod stdweb {
    use stdweb::Value;

    use super::{LocalRoomName, LocalRoomPosition};
    use crate::{
        macros::*,
        traits::{TryFrom, TryInto},
    };

    impl TryFrom<Value> for LocalRoomPosition {
        type Error = <Value as TryInto<String>>::Error;

        fn try_from(v: Value) -> Result<LocalRoomPosition, Self::Error> {
            let x: u32 = (js! {return @{&v}.x}).try_into()?;
            let y: u32 = (js! {return @{&v}.y}).try_into()?;
            let room_name: LocalRoomName = (js! {return @{&v}.roomName}).try_into()?;

            Ok(LocalRoomPosition { x, y, room_name })
        }
    }

    // We don't use `js_deserializable!` since it would generate pretty much exactly
    // the code above, but with slightly extra cost since our `serde::Deserialize`
    // implementation has extra code to be backwards compatible with a different
    // format.
    //
    // On the other hand, we do want `js_serializable!()` since it does more than
    // just implement `TryFrom<LocalRoomPosition> for Value` - it also gives us
    // `JsSerializable` and other impls.

    js_serializable!(LocalRoomPosition);
}

mod room_pos_serde {
    use serde::{de::Error, Deserialize, Deserializer, Serialize, Serializer};

    use super::{LocalRoomName, LocalRoomPosition};

    #[derive(Serialize, Deserialize)]
    #[serde(rename = "SerializedLocalRoomPosition")]
    struct EfficientFormat {
        room_x: i32,
        room_y: i32,
        x: u32,
        y: u32,
    }

    #[derive(Serialize, Deserialize)]
    #[serde(rename_all = "camelCase")]
    struct ReadableFormat {
        room_name: LocalRoomName,
        x: u32,
        y: u32,
    }

    #[derive(Deserialize)]
    // efficient way of storing "either format works"
    //
    // This should be faster than using an untagged enum since this way serde knows
    // exactly what fields are possible in all variants at compile time.
    struct EitherFormat {
        // like EfficientFormat
        #[serde(default)]
        room_x: Option<i32>,
        #[serde(default)]
        room_y: Option<i32>,
        // like ReadableFormat
        #[serde(default)]
        #[serde(rename = "camelCase")]
        room_name: Option<LocalRoomName>,
        // in both
        x: u32,
        y: u32,
    }

    impl From<EfficientFormat> for LocalRoomPosition {
        fn from(
            EfficientFormat {
                room_x,
                room_y,
                x,
                y,
            }: EfficientFormat,
        ) -> Self {
            LocalRoomPosition {
                room_name: LocalRoomName {
                    x_coord: room_x,
                    y_coord: room_y,
                },
                x,
                y,
            }
        }
    }

    impl From<LocalRoomPosition> for EfficientFormat {
        fn from(LocalRoomPosition { room_name, x, y }: LocalRoomPosition) -> Self {
            EfficientFormat {
                room_x: room_name.x_coord,
                room_y: room_name.y_coord,
                x,
                y,
            }
        }
    }

    impl From<ReadableFormat> for LocalRoomPosition {
        fn from(ReadableFormat { room_name, x, y }: ReadableFormat) -> Self {
            LocalRoomPosition { room_name, x, y }
        }
    }

    impl From<LocalRoomPosition> for ReadableFormat {
        fn from(LocalRoomPosition { room_name, x, y }: LocalRoomPosition) -> Self {
            ReadableFormat { room_name, x, y }
        }
    }

    impl Serialize for LocalRoomPosition {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: Serializer,
        {
            if serializer.is_human_readable() {
                ReadableFormat::from(*self).serialize(serializer)
            } else {
                EfficientFormat::from(*self).serialize(serializer)
            }
        }
    }

    impl<'de> Deserialize<'de> for LocalRoomPosition {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: Deserializer<'de>,
        {
            if deserializer.is_human_readable() {
                // an older version of the library always serialized as 'EfficientFormat' - this
                // keeps compatibility with that.
                let either = EitherFormat::deserialize(deserializer)?;
                match (either.room_name, either.room_x, either.room_y) {
                    (Some(room_name), _, _) => Ok(LocalRoomPosition {
                        x: either.x,
                        y: either.y,
                        room_name,
                    }),
                    (_, Some(room_x), Some(room_y)) => Ok(EfficientFormat {
                        x: either.x,
                        y: either.y,
                        room_x,
                        room_y,
                    }
                    .into()),
                    (None, Some(_), None) => Err(D::Error::missing_field("room_y")),
                    (None, None, Some(_)) => Err(D::Error::missing_field("room_x")),
                    (None, None, None) => Err(D::Error::missing_field("roomName")),
                }
            } else {
                // we don't use EitherFormat here because in some binary formats like bincode,
                // there isn't the metadata necessary to figure out that a field with a given
                // name isn't present.
                EfficientFormat::deserialize(deserializer).map(Into::into)
            }
        }
    }
}
