#[macro_use]
extern crate enum_primitive;
#[macro_use]
extern crate log;
extern crate num_traits;
#[macro_use]
extern crate stdweb;

macro_rules! js_unwrap {
    ($($code:tt)*) => ((js! { return $($code)* }).try_into().expect(concat!("js_unwrap at ", line!(), " in ", file!())))
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
            use stdweb::unstable::TryInto;
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
