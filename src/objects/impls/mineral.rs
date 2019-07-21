use crate::{
    constants::{Density, ResourceType},
    macros::*,
    objects::Mineral,
};

simple_accessors! {
    Mineral;
    (density -> density -> u32),
    (mineral_amount -> mineralAmount -> Density),
    // id from HasId trait
    (ticks_to_regeneration -> ticksToRegeneration -> u32),
}

impl Mineral {
    pub fn mineral_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.mineralType))
    }
}
