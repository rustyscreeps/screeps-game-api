//! Typed bindings to the Screeps in-game API for WASM Rust AIs.
//!
//! # Cargo Features
//!
//! ## `generate-pixel`
//!
//! Enables the function to generate pixels, which is only present on the
//! Screeps: World official servers.
//!
//! ## `inter-shard-memory`
//!
//! Enables interacting with `IntershardMemory`, which is not present in most
//! private server environments.
//!
//! ## `score`
//!
//! Enables the score resource and entities, introduced for Screeps Seasonal's
//! first season.
//!
//! ## `symbols`
//!
//! Enables the symbol resources and entities, introduced for Screeps Seasonal's
//! second season.
//!
//! ## `thorium`
//!
//! Enables the thorium resource and reactor object, introduced for Screeps
//! Seasonal's fifth season.
//!
//! ## `unsafe-return-conversion`
//!
//! Enables return code conversion from game functions that presumes all return
//! code values are in the expected ranges skipping checks, and risks undefined
//! behavior if they are not.
//!
//! ## `mmo`
//!
//! Enables the `generate-pixel` and `inter-shard-memory` features, which are
//! present on the Screeps: World official servers but not on private servers.
//!
//! ## `seasonal-season-1`
//!
//! Enables the `score` feature, a mechanic introduced for Screeps Seasonal's
//! first season, as well as enabling constants relevant to season 1.
//!
//! ## `seasonal-season-2`
//!
//! Enables the `symbols` feature, a mechanic introduced for Screeps Seasonal's
//! second season, as well as enabling constants relevant to season 2.
//!
//! ## `seasonal-season-5`
//!
//! Enables the `thorium` feature, a mechanic introduced for Screeps Seasonal's
//! fifth season, as well as enabling constants relevant to season 5.
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
pub(crate) mod prototypes;
pub mod raw_memory;
pub mod traits;

pub use crate::{constants::*, enums::*, js_collections::*, local::*, objects::*, traits::*};

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
    pub use crate::{js_collections::*, traits::*};
}
