use crate::{macros::*, objects::StructureKeeperLair};

simple_accessors! {
    impl StructureKeeperLair {
        pub fn ticks_to_spawn() -> u32 = ticksToSpawn;
    }
}
