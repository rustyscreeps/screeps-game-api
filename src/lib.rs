//! Typed bindings to the Screeps in-game API for WASM Rust AIs.
//!
//! # Cargo Features
//!
//! ## `check-all-casts`
//!
//! By default, `screeps-game-api` assumes that the Screeps JavaScript API calls
//! return the types that they are documented to return and bypasses
//! `instanceof` checks when constructing rust wrappers for those return values.
//!
//! To enable checking all types on all API calls, even ones when the screeps
//! server reliably returns the expected type, depend on `screeps-game-api` with
//! the `"check-all-casts"` feature flag:
//!
//! ```toml
//! [dependencies]
//! # ...
//! screeps-game-api = { version = "0.3", features = ["check-all-casts"] }
//! ```
#![recursion_limit = "128"]
// to build locally with doc_cfg enabled, run:
// `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

#[macro_use]
extern crate stdweb;

#[macro_use]
pub mod macros;

pub mod constants;
pub mod game;
#[cfg(feature = "inter-shard-memory")]
pub mod inter_shard_memory;
pub mod js_collections;
pub mod local;
pub mod memory;
pub mod objects;
pub mod pathfinder;
pub mod raw_memory;
pub mod traits;

pub use stdweb::private::ConversionError;

pub use crate::{
    constants::*,
    js_collections::JsVec,
    local::{ObjectId, Position, RawObjectId, RawObjectIdParseError, RoomName, RoomNameParseError},
    objects::*,
    traits::{FromExpectedType, IntoExpectedType},
};

/// An alias for `Position` for those used to the JavaScript `RoomPosition`
/// type.
pub type RoomPosition = Position;

/// Traits which implement base functionalities for Screeps types.
///
/// # Example
///
/// ```no_run
/// use screeps::prelude::*;
///
/// let c = screeps::game::creeps::get("Bob").unwrap();
///
/// // `HasId` trait brought in from prelude
/// let id = c.id();
/// ```
///
/// This module contains all base functionality traits, and no structures.
pub mod prelude {
    pub use crate::objects::{
        CanDecay, HasCooldown, HasId, HasPosition, HasStore, OwnedStructureProperties,
        RoomObjectProperties, SharedCreepProperties, StructureProperties,
    };
}
