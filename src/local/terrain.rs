use std::mem::MaybeUninit;

use js_sys::Uint8Array;

use crate::{constants::Terrain, objects::RoomTerrain};

use super::{xy_to_terrain_index, RoomXY, ROOM_AREA};

pub struct LocalRoomTerrain {
    bits: Box<[u8; ROOM_AREA]>,
}

impl LocalRoomTerrain {
    /// Gets the terrain at the specified position in this room.
    pub fn get(&self, xy: RoomXY) -> Terrain {
        // SAFETY: RoomXY is always a valid coordinate.
        let byte = unsafe { self.bits.get_unchecked(xy_to_terrain_index(xy)) };
        // not using Terrain::from_u8() because `0b11` value, wall+swamp, happens
        // in commonly used server environments (notably the private server default
        // map), and is special-cased in the engine code; we special-case it here
        match byte & 0b11 {
            0b00 => Terrain::Plain,
            0b01 | 0b11 => Terrain::Wall,
            0b10 => Terrain::Swamp,
            // Should be optimized out
            _ => unreachable!("all combinations of 2 bits are covered"),
        }
    }

    pub fn new_from_bits(bits: Box<[u8; ROOM_AREA]>) -> Self {
        Self { bits }
    }
}

impl From<RoomTerrain> for LocalRoomTerrain {
    fn from(terrain: RoomTerrain) -> LocalRoomTerrain {
        // create an uninitialized array of the correct size
        let mut data: Box<[MaybeUninit<u8>; ROOM_AREA]> =
            Box::new([MaybeUninit::uninit(); ROOM_AREA]);
        // create a Uint8Array mapped to the same point in wasm linear memory as our
        // uninitialized boxed array
        let js_buffer =
            unsafe { Uint8Array::view_mut_raw(data.as_mut_ptr() as *mut u8, ROOM_AREA) };
        // SAFETY: it's important to not allocate _anything_ in rust memory now that
        // we've created the Uint8Array pointed at an arbitrary wasm memory location;
        // we'd overwrite the wrong memory if the array moved due to allocation!

        // copy the terrain buffer into the memory backing the Uint8Array - this is the
        // boxed array, so this initializes it
        terrain
            .get_raw_buffer_to_array(&js_buffer)
            .expect("terrain data to copy");
        // data copied - explicitly drop the Uint8Array, so there's no chance it's used
        // again
        drop(js_buffer);
        // we've got the data in our boxed array, change to the needed type
        // SAFETY: `Box` has the same layout for sized types. `MaybeUninit<u8>` has the same layout as `u8`. The arrays are the same size. The `MaybeUninit<u8>` are all initialized because JS wrote to them.
        LocalRoomTerrain::new_from_bits(unsafe { std::mem::transmute::<Box<[MaybeUninit<u8>; ROOM_AREA]>, Box<[u8; ROOM_AREA]>>(data) })
    }
}
