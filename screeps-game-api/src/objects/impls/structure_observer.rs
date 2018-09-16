use {
    constants::ReturnCode,
    StructureObserver,
};

impl StructureObserver {
    pub fn observe_room(&self, room_name: &str) -> ReturnCode {
        js_unwrap!{@{self.as_ref()}.observeRoom(@{room_name})}
    }
}
