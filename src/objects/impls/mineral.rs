use crate::{
    constants::{Density, ResourceType},
    objects::Mineral,
};

simple_accessors! {
    impl Mineral {
        pub fn density() -> Density = density;
        // id from HasId trait
    }
}

impl Mineral {
    pub fn mineral_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.mineralType))
    }

    pub fn mineral_amount(&self) -> u32 {
        // workaround for the fact that some private servers return floating point
        // mineralAmount values
        js_unwrap!(Math.floor(@{self.as_ref()}.mineralAmount))
    }

    pub fn ticks_to_regeneration(&self) -> u32 {
        js_unwrap!(Math.max(0, @{self.as_ref()}.ticksToRegeneration || 0))
    }
}
