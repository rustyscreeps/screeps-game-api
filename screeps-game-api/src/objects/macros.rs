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
