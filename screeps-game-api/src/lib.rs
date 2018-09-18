//! `screeps-game-api`
//!
//! # Cargo Features
//!
//! ## `check-all-casts`
//!
//! By default, `screeps-game-api` assumes that the Screeps JavaScript API calls return the types
//! that they are documented to return and bypasses `instanceof` checks when constructing rust
//! wrappers for those return values.
//!
//! To enable checking all types on all API calls, even ones when the screeps server reliably
//! returns the expected type, depend on `screeps-game-api` with the `"check-all-casts"` feature
//! flag:
//!
//! ```toml
//! [dependencies]
//! # ...
//! screeps-game-api = { version = "0.2", features = ["check-all-casts"] }
//! ```
#![recursion_limit = "128"]
#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
extern crate num_traits;
#[macro_use]
extern crate scoped_tls;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;
#[macro_use]
extern crate stdweb_derive;

#[macro_use]
mod macros;

pub mod constants;
pub mod game;
pub mod memory;
pub mod objects;
pub mod pathfinder;
mod positions;
pub mod raw_memory;
pub mod traits;

pub use constants::*;
pub use objects::*;
pub use positions::{LocalRoomName, LocalRoomPosition};
pub use traits::{FromExpectedType, IntoExpectedType};

pub(crate) use stdweb::private::ConversionError;

/// Useful for `use screeps::prelude::*;` to bring in screeps traits. Does not contain any
/// structures in order to minimize namespace polution.
pub mod prelude {
    pub use objects::{
        CanDecay, CanStoreEnergy, HasCooldown, HasPosition, HasStore, OwnedStructureProperties,
        RoomObjectProperties, StructureProperties,
    };
}
