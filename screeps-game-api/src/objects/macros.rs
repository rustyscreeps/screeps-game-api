macro_rules! reference_wrappers {
    ($name:ident) => {
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
    };
    ($($name:ident),* $(,)*) => {
        $(
            reference_wrappers!($name);
        )*
    };
}

macro_rules! simple_accessors {
    ($struct_name:ident; $(($method:ident -> $prop:ident -> $ret:ty)),* $(,)*) => (
        impl $struct_name {
            $(
                pub fn $method(&self) -> $ret {
                    js_unwrap!(@{self.as_ref()}.$prop)
                }
            )*
        }
    );
    (
        $trait_name:ident for $struct_name:ident;
        $(($method:ident -> $prop:ident -> $ret:ty)),* $(,)*
    ) => (
        simple_accessors! {
            $trait_name for $struct_name;
            $(($method -> $prop -> $ret),)*;
        }
    );
    (
        $trait_name:ident for $struct_name:ident;
        $(($method:ident -> $prop:ident -> $ret:ty)),* $(,)*;
        $(
            $extra_func:tt
        )*
    ) => (
        impl $trait_name for $struct_name {
            $(
                fn $method(&self) -> $ret {
                    js_unwrap!(@{self.as_ref()}.$prop)
                }
            )*
            $(
                $extra_func
            )*
        }
    )
}

macro_rules! impl_room_object_properties {
    ($struct_name:ident) => (
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
    );
    ($($struct_name:ident),* $(,)*) => (
        $(
            impl_room_object_properties!($struct_name);
        )*
    );
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
