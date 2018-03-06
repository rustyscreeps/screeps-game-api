use objects::{Tombstone, Creep};
use constants::ResourceType;

impl Tombstone {
    pub fn creep(&self) -> Creep {
        js_unwrap!(${&self.0}.creep)
    }
    pub fn death_time(&self) -> u32 {
        js_unwrap!(@{self.as_ref()}.deathTime)
    }
    pub fn ticks_to_decay(&self) -> u32 {
        js_unwrap!(@{self.as_ref()}.ticksToDecay)
    }
}

simple_accessors! {
    Tombstone;
    (creep -> creep -> Creep),
    (death_time -> deathTime -> u32),
    (ticks_to_decay -> ticksToDecay -> u32),
}
