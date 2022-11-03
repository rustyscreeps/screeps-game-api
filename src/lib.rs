//! Typed bindings to the Screeps in-game API for WASM Rust AIs.
// //!
// //! # Cargo Features
// //!
// //! ## `check-all-casts`
// //!
// //! By default, `screeps-game-api` assumes that the Screeps JavaScript API calls
// //! return the types that they are documented to return and bypasses
// //! `instanceof` checks when constructing rust wrappers for those return values.
// //!
// //! To enable checking all types on all API calls, even ones when the screeps
// //! server reliably returns the expected type, depend on `screeps-game-api` with
// //! the `"check-all-casts"` feature flag:
// //!
// //! ```toml
// //! [dependencies]
// //! # ...
// //! screeps-game-api = { version = "0.9", features = ["check-all-casts"] }
// //! ```
#![recursion_limit = "128"]
// to build locally with doc_cfg enabled, run:
// `RUSTDOCFLAGS="--cfg docsrs" cargo +nightly doc --all-features`
#![cfg_attr(docsrs, feature(doc_auto_cfg))]

pub mod console;
pub mod constants;
pub mod enums;
pub mod game;
#[cfg(feature = "inter-shard-memory")]
pub mod inter_shard_memory;
pub mod js_collections;
pub mod local;
pub mod memory;
pub mod objects;
pub mod pathfinder;
pub mod prototypes;
pub mod raw_memory;
pub mod traits;

pub use crate::{
    constants::*, enums::*, game::*, js_collections::*, local::*, objects::*, pathfinder::*,
    raw_memory::*, traits::*,
};

#[cfg(feature = "inter-shard-memory")]
pub use crate::inter_shard_memory::*;

/// Traits which implement base functionalities for Screeps types.
///
/// # Example
///
/// ```no_run
/// use js_sys::{JsString, Reflect};
/// use screeps::{game, prelude::*, Creep};
///
/// let c = game::creeps().get(String::from("Bob")).unwrap();
///
/// // `HasId` trait brought in from prelude
/// let id = c.try_id().unwrap();
/// ```
///
/// This module contains all base functionality traits, and no structures.
pub mod prelude {
    pub use crate::traits::*;
}
