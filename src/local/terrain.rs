use std::mem::MaybeUninit;

use js_sys::Uint8Array;

use crate::{constants::Terrain, objects::RoomTerrain};

use super::{xy_to_terrain_index, RoomXY, ROOM_AREA};

#[derive(Debug, Clone)]
pub struct LocalRoomTerrain {
    bits: Box<[u8; ROOM_AREA]>,
}

/// A matrix representing the terrain of a room, stored in Rust memory.
///
/// Use [`RoomTerrain`] if data stored in JavaScript memory is preferred.
impl LocalRoomTerrain {
    /// Gets the terrain at the specified position in this room.
    pub fn get_xy(&self, xy: RoomXY) -> Terrain {
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

    /// Creates a `LocalRoomTerrain` from the bytes that correspond to the
    /// room's terrain data.
    ///
    /// This is like the `RoomTerrain` type but performs all operations on data
    /// stored in wasm memory. Each byte in the array corresponds to the value
    /// of the `Terrain` at the given position.
    ///
    /// The bytes are in row-major order - that is they start at the top left,
    /// then move to the top right, and then start at the left of the next row.
    /// This is different from `LocalCostMatrix`, which is column-major.
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

        // SAFETY: if any allocations happen in rust, this buffer will be detached from
        // wasm memory and no longer writable - we use it immediately then discard it to
        // avoid this
        let js_buffer =
            unsafe { Uint8Array::view_mut_raw(data.as_mut_ptr() as *mut u8, ROOM_AREA) };

        // copy the terrain buffer into the memory backing the Uint8Array - this is the
        // boxed array, so this initializes it
        terrain
            .get_raw_buffer_to_array(&js_buffer)
            .expect("terrain data to copy");
        // data copied - explicitly drop the Uint8Array, so there's no chance it's used
        // again
        drop(js_buffer);
        // we've got the data in our boxed array, change to the needed type
        // SAFETY: `Box` has the same layout for sized types. `MaybeUninit<u8>` has the
        // same layout as `u8`. The arrays are the same size. The `MaybeUninit<u8>` are
        // all initialized because JS wrote to them.
        LocalRoomTerrain::new_from_bits(unsafe {
            std::mem::transmute::<Box<[MaybeUninit<u8>; ROOM_AREA]>, Box<[u8; ROOM_AREA]>>(data)
        })
    }
}
