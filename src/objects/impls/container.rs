use crate::{macros::*, objects::StructureContainer};

simple_accessors! {
    impl StructureContainer {
        pub fn ticks_to_decay() -> u32 = ticksToDecay;
    }
}
