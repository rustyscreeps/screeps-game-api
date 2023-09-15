//! Pure-data structures relating to Screeps.
use std::ops::Range;

mod cost_matrix;
mod lodash_filter;
mod object_id;
mod position;
mod room_coordinate;
mod room_name;
mod terrain;

/// Represents two constants related to room names.
///
/// First, this is the constant added to room coordinates before they're stored
/// in the packed representation.
///
/// Second, `-HALF_WORLD_SIZE` is the minimum representable room name
/// coordinate, and `HALF_WORLD_SIZE - 1` is the maximum representable room name
/// coordinate.
const HALF_WORLD_SIZE: i32 = 128;

/// Valid room name coordinates.
const VALID_ROOM_NAME_COORDINATES: Range<i32> = -HALF_WORLD_SIZE..HALF_WORLD_SIZE;

/// Valid world positions
// not an inclusive range since the range for world coords is `-128..=127`, and
// the range for room coords is `0..=49`.
const VALID_WORLD_POSITIONS: Range<i32> =
    -((ROOM_SIZE as i32) * HALF_WORLD_SIZE)..(ROOM_SIZE as i32) * HALF_WORLD_SIZE;

use crate::ROOM_SIZE;

pub use self::{
    cost_matrix::*, lodash_filter::*, object_id::*, position::*, room_coordinate::*, room_name::*,
    terrain::*,
};
