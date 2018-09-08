/* Macro used to encapsulate all screeps game objects

Macro syntax: 
reference_wrapper!{
    $obj1,
    $obj2,
    ...
}

Screeps game objects, in javascript, can be accessed via stdweb's `Reference`
object. This macro: 
  - Creates a struct named `objX`;
  - Implements traits `AsRef<Reference>`, `TryFrom<Value>` for `objX`
  - Implements trait `From<objX>` for `Reference`

*/
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

/* Automatically creates simple accessors to fields of screep objects

On top of an object created from `reference_wrapper!`, this macro creates an
implementation of the struct for a collection of fields from the screeps
object. 

Method Syntax: 
simple_accessor! {
    $struct_name;
    ($rust_method_name1 -> $js_field_name1 -> $rust_type1),
    ($rust_method_name2 -> $js_field_name2 -> $rust_type2),
    ...
}
*/
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

/* Implements the unsafe trait RoomObjectProperties for a Structure struct 

Macro Syntax: 
impl_room_object_properties!{
    $structure1,
    $structure2,
    ...
}
*/
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
