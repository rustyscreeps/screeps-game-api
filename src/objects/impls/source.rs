use crate::objects::Source;

simple_accessors! {
    impl Source {
        pub fn energy() -> u32 = energy;
        pub fn energy_capacity() -> u32 = energyCapacity;
    }
}

impl Source {
    pub fn ticks_to_regeneration(&self) -> u32 {
        js_unwrap!(Math.max(0, @{self.as_ref()}.ticksToRegeneration || 0))
    }
}
