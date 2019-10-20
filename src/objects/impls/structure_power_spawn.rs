use crate::{constants::ReturnCode, objects::StructurePowerSpawn};

impl StructurePowerSpawn {
    pub fn process_power(&self) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.processPower()}
    }
}
