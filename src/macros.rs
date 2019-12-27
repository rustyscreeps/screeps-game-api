//! This file groups all macros used throughout the library. Since interop with
//! javascript often gets unwieldy, macros are used extensively to follow DRY
//! principles.
//!
//! The documentation tries to give a good enough picture of how the macros
//! should be used, but are in no way a formal description. For a better
//! understanding of the `macro_rule!` arcane yet simple syntax, have a look
//! at [`Macros, A Methodical Introduction`][macro-book]
//!
//! [macro-book]: https://danielkeep.github.io/tlborm/book/mbe-README.html

/// Used to get data from a javascript reference back into rust code.
///
/// Macro syntax (`$name` are expressions):
///
/// ```ignore
/// js_unwrap!($jsExpr)
/// ```
///
/// For reference, `js!()` is a macro that returns a `stdweb::Value` enum.
/// See <https://docs.rs/stdweb/0.4.8/stdweb/enum.Value.html>.
///
/// Here, `js_unwrap!()` takes any valid javascript expression (expresses a
/// value) and will attempt conversion to the receiving variable type using
/// `try_into`. For example:
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
/// `stdweb::Value::Number` which is convertible to a u32 and should work
/// without problem.
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
/// (If unavailable: <https://docs.rs/stdweb/0.4.8/stdweb/unstable/trait.TryFrom.html> )
///
/// Note: for unwrapping reference types, use [`js_unwrap_ref!`] to avoid
/// instanceof checks.
macro_rules! js_unwrap {
    ($($code:tt)*) => (
        crate::traits::TryInto::try_into(js! { return $($code)*; })
            .expect(concat!("js_unwrap at ", line!(), " in ", file!()))
    )
}

/// Macro similar to [`js_unwrap!`], but with fewer `instanceof` checks.
///
/// # Example
///
/// ```ignore
/// let x: Creep = js_unwrap_ref!(Game.creeps.John);
/// ```
///
/// This will generate code
///
/// ```
/// let x: Creep = js!({ return Game.creeps.John; }).cast_expected_type().expect(...);
/// ```
///
/// `cast_expected_type` will ensure that the return value is a
/// [`stdweb::Reference`], but it won't do any more than that. If the JavaScript
/// behaves incorrectly and returns something other than a Creep, and the
/// `"check-all-casts"` feature is not enabled, it will silently make a
/// [`screeps::Creep`] containing the wrong value which will fail when used.
macro_rules! js_unwrap_ref {
    ($($code:tt)*) => (
        crate::traits::IntoExpectedType::into_expected_type(js! { return $($code)*; })
            .expect(concat!("js_unwrap_ref at ", line!(), " in ", file!()))
    )
}

/// Macro used to encapsulate all screeps game objects
///
/// Macro syntax:
///
/// ```
/// reference_wrapper! {
///     #[reference(instance_of = "Creep")]
///     Creep,
///     #[reference(instance_of = "Room")],
///     Room,
///     // ...
/// }
/// ```
///
/// Screeps game objects, in javascript, can be accessed via stdweb's
/// `Reference` object. For each ident `objJ` mentioned, this macro:
///
/// - Creates a struct named `objX`;
/// - Uses `#[derive(Clone, ReferenceType)]` which implements these traits for
///   `objX`:
///   - `InstanceOf`
///   - `AsRef<Reference>`
///   - `ReferenceType`
///   - `Into<Reference>`
///   - `TryInto<Reference>`
///   - `TryFrom<Reference>`
///   - `TryFrom<&Reference>`
///   - `TryFrom<Value>`
///   - `TryFrom<&Value>`
/// - Implements `FromExpectedType<Reference>` for `objJ`
macro_rules! reference_wrappers {
    (
        $(
            $(#[ $attr:meta ])*
            $vis:vis struct $name:ident(...);
        )+
    ) => {
        $(
            #[derive(Clone, ReferenceType)]
            $(
                #[$attr]
            )*
            $vis struct $name(Reference);

            impl crate::traits::FromExpectedType<Reference> for $name {
                fn from_expected_type(reference: Reference) -> Result<Self, ConversionError> {
                    #[cfg(feature = "check-all-casts")]
                    {
                        $crate::traits::TryFrom::try_from(reference)
                    }
                    #[cfg(not(feature = "check-all-casts"))]
                    {
                        unsafe { Ok(stdweb::ReferenceType::from_reference_unchecked(reference)) }
                    }
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
///
/// ```ignore
/// simple_accessors! {
///     impl $struct_name {
///         pub fn $rust_method_name1() -> $rust_type1 = $js_field_name1;
///         pub fn $rust_method_name2() -> $rust_type2 = $js_field_name2;
///         ...
///     }
/// }
/// ```
macro_rules! simple_accessors {
    (impl $struct_name:ident {
        $(
            $vis:vis fn $method:ident () -> $ret:ty = $prop:ident;
        )+
    }) => (
        impl $struct_name {
            $(
                $vis fn $method(&self) -> $ret {
                    js_unwrap!(@{self.as_ref()}.$prop)
                }
            )*
        }
    )
}

/// Macro for mass implementing `StructureProperties`, `PartialEq` and `Eq` for
/// a type.
///
/// Macro syntax:
///
/// ```ignore
/// impl_structure_properties!{
///     $struct1,
///     $struct2,
///     ...
/// }
/// ```
///
/// This macro accepts a comma-separated list of types on which to implement the
/// unsafe `StructureProperties` trait on a screeps object.
/// From that implementation, the type gets the `id` method which is used to
/// implement `PartialEq` and `Eq`.
///
/// # Safety
/// The macro assumes that it is implementing the trait to a valid `Reference`
/// (See `reference_wrapper` macro) which will support all `StructureProperties`
/// methods.
macro_rules! impl_structure_properties {
    ( $( $struct_name:ty ),+ $(,)? ) => {$(
        unsafe impl StructureProperties for $struct_name {}
    )*};
}

/// Implements `id` method for Structures and Creep
///
/// This generates the implementation, for the structures given, of the `HasId`,
/// `PartialEq` and `Eq` traits. The last two are implemented using the `id()`
/// method.
///
/// Macro Syntax:
/// ```ignore
/// impl_has_id! {
///     $struct_name1;
///     $struct_name2;
///     ...
/// }
/// ```
macro_rules! impl_has_id {
    ($($struct_name:ty),+ $(,)?) => {$(
        unsafe impl HasId for $struct_name {}

        impl PartialEq for $struct_name {
            fn eq(&self, other: &$struct_name) -> bool {
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
/// ```ignore
/// creep_simple_generic_action! {
///     impl Creep {
///         pub fn $rust_method_name1($action_target_trait1) = js_method_name1();
///         pub fn $rust_method_name2($action_target_trait2) = js_method_name2();
///         ...
///     }
/// }
/// ```
///
/// For this macro, the last comma is facultative.
///
/// The generic comes from the fact that this implements the method to be able
/// to target any object that conforms to the `action_target_trait` trait.
macro_rules! creep_simple_generic_action {
    (
        impl $struct_name:ident {
            $(
                $vis:vis fn $method:ident($trait:ident) = $js_name:ident ();
            )+
        }
    ) => (
        impl $struct_name {
            $(
                $vis fn $method<T>(&self, target: &T) -> ReturnCode
                where
                    T: ?Sized + $trait,
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
/// ```ignore
/// creep_simple_generic_action! {
///     impl Creep {
///         pub fn $rust_method_name1($target_type1) = js_method_name1();
///         pub fn $rust_method_name2($target_type2) = js_method_name2();
///         ...
///     }
/// }
/// ```
///
/// For this macro, the last comma is facultative.
///
/// The concrete comes from the fact that this implements the method to be able
/// to target only the `type` given.
macro_rules! creep_simple_concrete_action {
    (
        impl $struct_name:ident {
            $(
                $vis:vis fn $method:ident($type:ty) = $js_name:ident ();
            )+
        }
    ) => (
        impl $struct_name {
            $(
                $vis fn $method(&self, target: &$type) -> ReturnCode {
                    js_unwrap!(@{self.as_ref()}.$js_name(@{target.as_ref()}))
                }
            )*
        }
    )
}

/// Declares an item with a doc attribute computed by some macro expression.
/// This allows documentation to be dynamically generated based on input.
/// Necessary to work around https://github.com/rust-lang/rust/issues/52607.
macro_rules! calculated_doc {
    (
        $(
            #[doc = $doc:expr]
            $thing:item
        )*
    ) => (
        $(
            #[doc = $doc]
            $thing
        )*
    );
}

macro_rules! typesafe_find_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path);
        )*
    ) => (
        $(
            calculated_doc! {
                #[doc = concat!(
                    "Zero-sized constant representing the `FIND_",
                    stringify!($constant_name),
                    "` constant."
                )]
                #[allow(bad_style)]
                #[derive(Copy, Clone, Debug, Default)]
                $vis struct $constant_name;
            }
            unsafe impl FindConstant for $constant_name {
                type Item = $result;

                #[inline]
                fn find_code(&self) -> i16 {
                    $value
                }
            }
        )*
    );
}

macro_rules! typesafe_look_constants {
    (
        $(
            $vis:vis struct $constant_name:ident = ($value:expr, $result:path, $conversion_method:expr);
        )*
    ) => (
        $(
            #[allow(bad_style)]
            $vis struct $constant_name;
            unsafe impl LookConstant for $constant_name {
                type Item = $result;

                fn convert_and_check_items(reference: ::stdweb::Value) -> Vec<Self::Item> {
                    ($conversion_method)(reference)
                        .expect(concat!("LookConstant ", stringify!($constant_name),
                               "expected correct type at ", line!(), " in ", file!()))
                }

                #[inline]
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
/// ```ignore
/// game_map_access!($rust_object_accessed1, $js_code_to_access1);
/// ```
///
/// Best used inside a module. It builds four functions, `keys`, `values`, `get`
/// and `hashmap`. For example, to retrieve a vector of all creeps names:
///
/// ```
/// screeps::game::creeps::keys();
/// ```
///
/// This macro defines functions for retrieving the `keys` (names) of the
/// collection, the `values` as `rust_object_accessedX` and a single object
/// via the `get` function.
macro_rules! game_map_access {
    ($type:path, $js_inner:expr $(,)?) => {
        use std::collections::HashMap;

        use crate::{objects};

        calculated_doc! {
            #[doc = concat!("Retrieve the full `HashMap<String, ",
                            stringify!($type),
                            ">`.")
            ]
            pub fn hashmap() -> HashMap<String, $type> {
                js_unwrap!($js_inner)
            }
        }

        /// Retrieve the string keys of this object.
        pub fn keys() -> Vec<String> {
            js_unwrap!(Object.keys($js_inner))
        }

        /// Retrieve all values in this object.
        pub fn values() -> Vec<$type> {
            js_unwrap_ref!(Object.values($js_inner))
        }

        /// Retrieve a specific value by key.
        pub fn get(name: &str) -> Option<$type> {
            js_unwrap_ref!($js_inner[@{name}])
        }
    };
}

/// Match on all variants of `Structure` and do the same thing for each of them.
macro_rules! match_structure_variants {
    ($source:expr, $name:ident => $action:expr) => {
        match $source {
            Structure::Container($name) => $action,
            Structure::Controller($name) => $action,
            Structure::Extension($name) => $action,
            Structure::Extractor($name) => $action,
            Structure::Factory($name) => $action,
            Structure::InvaderCore($name) => $action,
            Structure::KeeperLair($name) => $action,
            Structure::Lab($name) => $action,
            Structure::Link($name) => $action,
            Structure::Nuker($name) => $action,
            Structure::Observer($name) => $action,
            Structure::PowerBank($name) => $action,
            Structure::PowerSpawn($name) => $action,
            Structure::Portal($name) => $action,
            Structure::Rampart($name) => $action,
            Structure::Road($name) => $action,
            Structure::Spawn($name) => $action,
            Structure::Storage($name) => $action,
            Structure::Terminal($name) => $action,
            Structure::Tower($name) => $action,
            Structure::Wall($name) => $action,
        }
    };
}

/// Match on all variants of `StructureType` and construct `Structure` variants
/// from the same code for each of them.
macro_rules! construct_structure_variants {
    ($source:expr => $action:expr) => {
        match $source {
            StructureType::Container => Structure::Container($action),
            StructureType::Controller => Structure::Controller($action),
            StructureType::Extension => Structure::Extension($action),
            StructureType::Extractor => Structure::Extractor($action),
            StructureType::Factory => Structure::Factory($action),
            StructureType::InvaderCore => Structure::InvaderCore($action),
            StructureType::KeeperLair => Structure::KeeperLair($action),
            StructureType::Lab => Structure::Lab($action),
            StructureType::Link => Structure::Link($action),
            StructureType::Nuker => Structure::Nuker($action),
            StructureType::Observer => Structure::Observer($action),
            StructureType::PowerBank => Structure::PowerBank($action),
            StructureType::PowerSpawn => Structure::PowerSpawn($action),
            StructureType::Portal => Structure::Portal($action),
            StructureType::Rampart => Structure::Rampart($action),
            StructureType::Road => Structure::Road($action),
            StructureType::Spawn => Structure::Spawn($action),
            StructureType::Storage => Structure::Storage($action),
            StructureType::Terminal => Structure::Terminal($action),
            StructureType::Tower => Structure::Tower($action),
            StructureType::Wall => Structure::Wall($action),
        }
    };
}

/// Match on all variants of `Structure`, doing something wrapped in Some() for
/// some of them, and None for others.
macro_rules! match_some_structure_variants {
    ($source:expr, { $($allowed:ident),* $(,)* }, $name:ident => $action:expr) => {
        match $source {
            $(
                Structure::$allowed($name) => Some($action),
            )*
            _ => None,
        }
    };
}

/// Implements `Iterator` for `js_vec::IntoIter` or `js_vec::Iter`, using
/// `FromExpectedType` and panicking on incorrect types.
///
/// Accepts a list of types to implement the traits for. Each type must be a
/// single ident, optionally followed by `<'lifetime_param>` where
/// `lifetime_param` is a single named lifetime.
macro_rules! impl_js_vec_iterators_from_expected_type_panic {
    ($($name:ident $(<$single_life_param:lifetime>)*),+ $(,)?) => {
        $(
            impl<$($single_life_param, )* T> Iterator for $name<$($single_life_param, )* T>
            where
                T: FromExpectedType<Value>,
            {
                type Item = T;

                /// Gets the next item.
                ///
                /// # Panics
                ///
                /// Panics if the type is incorrect.
                fn next(&mut self) -> Option<Self::Item> {
                    if self.index as usize >= self.inner.len() {
                        None
                    } else {
                        let index = self.index;
                        self.index += 1;

                        Some(js_unwrap_ref!(@{AsRef::<Reference>::as_ref(&self.inner)}[@{index}]))
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let length = self.inner.len();
                    (length, Some(length))
                }
            }

            impl<$($single_life_param, )* T> ::std::iter::ExactSizeIterator
                for $name<$($single_life_param, )* T>
            where
                T: FromExpectedType<Value>,
            {
                fn len(&self) -> usize {
                    self.inner.len()
                }
            }
        )*
    }
}

/// Implements `Iterator` for `js_vec::IntoIter` or `js_vec::Iter`.
///
/// Accepts a list of types to implement the traits for. Each type must be a
/// single ident, optionally followed by `<'lifetime_param>` where
/// `lifetime_param` is a single named lifetime.
macro_rules! impl_js_vec_iterators_from_expected_type_with_result {
    ($($name:ident $(<$single_life_param:lifetime>)*),+ $(,)?) => {
        $(
            impl<$($single_life_param, )* T> Iterator for $name<$($single_life_param, )* T>
            where
                T: FromExpectedType<Value>,
            {
                type Item = Result<T, ConversionError>;

                fn next(&mut self) -> Option<Self::Item> {
                    if self.index as usize >= self.inner.len() {
                        None
                    } else {
                        let index = self.index;
                        self.index += 1;

                        Some(FromExpectedType::from_expected_type(
                            js!(@{AsRef::<Reference>::as_ref(&self.inner)}[@{index}])
                        ))
                    }
                }

                fn size_hint(&self) -> (usize, Option<usize>) {
                    let length = self.inner.len();
                    (length, Some(length))
                }
            }

            impl<$($single_life_param, )* T> ::std::iter::ExactSizeIterator
                for $name<$($single_life_param, )* T>
            where
                T: FromExpectedType<Value>,
            {
                fn len(&self) -> usize {
                    self.inner.len()
                }
            }
        )*
    }
}

/// Get a value from memory given a path, returning `None` if any thing along
/// the way does not exist.
///
/// # Examples
///
/// Get a reference with type u32 at the path creeps.John.count.
///
/// ```no_run
/// #[macro_use]
/// extern crate screeps;
///
/// # fn main() {
/// let mem = screeps::memory::root();
/// let val = mem_get!(mem.creeps.John.count.i32);
/// # }
/// ```
///
/// Get something using a variable path.
///
/// ```no_run
/// #[macro_use]
/// extern crate screeps;
///
/// # fn main() {
/// let mem = screeps::memory::root();
/// let creep_name = "John";
/// let what_to_get = "count";
/// let val1 = mem_get!(mem.creeps[creep_name][what_to_get].i32);
/// let val2 = mem_get!(mem.creeps[creep_name].count.i32);
/// assert_eq!(val1, val2);
/// # }
/// ```
///
/// Accepted suffixes for type are methods that exist on `MemoryReference`, such
/// as `num`, `int`, `string`, `bool`, `arr` and `dict`.
#[macro_export]
macro_rules! mem_get {
    // Macro entry point
    ($memory_reference:ident $($rest:tt)*) => {
        mem_get!(@so_far { Ok(Some(&$memory_reference)) } @rest $($rest)*)
    };
    // Access the last part with a variable
    (@so_far { $reference_so_far:expr } @rest [ $final_part_variable:expr ] . $accessor:ident) => {
        $reference_so_far.and_then(|opt| match opt {
            Some(v) => v.$accessor($final_part_variable),
            None => Ok(None),
        })
    };
    // Access the last part with a hardcoded ident
    (@so_far { $reference_so_far:expr } @rest . $final_part:ident . $accessor:ident) => {
        $reference_so_far.and_then(|opt| match opt {
            Some(v) => v.$accessor(stringify!($final_part)),
            None => Ok(None),
        })
    };
    // Access the next (but not last) part with a variable
    (@so_far { $reference_so_far:expr } @rest [ $next_part_variable:expr ] $($rest:tt)+) => {
        mem_get!(
            @so_far {
                $reference_so_far.and_then(|opt| match opt {
                    Some(v) => v.dict($next_part_variable),
                    None => Ok(None),
                })
            }
            @rest $($rest)*
        )
    };
    // Access the next (but not last) part with a hardcoded ident
    (@so_far { $reference_so_far:expr } @rest . $next_part:ident $($rest:tt)+) => {
        mem_get!(
            @so_far {
                $reference_so_far.and_then(|opt| match opt {
                    Some(v) => v.dict(stringify!($next_part)),
                    None => Ok(None),
                })
            }
            @rest $($rest)*
        )
    };
    ($($not_valid:tt)*) => {
        compile_error!(concat!("Unexpected usage of mem_get! usage: ", stringify!($($not_valid)*)))
    }
}

/// Set a value in memory given a path, creating dicts for intermediate places
/// if they do not exist.
///
/// # Return
///
/// This macro produces a `Result<(), ::screeps::memory::UnexpectedTypeError>`.
/// The error path will trigger if any of the intermediate memory keys exist but
/// are not dictionaries.
///
/// # Examples
///
/// Set Memory.creeps.John.count to 42.
///
/// ```no_run
/// #[macro_use]
/// extern crate screeps;
///
/// # fn main() {
/// let mem = screeps::memory::root();
/// mem_set!(mem.creeps.John.count = 42).unwrap();
/// # }
/// ```
///
/// Set something using a variable path.
///
/// ```no_run
/// #[macro_use]
/// extern crate screeps;
///
/// # fn main() {
/// let mem = screeps::memory::root();
/// let creep_name = "John";
/// let what_to_set = "count";
/// mem_set!(mem.creeps[creep_name][what_to_set] = 51).unwrap();
/// mem_set!(mem.creeps[creep_name].count = 52).unwrap();
/// # }
/// ```
#[macro_export]
macro_rules! mem_set {
    // Macro entry point
    ($memory_reference:ident $($rest:tt)*) => {
        mem_set!(
            @path_so_far { stringify!($memory_reference) }
            @so_far { Ok(&$memory_reference) }
            @rest $($rest)*
        )
    };
    // Perform the set given a variable for the last part of the path.
    (
        @path_so_far { $path_so_far:expr }
        @so_far { $reference_so_far:expr }
        @rest [ $final_part_variable:expr ] = $value:expr
    ) => {
        $reference_so_far.map(|v| v.set($final_part_variable, $value))
    };
    // Perform the set given a hardcoded ident for the last part of the path.
    (
        @path_so_far { $path_so_far:expr }
        @so_far { $reference_so_far:expr }
        @rest . $final_part:ident = $value:expr
    ) => {
        $reference_so_far.map(|v| v.set(stringify!($final_part), $value))
    };
    // Access the next (but not last) part with a variable ident.
    (
        @path_so_far { $path_so_far:expr }
        @so_far { $reference_so_far:expr }
        @rest [ $next_part_variable:expr ] $($rest:tt)+
    ) => {
        mem_set!(
            @path_so_far { concat!($path_so_far, ".", stringify!([$next_part_variable])) }
            @so_far {
                $reference_so_far.and_then(|v| v.dict_or_create($next_part_variable))
            }
            @rest $($rest)*
        )
    };
    // Access the next (but not last) part with a hardcoded ident
    (
        @path_so_far { $path_so_far:expr }
        @so_far { $reference_so_far:expr }
        @rest . $next_part:ident $($rest:tt)+
    ) => {
        mem_set!(
            @path_so_far { concat!($path_so_far, ".", stringify!($next_part)) }
            @so_far {
                $reference_so_far.and_then(|v| v.dict_or_create(stringify!($next_part)))
            }
            @rest $($rest)*
        )
    };
    ($($not_valid:tt)*) => {
        compile_error!(concat!("Unexpected usage of mem_set! usage: ", stringify!($($not_valid)*)))
    }
}
