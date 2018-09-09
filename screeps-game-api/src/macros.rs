//! This file groups all macros used throughout the library. Since interop with
//! javascript often gets unwieldy, macros are used extensively to follow DRY
//! principles.
//! 
//! The documentation tries to give a good enough picture of how the macros
//! should be used, but are in no way a formal description. For a better
//! understanding of the `macro_rule!` arcane yet simple syntax, have a look
//! at [`Macros, A Methodical Introduction`](https://danielkeep.github.io/tlborm/book/mbe-README.html).

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

/// Macro used to encapsulate all screeps game objects
/// 
/// Macro syntax: 
/// reference_wrapper!{
///     $obj1,
///     $obj2,
///     ...
/// }
/// 
/// Screeps game objects, in javascript, can be accessed via stdweb's `Reference`
/// object. This macro: 
///   - Creates a struct named `objX`;
///   - Implements traits `AsRef<Reference>`, `TryFrom<Value>` for `objX`
///   - Implements trait `From<objX>` for `Reference`
macro_rules! reference_wrappers {
    ($($name:ident),* $(,)*) => {
        $(
            #[derive(Clone)]
            pub struct $name(Reference);

            impl AsRef<Reference> for $name {
                fn as_ref(&self) -> &Reference {
                    &self.0
                }
            }
            impl From<$name> for Reference {
                fn from(wrapper: $name) -> Reference {
                    wrapper.0
                }
            }
            impl TryFrom<Value> for $name {
                type Error = <Value as TryInto<Reference>>::Error;

                fn try_from(v: Value) -> Result<$name, Self::Error> {
                    Ok($name(v.try_into()?))
                }
            }
        )*
    };
}

/// Automatically creates simple accessors to fields of screep objects
/// 
/// On top of an object created from `reference_wrapper!`, this macro creates an
/// implementation of the struct for a collection of fields from the screeps
/// object. 
/// 
/// Method Syntax: 
/// simple_accessor! {
///     $struct_name;
///     ($rust_method_name1 -> $js_field_name1 -> $rust_type1),
///     ($rust_method_name2 -> $js_field_name2 -> $rust_type2),
///     ...
/// }
macro_rules! simple_accessors {
    ($struct_name:ident; $(($method:ident -> $prop:ident -> $ret:ty)),* $(,)*) => (
        impl $struct_name {
            $(
                pub fn $method(&self) -> $ret {
                    js_unwrap!(@{self.as_ref()}.$prop)
                }
            )*
        }
    )
}

/// Implements the unsafe trait RoomObjectProperties for a Structure struct 
/// 
/// Macro Syntax: 
/// impl_room_object_properties!{
///     $structure1,
///     $structure2,
///     ...
/// }
macro_rules! impl_room_object_properties {
    ($($struct_name:ident),* $(,)*) => {
        $(
            unsafe impl RoomObjectProperties for $struct_name {
                fn try_from(obj: RoomObject) -> Option<Self> {
                    let is_me = js_unwrap!(@{obj.as_ref()} instanceof $struct_name);
                    if is_me {
                        Some($struct_name(obj.0))
                    } else {
                        None
                    }
                }
            }
        )*
    };
}

/// Macro for mass implementing `StructureProperties`, `PartialEq` and `Eq` for a type. 
/// 
/// Macro syntax:
/// impl_structure_properties!{
///     $struct1,
///     $struct2,
///     ...
/// }
/// 
/// This macro accepts a comma-separated list of types on which to implement the unsafe `StructureProperties` trait on
/// a screeps object. 
/// From that implementation, the type gets the `id` method which is used to implement `PartialEq` and `Eq`.
/// 
/// # Safety
/// The macro assumes that it is implementing the trait to a valid `Reference` 
/// (See `reference_wrapper` macro) which will support all `StructureProperties` methods.
/// 
macro_rules! impl_structure_properties {
    ( $( $struct_name:ty ),+ ) => {$(
        unsafe impl StructureProperties for $struct_name {}
        
        impl PartialEq for $struct_name {
            fn eq(&self, other: &$struct_name) -> bool{
                self.id() == other.id()
            }
        }

        impl Eq for $struct_name {}
    )*};
}

/// Implements action methods for creeps
/// 
/// This macro is used to implement generic `creep` methods that returns a 
/// `ReturnCode`, a number indicating the status of the action requested. 
/// 
/// Macro Syntax:
/// creep_simple_generic_action!{
///     ($rust_method_name1($action_target_trait1) -> js_method_name1),
///     ($rust_method_name2($action_target_trait2) -> js_method_name2),
///     ...
/// }
/// 
/// For this macro, the last comma is facultative.
/// 
/// The generic comes from the fact that this implements the method to be able to
/// target any object that conforms to the `action_target_trait` trait.
macro_rules! creep_simple_generic_action {
    ($(($method:ident($trait:ident) -> $js_name:ident)),* $(,)*) => (
        impl Creep {
            $(
                pub fn $method<T>(&self, target: &T) -> ReturnCode
                where
                    T: $trait,
                {
                    js_unwrap!(@{self.as_ref()}.$js_name(@{target.as_ref()}))
                }
            )*
        }
    )
}

/// Implements action methods for creeps
/// 
/// This macro is used to implement concrete `creep` methods that returns a 
/// `ReturnCode`, a number indicating the status of the action requested. 
/// 
/// Macro Syntax:
/// creep_simple_generic_action!{
///     ($rust_method_name1($target_type1) -> js_method_name1),
///     ($rust_method_name2($target_type2) -> js_method_name2),
///     ...
/// }
/// 
/// For this macro, the last comma is facultative.
/// 
/// The concrete comes from the fact that this implements the method to be able to
/// target only the `type` given.
macro_rules! creep_simple_concrete_action {
    ($(($method:ident($type:ty) -> $js_name:ident)),* $(,)*) => (
        impl Creep {
            $(
                pub fn $method(&self, target: &$type) -> ReturnCode
                {
                    js_unwrap!(@{self.as_ref()}.$js_name(@{target.as_ref()}))
                }
            )*
        }
    )
}

macro_rules! js_unwrap_array {
    ($($code:tt)*) => ({
        // TODO: should we do an assertion in the JS code here?
        let v = js_unwrap!($($code)*);

        unsafe { ::objects::impls::utils::game_array_to_vec(v) }
            .expect(concat!("js_unwrap_array at ", line!(), " in ", file!()))
    })
}

macro_rules! typesafe_find_constants {
    (
        $($constant_name:ident, $value:expr, $result:path;)*
    ) => (
        $(
            #[allow(bad_style)]
            pub struct $constant_name;
            unsafe impl FindConstant for $constant_name {
                type Item = $result;

                fn find_code(&self) -> i32 {
                    $value
                }
            }
        )*
    );
}

macro_rules! typesafe_look_constants {
    (
        $($constant_name:ident, $value:expr, $result:path;)*
    ) => (
        $(
            #[allow(bad_style)]
            pub struct $constant_name;
            unsafe impl LookConstant for $constant_name {
                type Item = $result;

                fn look_code(&self) -> Look {
                    $value
                }
            }
        )*
    );
}

// Todo: this way of handling a return object isn't consistent with some others
// used elsewhere (eg: signs)
/// Creates accessors for the main game collections
/// 
/// Macro syntax:
/// game_map_access!{
///     $rust_mod_name1, $rust_object_accessed1, $js_code_to_access1;
///     $rust_mod_name2, $rust_object_accessed2, $js_code_to_access2;
///     ...
/// }
/// 
/// Builds a module for often accessed collections. Those can then be accesed
/// via functions. For example, to retreive a vector of all creeps names: 
/// ```
/// screeps::game::creeps.keys();
/// ```
/// 
/// This macro defines functions for retreiving the `keys` (names) of the
/// collection, the `values` as `rust_object_accessedX` and a single object
/// via the `get` function.
/// 
macro_rules! game_map_access {
    (
        $(
            $(
                    #[$attr:meta]
            )*
            ($mod_name:ident, $type:path, $js_inner:expr) $(,)*
        ),* $(,)*
    ) => {
        $(
            $(
                    #[$attr]
            )*
            pub mod $mod_name {
                use objects;

                /// Retrieve the string keys of this object.
                pub fn keys() -> Vec<String> {
                    js_unwrap!(Object.keys($js_inner))
                }

                /// Retrieve all values in this object.
                pub fn values() -> Vec<$type> {
                    js_unwrap!(Object.values($js_inner))
                }

                /// Retrieve a specific value by key.
                pub fn get(name: &str) -> Option<$type> {
                    js_unwrap!($js_inner[@{name}])
                }
            }
        )*
    };
}
