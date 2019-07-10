use crate::{macros::*, objects::Source};
simple_accessors! {
    Source;
    (energy -> energy -> u32),
    (energy_capacity -> energyCapacity -> u32),
    (ticks_to_regeneration -> ticksToRegeneration -> u32),
}
