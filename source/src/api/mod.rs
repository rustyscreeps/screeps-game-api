macro_rules! get_from_js {
    ($name:ident -> { $($js_side:tt)* } -> $rust_ret_type:ty) => (
        get_from_js!($name() -> { $($js_side:tt)* } -> $rust_ret_type);
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
            (js! { return $($js_side)*; }).try_into().unwrap()
        }
    )
}

pub mod game;
pub mod objects;

pub use self::objects::*;

pub enum Part {
    Work,
    Move,
    Carry,
    Attack,
    RangedAttack,
    Heal,
    Tough,
    Claim,
}

enum_from_primitive! {
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum ReturnCode {
    Ok = 0,
    NotOwner = -1,
    NameExists = -3,
    Busy = -4,
    NotFound = -5,
    NotEnough = -6,
    InvalidTarget = -7,
    Full = -8,
    NotInRange = -9,
    INvalidArgs = -10,
    Tired = -11,
    NoBodypart = -12,
    RclNotEnough = -14,
    GclNotEnough = -15,
}
}
