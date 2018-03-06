use objects::Source;

simple_accessors! {
    Source;
    (energy -> energy -> u32),
    (energy_capacity -> energyCapacity -> u32),
    (id -> id -> String),
    (ticks_to_regeneration -> ticksToRegeneration -> u32),
}
