use crate::{local::RoomName, macros::*, objects::Nuke};

simple_accessors! {
    impl Nuke {
        // id from HasID
        pub fn launch_room_name() -> RoomName = launchRoomName;
        pub fn time_to_land() -> u32 = timeToLand;
    }
}
