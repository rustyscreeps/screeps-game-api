use crate::{constants::ReturnCode, local::RoomName, objects::StructureObserver};

impl StructureObserver {
    pub fn observe_room(&self, room_name: RoomName) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.observeRoom(@{room_name})}
    }
}
