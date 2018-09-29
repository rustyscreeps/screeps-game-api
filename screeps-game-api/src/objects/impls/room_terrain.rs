use {constants::Terrain, objects::RoomTerrain};

impl RoomTerrain {
    pub fn constructor(room_name: &str) -> Self {
        js_unwrap!{new Room.Terrain(@{room_name})}
    }

    pub fn get(&self, x: u32, y: u32) -> Terrain {
        js_unwrap!{@{self.as_ref()}.get(@{x}, @{y})}
    }

    pub fn get_raw_buffer(&self) -> Vec<u8> {
        js_unwrap!(@{self.as_ref()}.getRawBuffer())
    }
}
