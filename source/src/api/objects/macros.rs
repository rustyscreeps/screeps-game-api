macro_rules! reference_wrappers {
    ($name:ident) => {
        #[derive(Clone)]
        pub struct $name(Reference);

        impl AsRef<Reference> for $name {
            fn as_ref(&self) -> &Reference {
                &self.0
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
                    js_unwrap!(@{&self.0}.$prop)
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
                    js_unwrap!(@{&self.0}.$prop)
                }
            )*
            $(
                $extra_func
            )*
        }
    )
}

macro_rules! impl_room_object {
    ($name:ident) => (
        simple_accessors! {
            RoomObjectProperties for $name;
            (pos -> pos -> RoomPosition),
            (room -> room -> Room),
        }
    );

    ($($name:ident),* $(,)*) => (
        $(
            impl_room_object!($name);
        )*
    )
}

macro_rules! impl_structure {
    ($name:ident) => (
        simple_accessors! {
            StructureProperties for $name;
            (hits -> hits -> i32),
            (hits_max -> hitsMax -> i32),
            (id -> id -> String),
            (is_active -> isActive -> bool);

            fn destroy(&self) -> ReturnCode {
                js_unwrap!(@{&self.0}.destroy())
            }
        }
    );

    ($($name:ident),* $(,)*) => (
        $(
            impl_structure!($name);
        )*
    )
}

macro_rules! impl_owned_structure {
    ($name:ident) => (
        simple_accessors! {
            OwnedStructureProperties for $name;
            (my -> my -> bool);
            fn owner(&self) -> Option<String> {
                (js! {
                    var self = @{&self.0};
                    if (self.owner) {
                        return self.owner.username;
                    } else {
                        return null;
                    }
                }).try_into().unwrap()
            }
        }
    );

    ($($name:ident),* $(,)*) => (
        $(
            impl_owned_structure!($name);
        )*
    )
}
