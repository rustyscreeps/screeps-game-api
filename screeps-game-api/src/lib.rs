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
mod macros;

pub mod constants;
pub mod game;
pub mod memory;
pub mod objects;
pub mod pathfinder;
mod positions;
pub mod raw_memory;

pub use constants::*;
pub use objects::*;
pub use positions::{LocalRoomName, LocalRoomPosition};

pub(crate) use stdweb::private::ConversionError;

/// Useful for `use screeps::prelude::*;` to bring in screeps traits. Does not contain any
/// structures in order to minimize namespace polution.
pub mod prelude {
    pub use objects::{
        HasPosition, 
        HasStore, 
        CanStoreEnergy,
        HasCooldown,
        CanDecay,
        OwnedStructureProperties, 
        RoomObjectProperties, 
        StructureProperties,
    };
}
