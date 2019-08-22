//! Constants, most copied from [the game constants].
//!
//! Last updated on 2019-07-13, `068343753adf090fd1991944d2292be9e236b7dd` from
//! <https://github.com/screeps/common/commits/master/lib/constants.js>.
//!
//! Currently missing:
//! - FIND_DROPPED_ENERGY (deprecated in Screeps)
//! - OBSTACLE_OBJECT_TYPES
//! - WORLD_WIDTH / WORLD_HEIGHT (deprecated in Screeps)
//! - CONTROLLER_STRUCTURES
//! - REACTIONS
//! - BOOSTS
//! - POWER_INFO
//!
//! # Notes on Deserialization
//!
//! There are two general types of enum constants in this file. Some are
//! represented by integers in the game - and those are represented by integers
//! in screeps. Their [`serde::Deserialize`], `TryFrom<Value>` and
//! [`num_traits::FromPrimitive`] implementations will convert from these
//! integers.
//!
//! The other type is enums represented by strings in the game, but integers in
//! this repository. This change in representation is done for efficiency, as
//! transferring strings from JavaScript to Rust is much slower than a single
//! integer.
//!
//! This second type of enum will also implement [`serde::Deserialize`],
//! `TryFrom<Value>`, but these will not convert from the made-up integer
//! values, and will fail converting from the constant strings from the game.
//!
//! To convert from constant strings, you have two options, depending on the
//! context.
//!
//! If you need to manually consume from a value in JavaScript, there are two
//! utility JavaScript functions per enum. They generally take the form
//! `__TYPE_num_to_str` and `__TYPE_str_to_num`. For example,
//! `__structure_type_num_to_str` and `__structure_type_str_to_num` convert
//! between [`StructureType`] integer representations and string
//! representations. See documentation on enums for more conversion functions.
//!
//! To use these, call the functions in JavaScript, like so:
//!
//! ```no_run
//! use screeps::{game, traits::TryInto, StructureType};
//! use stdweb::js;
//!
//! let spawns = game::spawns::values();
//! let r: StructureType = (js! {
//!     return __structure_type_str_to_num(@{spawns[0].as_ref()}.structureType);
//! })
//! .try_into()
//! .expect("expected structure type to convert successfully");
//! ```
//!
//! If you need to consume strings already in Rust, use either the [`FromStr`]
//! trait, or one of the `deserialize_from_str` functions on each of these
//! constants.
//!
//! [the game constants]: https://github.com/screeps/common/blob/master/lib/constants.js
//! [`FromStr`]: std::str::FromStr
pub mod find;
pub mod look;
mod numbers;
mod small_enums;
mod types;

pub use self::{
    find::FindConstant,
    look::{Look, LookConstant},
    numbers::*,
    small_enums::*,
    types::*,
};
