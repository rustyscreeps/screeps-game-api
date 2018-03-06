#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
extern crate num_traits;
extern crate serde;
#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate stdweb;

macro_rules! js_unwrap {
    ($($code:tt)*) => (
        ::stdweb::unstable::TryInto::try_into(js! { return $($code)* })
            .expect(concat!("js_unwrap at ", line!(), " in ", file!()))
    )
}

macro_rules! get_from_js {
    ($name:ident -> { $js_side:expr } -> $rust_ret_type:ty) => (
        get_from_js!($name() -> { $js_side } -> $rust_ret_type);
    );
    (
        $name:ident(
            $($param_ident:ident: $param_ty:ty),*
        ) -> {
            $($js_side:tt)*
        } -> $rust_ret_type:ty
    ) => (
        pub fn $name(
            $($param_ident: $param_ty),*
        ) -> $rust_ret_type {
            js_unwrap!($($js_side)*)
        }
    )
}

pub mod game;
pub mod objects;
pub mod constants;
pub mod memory;
pub mod raw_memory;

pub use self::objects::*;
pub use self::constants::*;

/// Useful for `use screeps::prelude::*;` to bring in screeps traits.
pub mod prelude {
    pub use objects::{HasPosition, HasStore, OwnedStructureProperties, RoomObjectProperties,
                      StructureProperties};
}
