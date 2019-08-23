use crate::{
    constants::{ReturnCode, StructureType},
    macros::*,
    objects::ConstructionSite,
    traits::TryInto,
};

simple_accessors! {
    impl ConstructionSite {
        pub fn my() -> bool = my;
        pub fn progress() -> u32 = progress;
        pub fn progress_total() -> u32 = progressTotal;
    }
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
        })
        .try_into()
        .expect("expected ConstructionSite.owner.username to be a non-null string")
    }

    pub fn remove(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.remove())
    }

    pub fn structure_type(&self) -> StructureType {
        js_unwrap!(__structure_type_str_to_num(@{self.as_ref()}.structureType))
    }
}
