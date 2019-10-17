use crate::{
    constants::{Density, ResourceType},
    objects::Mineral,
};

simple_accessors! {
    impl Mineral {
        pub fn density() -> u32 = density;
        pub fn mineral_amount() -> Density = mineralAmount;
        // id from HasId trait
        pub fn ticks_to_regeneration() -> u32 = ticksToRegeneration;
    }
}

impl Mineral {
    pub fn mineral_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.mineralType))
    }
}
