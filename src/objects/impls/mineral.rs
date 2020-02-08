use crate::{
    constants::{Density, ResourceType},
    objects::Mineral,
};

simple_accessors! {
    impl Mineral {
        pub fn density() -> Density = density;
        // id from HasId trait
        pub fn ticks_to_regeneration() -> u32 = ticksToRegeneration;
    }
}

impl Mineral {
    pub fn mineral_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.mineralType))
    }

    pub fn mineral_amount(&self) -> u32 {
        // workaround for the fact that some private servers return floating point mineralAmount values
        js_unwrap!(Math.floor(@{self.as_ref()}.mineralAmount))
    }
}
