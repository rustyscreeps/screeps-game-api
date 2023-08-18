use crate::{RoomXY, Terrain, ROOM_AREA, ROOM_SIZE};
use core::fmt::Write;
use js_sys::Uint8Array;
use num_traits::FromPrimitive;

/// Derived from [`crate::constants::Terrain`], the bit flags use 0b00, 0b01,
/// and 0b10. If lava is added (very unlikely, but there is a constant), that
/// will use 0b1_00, and this code will need to be updated because all sorts of
/// things assume that it divides evenly.
const BITS_PER_TERRAIN: u32 = 2;

const TERRAIN_BIT_MASK: u8 = u8::MAX >> (u8::BITS - BITS_PER_TERRAIN);

// Flooring in integer division is intentional, we want the maximum number of
// terrain that can fit without overflowing a byte.
const TERRAIN_PER_BYTE: usize = (u8::BITS / BITS_PER_TERRAIN) as usize;
const PACKED_ROOM_BYTES: usize = ROOM_AREA / TERRAIN_PER_BYTE;
// TERRAIN_PER_BYTE * PACKED_ROOM_BYTES == ROOM_AREA is an important property.
// Make sure that it holds. This can't become false unless major breaking
// changes happen.
const _ASSERT_TERRAIN_PER_BYTE_MULTIPLE: () =
    assert!(TERRAIN_PER_BYTE * PACKED_ROOM_BYTES == ROOM_AREA);

pub struct RawRoomTerrain {
    /// Entries in this array consist of packed bits that represent the terrain
    /// of a room. Each coordinate is represented by [`BITS_PER_TERRAIN`] bits.
    /// SAFETY: The bits packed into each byte must be valid bit patterns for
    /// [`Terrain`].
    packed: Box<[u8; PACKED_ROOM_BYTES]>,
}

// Public API
impl RawRoomTerrain {
    /// Gets the terrain at the specified position in this room.
    #[must_use]
    pub fn get_xy(&self, xy: RoomXY) -> Terrain {
        let linear = terrain_xy_to_linear(xy);
        let (arr_idx, bit_offset) = linear_idx_to_bit_idx(linear);
        // SAFETY:
        // - arr_idx is based on a `RoomXY`, which is guaranteed to be in bounds of a
        //   room, so `terrain_xy_to_linear` is a linear index in bounds.
        // - `linear_idx_to_bit_idx` is guaranteed to give a result in bounds of the
        //   packed array if the linear index is in bounds.
        let byte = unsafe { self.packed.get_unchecked(arr_idx) };
        let terrain = Terrain::from_u8(
            (byte >> (u8::BITS - BITS_PER_TERRAIN - bit_offset)) & TERRAIN_BIT_MASK,
        );

        // SAFETY: `RawRoomTerrain` has a safety condition that the bytes that it
        // represents are valid for `Terrain`.
        unsafe { terrain.unwrap_unchecked() }
    }
}

// impl details
impl RawRoomTerrain {
    /// SAFETY: `data` must not contain any bytes that are not valid bit
    /// patterns for `Terrain`.
    #[must_use]
    pub(crate) unsafe fn new_from_js_buf(data: &Uint8Array) -> Self {
        let mut buf =
            Box::<[u8; ROOM_AREA]>::try_from(vec![0_u8; ROOM_AREA].into_boxed_slice()).unwrap();
        data.copy_to(&mut *buf);

        // SAFETY: valid byte condition upheld by caller.
        unsafe { Self::new_from_unpacked(&buf) }
    }

    /// SAFETY: `data` must not contain any bytes that are not valid bit
    /// patterns for `Terrain`.
    #[must_use]
    pub(crate) unsafe fn new_from_unpacked(data: &[u8; ROOM_AREA]) -> Self {
        // Note: The intermediate Vec and try_from are optimized out so that the
        // `unwrap` is unreachable, but it's here for the types to work out.
        let mut packed = Box::<[u8; PACKED_ROOM_BYTES]>::try_from(
            vec![0_u8; PACKED_ROOM_BYTES].into_boxed_slice(),
        )
        .unwrap();

        // Split the array into groups of TERRAIN_PER_BYTE.
        // LLVM is good at recognizing and optimizing out checks in `bytemuck` methods,
        // so this is a no-op at runtime.
        let split = bytemuck::cast_ref::<_, [[u8; TERRAIN_PER_BYTE]; PACKED_ROOM_BYTES]>(data);

        for (idx, unpacked) in split.iter().enumerate() {
            packed[idx] = RawRoomTerrain::pack_byte(*unpacked);
        }

        Self { packed }
    }

    #[must_use]
    #[inline]
    fn pack_byte(terrain_data: [u8; TERRAIN_PER_BYTE]) -> u8 {
        let mut packed = 0_u8;
        for data in terrain_data {
            packed <<= BITS_PER_TERRAIN;
            packed |= data & TERRAIN_BIT_MASK;
        }

        packed
    }

    /// Unpacks a byte of compressed data into an array of masked bits.
    #[must_use]
    #[inline]
    fn unpack_byte(byte: u8) -> [u8; TERRAIN_PER_BYTE] {
        let mut buf = [0_u8; TERRAIN_PER_BYTE];

        // Iterate in reverse to repeatedly mask off the end of the bits.
        let mut byte = byte;
        for idx in (0..TERRAIN_PER_BYTE).rev() {
            let masked_bits = byte & TERRAIN_BIT_MASK;
            byte >>= BITS_PER_TERRAIN;

            buf[idx] = masked_bits;
        }

        buf
    }
}

impl core::fmt::Debug for RawRoomTerrain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("RawRoomTerrain")?;

        // Handle pretty formatting with `#` by writing new lines and decoding the
        // terrain
        if f.alternate() {
            f.write_str(" [\n")?;

            for &byte in self.packed.iter() {
                f.write_str("    ")?;
                write!(f, "{:>#010b} ", byte)?;

                // build the string that describes the terrain in plain text
                let mut terrain_str = String::from("(");
                for (bit_idx, &terrain_bits) in RawRoomTerrain::unpack_byte(byte).iter().enumerate()
                {
                    let prefix = if bit_idx == 0 { "" } else { ", " };
                    terrain_str.write_str(prefix)?;
                    if let Some(terrain) = Terrain::from_u8(terrain_bits) {
                        write!(&mut terrain_str, "{:?}", terrain)?;
                    } else {
                        write!(&mut terrain_str, "INVALID <{:>02b}>", terrain_bits)?;
                    }
                }
                terrain_str.push(')');

                f.write_str(terrain_str.as_str())?;
                f.write_char('\n')?;
            }

            f.write_str("\n]")?;
            Ok(())
        } else {
            f.write_str(" [ ")?;

            // write the first byte without a leading comma, to simplify the x, x, ...
            // logic.
            write!(f, "{:>#010b}", self.packed[0])?;
            for byte in self.packed.iter().skip(1) {
                // left aligned, alternate formatting, 0 padded, 10 width (0b + 8 bit) binary
                // formatting.
                write!(f, ", {:>#010b}", byte)?;
            }
            f.write_str(" ]")?;
            Ok(())
        }
    }
}

/// Converts a [`RoomXY`] to a linear index suitable for raw terrain buffer
/// access.
///
/// Beacuse of [`RoomXY`]'s safety condition that the position is valid, this is
/// guaranteed to return a value in the range 0..ROOM_AREA.
///
/// Note: This function returns an index suitable for raw terrain access, but
/// not suitable for [`CostMatrix`], because the X/Y order in the backing
/// storage is swapped.
#[must_use]
#[inline]
const fn terrain_xy_to_linear(xy: RoomXY) -> usize {
    xy.y.u8() as usize * ROOM_SIZE as usize + xy.x.u8() as usize
}

/// Converts a linear room index into the appropriate index for the packed byte
/// array and the bits within that byte.
#[must_use]
#[inline]
const fn linear_idx_to_bit_idx(idx: usize) -> (usize, u32) {
    // int division to floor is intentional
    let arr_idx = idx / TERRAIN_PER_BYTE;
    let bit_offset = BITS_PER_TERRAIN * idx.rem_euclid(TERRAIN_PER_BYTE) as u32;

    (arr_idx, bit_offset)
}

#[cfg(test)]
mod tests {
    use crate::{local::terrain::RawRoomTerrain, RoomXY, Terrain, ROOM_AREA};

    #[test]
    fn test_packed_from_buffer() {
        let mut buf = [0; ROOM_AREA];
        buf[50 * 0 + 0] = Terrain::Plain as u8;
        buf[50 * 0 + 1] = Terrain::Swamp as u8;
        buf[50 * 0 + 2] = Terrain::Swamp as u8;
        buf[50 * 0 + 3] = Terrain::Swamp as u8;
        buf[50 * 0 + 4] = Terrain::Plain as u8;
        buf[50 * 0 + 5] = Terrain::Swamp as u8;
        buf[50 * 0 + 6] = Terrain::Plain as u8;
        buf[50 * 0 + 7] = Terrain::Swamp as u8;

        buf[50 * 1 + 0] = Terrain::Wall as u8;
        buf[50 * 1 + 1] = Terrain::Wall as u8;
        buf[50 * 1 + 2] = Terrain::Swamp as u8;
        buf[50 * 1 + 3] = Terrain::Plain as u8;
        buf[50 * 1 + 4] = Terrain::Wall as u8;
        buf[50 * 1 + 5] = Terrain::Plain as u8;
        buf[50 * 1 + 6] = Terrain::Wall as u8;
        buf[50 * 1 + 7] = Terrain::Swamp as u8;

        // SAFETY: The 0 defaults are a valid Terrain, and the buffer is populated by
        // casting the Terrain to a byte, so all bytes are valid.
        let raw = unsafe { RawRoomTerrain::new_from_unpacked(&buf) };

        // first row
        assert_eq!(
            raw.get_xy(RoomXY::try_from((0_u8, 0_u8)).unwrap()),
            Terrain::Plain
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((1_u8, 0_u8)).unwrap()),
            Terrain::Swamp
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((2_u8, 0_u8)).unwrap()),
            Terrain::Swamp
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((3_u8, 0_u8)).unwrap()),
            Terrain::Swamp
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((4_u8, 0_u8)).unwrap()),
            Terrain::Plain
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((5_u8, 0_u8)).unwrap()),
            Terrain::Swamp
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((6_u8, 0_u8)).unwrap()),
            Terrain::Plain
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((7_u8, 0_u8)).unwrap()),
            Terrain::Swamp
        );

        // second row
        assert_eq!(
            raw.get_xy(RoomXY::try_from((0_u8, 1_u8)).unwrap()),
            Terrain::Wall
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((1_u8, 1_u8)).unwrap()),
            Terrain::Wall
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((2_u8, 1_u8)).unwrap()),
            Terrain::Swamp
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((3_u8, 1_u8)).unwrap()),
            Terrain::Plain
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((4_u8, 1_u8)).unwrap()),
            Terrain::Wall
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((5_u8, 1_u8)).unwrap()),
            Terrain::Plain
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((6_u8, 1_u8)).unwrap()),
            Terrain::Wall
        );
        assert_eq!(
            raw.get_xy(RoomXY::try_from((7_u8, 1_u8)).unwrap()),
            Terrain::Swamp
        );
    }
}
