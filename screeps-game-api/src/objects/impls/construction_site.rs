use stdweb::unstable::TryInto;

use {ReturnCode, StructureType};

// TODO: Use root import after https://github.com/rust-lang/rust/issues/53140 is fixed.
use super::super::ConstructionSite;

simple_accessors! {
    ConstructionSite;
    (id -> id -> String),
    (my -> my -> bool),
    (progress -> progress -> i32),
    (progress_total -> progressTotal -> i32),
    (structure_type -> structureType -> StructureType),
}

impl ConstructionSite {
    pub fn owner(&self) -> String {
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
