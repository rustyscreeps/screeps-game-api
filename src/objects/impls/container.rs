use crate::{macros::*, objects::StructureContainer};

simple_accessors! {
    StructureContainer;
    (ticks_to_decay -> ticksToDecay -> u32),
}
