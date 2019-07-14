use stdweb::UnsafeTypedArray;

use {
    constants::{ReturnCode, Terrain},
    objects::RoomTerrain,
    traits::TryInto,
};

impl RoomTerrain {
    pub fn constructor(room_name: &str) -> Self {
        js_unwrap!(new Room.Terrain(@{room_name}))
    }

    pub fn get(&self, x: u32, y: u32) -> Terrain {
        js_unwrap!(@{self.as_ref()}.get(@{x}, @{y}))
    }

    pub fn get_raw_buffer(&self) -> Vec<u8> {
        let mut buffer: Vec<u8> = vec![0; 2500];
        self.get_raw_buffer_to_vec(&mut buffer)
            .expect("Panic in get_raw_buffer.");
        buffer
    }

    pub fn get_raw_buffer_to_vec<'a>(
        &self,
        buffer: &'a mut Vec<u8>,
    ) -> Result<&'a mut Vec<u8>, ReturnCode> {
        let is_success: bool;
        {
            let arr: UnsafeTypedArray<u8> = unsafe { UnsafeTypedArray::new(buffer.as_mut_slice()) };

            is_success = js! {
                var bytes = @{arr};
                return @{self.as_ref()}.getRawBuffer(bytes) === bytes;
            }
            .try_into()
            .unwrap();
        }
        if is_success {
            Ok(buffer)
        } else {
            Err(ReturnCode::InvalidArgs)
        }
    }
}
