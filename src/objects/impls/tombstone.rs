use crate::objects::{Creep, Tombstone};

simple_accessors! {
    impl Tombstone {
        pub fn creep() -> Creep = creep;
        pub fn death_time() -> u32 = deathTime;
        pub fn ticks_to_decay() -> u32 = ticksToDecay;
    }
}
