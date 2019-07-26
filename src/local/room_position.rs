//! Structures relating to room name parsing.
use std::fmt;

use super::LocalRoomName;
use crate::objects::{HasPosition, RoomPosition};

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
        RoomPosition::new(self.x, self.y, &self.room_name.to_array_string())
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
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

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
                ReadableFormat::deserialize(deserializer).map(Into::into)
            } else {
                EfficientFormat::deserialize(deserializer).map(Into::into)
            }
        }
    }
}
