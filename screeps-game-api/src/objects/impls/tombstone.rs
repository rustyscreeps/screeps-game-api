use objects::{Creep, Tombstone};

simple_accessors! {
    Tombstone;
    (creep -> creep -> Creep),
    (death_time -> deathTime -> u32),
    (ticks_to_decay -> ticksToDecay -> u32),
    (id -> id -> String)
}

impl PartialEq for Tombstone {
    #[inline]
    fn eq(&self, other: &Tombstone) -> bool{
        self.id() == other.id()
    }
}

impl Eq for Tombstone {}
