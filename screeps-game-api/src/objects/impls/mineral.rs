use {
    constants::{Density, ResourceType},
    Mineral,
};

simple_accessors! {
    Mineral;
    (density -> density -> u32),
    (mineral_amount -> mineralAmount -> Density),
    (mineral_type -> mineralType -> ResourceType),
    // id from HasId trait
    (ticks_to_regeneration -> ticksToRegeneration -> u32),
}
