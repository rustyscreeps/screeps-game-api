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
            (js! { return $($js_side)*; }).try_into().unwrap()
        }
    )
}

pub mod game;
pub mod objects;
pub mod constants;

pub use self::objects::*;
pub use self::constants::*;

pub struct BodyPart {
    pub boost: Option<String>,
    pub ty: Part,
    pub hits: i32,
}
