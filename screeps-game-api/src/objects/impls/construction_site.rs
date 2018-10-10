use {
    constants::{ReturnCode, StructureType},
    objects::ConstructionSite,
    traits::TryInto,
};

simple_accessors! {
    ConstructionSite;
    (my -> my -> bool),
    (progress -> progress -> u32),
    (progress_total -> progressTotal -> u32),
    (structure_type -> structureType -> StructureType),
}

impl ConstructionSite {
    pub fn owner_name(&self) -> String {
        (js! {
            var self = @{self.as_ref()};
            if (self.owner) {
                return self.owner.username;
            } else {
                return null;
            }
        }).try_into()
        .expect("expected ConstructionSite.owner.username to be a non-null string")
    }

    pub fn remove(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.remove())
    }
}
