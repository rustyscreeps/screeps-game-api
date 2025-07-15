use std::mem::MaybeUninit;

use js_sys::Uint8Array;

use crate::{
    constants::{Terrain, ROOM_AREA},
    objects::RoomTerrain,
};

use super::RoomXY;

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
        let byte = self.bits[xy.y][xy.x];
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

    /// Gets a slice of the underlying bytes that comprise the room's terrain
    /// data.
    ///
    /// The bytes are in row-major order - that is they start at the top left,
    /// then move to the top right, and then start at the left of the next row.
    /// This is different from `LocalCostMatrix`, which is column-major.
    pub fn get_bits(&self) -> &[u8; ROOM_AREA] {
        &self.bits
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

#[cfg(test)]
mod test {
    use super::*;
    use crate::constants::{ROOM_AREA, ROOM_SIZE};

    #[test]
    pub fn addresses_data_in_row_major_order() {
        // Initialize terrain to be all plains
        let mut raw_terrain_data = Box::new([0; ROOM_AREA]);

        // Adjust (1, 0) to be a swamp; in row-major order this is the second element
        // (index 1) in the array; in column-major order this is the 51st
        // element (index 50) in the array.
        raw_terrain_data[1] = 2; // Terrain::Swamp has the numeric representation 2

        // Construct the local terrain object
        let terrain = LocalRoomTerrain::new_from_bits(raw_terrain_data);

        // Pull the terrain for location (1, 0); if it comes out as a Swamp, then we
        // know the get_xy function pulls data in row-major order; if it comes
        // out as a Plain, then we know that it pulls in column-major order.
        let xy = unsafe { RoomXY::unchecked_new(1, 0) };
        let tile_type = terrain.get_xy(xy);
        assert_eq!(Terrain::Swamp, tile_type);
    }

    #[test]
    pub fn get_bits_returns_a_byte_array_that_can_reconstitute_the_local_terrain() {
        // Initialize terrain to be all plains
        let mut raw_terrain_data = Box::new([0; ROOM_AREA]);

        // Adjust terrain to be heterogeneous
        for i in 0..ROOM_AREA {
            // Safety: mod 3 will always be a valid u8
            let tile_type: u8 = (i % 3) as u8; // Range: 0, 1, 2 -> Plains, Wall, Swamp
            raw_terrain_data[i] = tile_type;
        }

        // Construct the local terrain object
        let terrain = LocalRoomTerrain::new_from_bits(raw_terrain_data);

        // Grab the bits
        let bits = *terrain.get_bits();

        // Build the new terrain from the copied bits
        let new_terrain = LocalRoomTerrain::new_from_bits(Box::new(bits));

        // Iterate over all room positions and verify that they match in both terrain
        // objects
        for x in 0..ROOM_SIZE {
            for y in 0..ROOM_SIZE {
                // Safety: x and y are both explicitly restricted to room size
                let xy = unsafe { RoomXY::unchecked_new(x, y) };
                assert_eq!(terrain.get_xy(xy), new_terrain.get_xy(xy));
            }
        }
    }
}
