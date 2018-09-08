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

/// Used to get data from a javascript reference back into rust code. 
/// 
/// Macro syntax (`$name` are expressions): 
/// js_unwrap!($jsExpr)
/// 
/// For reference, `js!()` is a macro that returns a `stdweb::Value` enum.
/// https://docs.rs/stdweb/0.4.8/stdweb/enum.Value.html
/// 
/// Here, `js_unwrap!()` takes any valid javascript expression (expresses a value) 
/// and will attempt conversion to the receiving variable type using `try_into`. 
/// For example:
/// ```
/// let s: u32 = js_unwrap!(Game.time);
/// ```
/// 
/// This will be be converted to 
/// ```
/// let s: u32 = js!(return Game.time;).try_into().expect('Some Err Msg');
/// ```
/// 
/// Since `Game.time` returns a javascript `number`, `js!` spits out a
/// `stdweb::Value::Number` which is convertible to a u32 and should work without
/// problem.
/// 
/// A non-exhaustive list of types that work (use your judgement)
/// 
///   js      |  rust
/// ------------------------
/// Number    | u32, i32, f32
/// String    | String
/// bool      | Bool
/// 
/// For the full list, see the documentation for [`stdweb::unstable::TryFrom`].
/// (If unavailable: https://docs.rs/stdweb/0.4.8/stdweb/unstable/trait.TryFrom.html )
macro_rules! js_unwrap {
    ($($code:tt)*) => (
        ::stdweb::unstable::TryInto::try_into(js! { return $($code)* })
            .expect(concat!("js_unwrap at ", line!(), " in ", file!()))
    )
}

/// Creates a getter method to unwrap a field of a javascript object.
/// 
/// Macro Syntax (`$name` are expressions): 
/// get_from_js!($method_name -> {$js_statement} -> $rust_type)
/// get_from_js!($method_name($param1, $param2, ...) -> {$js_statement} -> $rust_type)
/// 
/// Building on top of `js_unwrap!()`, this creates an accessor to a javascript
/// object method or attribute. 
/// 
/// # Example
/// ```
/// get_from_js!(
///     limit -> {
///         Game.cpu.limit
///     } -> u32
/// )
/// ```
/// 
/// Will become:
/// ```
/// pub fn limit() -> u32{
///     js_unwrap!(Game.cpu.limit)
/// }
/// ```
/// which would best be used inside the implementation for `cpu` in this case.
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

/// Useful for `use screeps::prelude::*;` to bring in screeps traits. Does not contain any
/// structures in order to minimize namespace polution.
pub mod prelude {
    pub use objects::{
        HasPosition, HasStore, OwnedStructureProperties, RoomObjectProperties, StructureProperties,
    };
}
