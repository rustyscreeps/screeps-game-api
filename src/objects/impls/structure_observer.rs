use crate::{constants::ReturnCode, local::LocalRoomName, macros::*, objects::StructureObserver};

impl StructureObserver {
    pub fn observe_room(&self, room_name: LocalRoomName) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.observeRoom(@{room_name})}
    }
}
