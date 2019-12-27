use crate::{
    constants::ReturnCode,
    objects::{Attackable, SharedCreepProperties, StructureProperties, StructureTower},
};

impl StructureTower {
    pub fn attack<T>(&self, target: &T) -> ReturnCode
    where
        T: Attackable,
    {
        js_unwrap! { @{self.as_ref()}.attack( @{target.as_ref()} ) }
    }

    pub fn heal<T>(&self, target: &T) -> ReturnCode
    where
        T: SharedCreepProperties,
    {
        js_unwrap! { @{self.as_ref()}.heal( @{target.as_ref()} ) }
    }

    pub fn repair<T>(&self, target: &T) -> ReturnCode
    where
        T: StructureProperties,
    {
        js_unwrap! { @{self.as_ref()}.repair( @{target.as_ref()} ) }
    }
}
