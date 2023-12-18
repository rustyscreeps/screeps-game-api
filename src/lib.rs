//! Typed bindings to the Screeps: World in-game API for WASM Rust AIs.
//!
//! # Performance
//!
//! Due to the limited CPU available to the player in Screeps: World, the
//! performance implications of running WebAssembly are good to be aware of.
//!
//! WebAssembly instances have a dedicated linear memory, and data being passed
//! in or out must be copied. Additionally, Rust uses UTF-8 strings while
//! JavaScript uses UTF-16 strings, so any strings being copied must also be
//! converted between encodings.
//!
//! Additionally, compile time for the `WebAssembly.Module` can be considerable,
//! requiring a lot of CPU time on each initial startup tick (and potentially
//! requiring skipped ticks for larger bots). However, global resets are very
//! infrequent in most environments, so the impact of this isn't severe.
//!
//! After compilation, the WebAssembly environment has near-native performance,
//! and no Javascript garbage collection happening for its memory space,
//! allowing for faster execution of some types of workloads. Overall, the
//! advantages and disadvantages of WebAssembly in Screeps are relatively small,
//! especially when compared to the relatively high 0.2ms cost of game actions.
//!
//! # Data Persistence
//!
//! In the Screeps: World JavaScript environment, the `Memory` object is the
//! typical way to store data in a way that persists through the environment
//! resets that happen occasionally, either triggered by deploying a new version
//! of your code or due to natural expiration in the server. It provides a
//! wrapper that automatically deserializes the contents of `RawMemory` via the
//! `JSON.parse()` JavaScript function when accessed for the first time each
//! tick, then gets serialized by `JSON.stringify()` at the end of the tick.
//!
//! Using this untyped `Memory` object (or the reference to a part of it, which
//! can be obtained from the `memory` function on various game objects) from
//! within WebAssembly can be awkward, but is recommended if you need to
//! maintain compatibility with the default `Memory` object.
//!
//! An alternative that you may prefer is to use `RawMemory` instead, fetching
//! the stored data in string format using [`raw_memory::get`] and deserializing
//! within WebAssembly using [`serde`] or another serializion approach, then
//! serializing and using [`raw_memory::set`] to store the data.
//!
//! If you choose the `RawMemory` approach, be aware that some game methods
//! (notably [`StructureSpawn::spawn_creep`] and [`Creep::move_to`]) directly
//! store data in the `Memory` object; replacing the 'special' `Memory` object
//! with one that doesn't attempt to deserialize the contents of `RawMemory` may
//! be advisable if you're using it directly (note that this needs to be done
//! each tick to be effective).
//!
//! # Cargo Features
//!
//! ## `sim`
//!
//! Enables special-case handling of the unique room name present in the
//! simulator - must be enabled to build code that is compatible with that
//! environment. If this is enabled, the top-left valid room coordinate has the
//! name `sim`, otherwise it's named `W127N127`.
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
// warn when functions can safely be given the const keyword, see
// https://rust-lang.github.io/rust-clippy/master/index.html#/missing_const_for_fn
// unfortunately this warns for bindgen-attached functions so we can't leave it
// enabled

// #![warn(clippy::missing_const_for_fn)]

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
