use crate::{
    macros::*,
    objects::{Creep, Tombstone},
};

simple_accessors! {
    Tombstone;
    (creep -> creep -> Creep),
    (death_time -> deathTime -> u32),
    (ticks_to_decay -> ticksToDecay -> u32),
}
