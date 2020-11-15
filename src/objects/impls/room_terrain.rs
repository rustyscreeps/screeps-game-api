use wasm_bindgen::prelude::*;
use js_sys::{JsString, Uint8Array};

#[wasm_bindgen]
extern "C" {
    /// An object representing a room's terrain held in the javascript heap.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room-Terrain)
    #[wasm_bindgen(js_namespace = Room, js_name = Terrain)]
    pub type RoomTerrain;

    /// Gets the terrain for any room by name, regardless of current visibility of the room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.constructor)
    #[wasm_bindgen(constructor, js_namespace = Room)]
    pub fn new(room_name: &JsString) -> RoomTerrain;

    /// Get the type of type at given coordinates.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get(this: &RoomTerrain, x: u8, y: u8) -> u8;

    /// Get a copy of the underlying Uint8Array with the data about the room's terrain.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get_raw_buffer(this: &RoomTerrain) -> Uint8Array;

    /// Copy the contents of the array buffer into an existing [`Uint8Array`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Room.Terrain.get)
    #[wasm_bindgen(method)]
    pub fn get_raw_buffer_to_array(this: &RoomTerrain, destination: &Uint8Array) -> i8;
}



// use stdweb::UnsafeTypedArray;

// use crate::{
//     constants::{ReturnCode, Terrain},
//     local::RoomName,
//     objects::RoomTerrain,
//     traits::TryInto,
// };

// impl RoomTerrain {
//     pub fn constructor(room_name: RoomName) -> Self {
//         js_unwrap!(new Room.Terrain(@{room_name}))
//     }

//     pub fn get(&self, x: u32, y: u32) -> Terrain {
//         js_unwrap!(@{self.as_ref()}.get(@{x}, @{y}))
//     }

//     pub fn get_raw_buffer(&self) -> Vec<u8> {
//         let mut buffer: Vec<u8> = vec![0; 2500];
//         self.get_raw_buffer_to_vec(&mut buffer)
//             .expect("Panic in get_raw_buffer.");
//         buffer
//     }

//     pub fn get_raw_buffer_to_vec<'a>(
//         &self,
//         buffer: &'a mut Vec<u8>,
//     ) -> Result<&'a mut Vec<u8>, ReturnCode> {
//         let is_success: bool;
//         {
//             let arr: UnsafeTypedArray<'_, u8> =
//                 unsafe { UnsafeTypedArray::new(buffer.as_mut_slice()) };

//             is_success = js! {
//                 var bytes = @{arr};
//                 return @{self.as_ref()}.getRawBuffer(bytes) === bytes;
//             }
//             .try_into()
//             .unwrap();
//         }
//         if is_success {
//             Ok(buffer)
//         } else {
//             Err(ReturnCode::InvalidArgs)
//         }
//     }

//     pub fn get_raw_buffer_to_array<'a>(
//         &self,
//         buffer: &'a mut [u8; 2500],
//     ) -> Result<(), ReturnCode> {
//         let is_success: bool;
//         {
//             let arr: UnsafeTypedArray<'_, u8> = unsafe { UnsafeTypedArray::new(&buffer[0..2500]) };

//             is_success = js! {
//                 var bytes = @{arr};
//                 return @{self.as_ref()}.getRawBuffer(bytes) === bytes;
//             }
//             .try_into()
//             .unwrap();
//         }
//         if is_success {
//             Ok(())
//         } else {
//             Err(ReturnCode::InvalidArgs)
//         }
//     }
// }
