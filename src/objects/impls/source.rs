use crate::objects::Source;

simple_accessors! {
    impl Source {
        pub fn energy() -> u32 = energy;
        pub fn energy_capacity() -> u32 = energyCapacity;
        pub fn ticks_to_regeneration() -> Option<u32> = ticksToRegeneration;
    }
}
