use {
    constants::ReturnCode,
    objects::{Creep, Structure, StructureTower},
};

impl StructureTower {
    pub fn attack(&self, target: &Creep) -> ReturnCode {
        js_unwrap! { @{self.as_ref()}.attack( @{target.as_ref()} ) }
    }

    pub fn heal(&self, target: &Creep) -> ReturnCode {
        js_unwrap! { @{self.as_ref()}.heal( @{target.as_ref()} ) }
    }

    pub fn repair(&self, target: &Structure) -> ReturnCode {
        js_unwrap! { @{self.as_ref()}.repair( @{target.as_ref()} ) }
    }
}
