use crate::objects::{Creep, Tombstone};

simple_accessors! {
    impl Tombstone {
        pub fn creep() -> Creep = creep;
        pub fn death_time() -> u32 = deathTime;
    }
}
