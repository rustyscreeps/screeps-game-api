use crate::{constants::ReturnCode, macros::*, objects::StructurePowerSpawn};

simple_accessors! {
    impl StructurePowerSpawn {
        pub fn power() -> u32 = power;
        pub fn power_capacity() -> u32 = powerCapacity;
    }
}

impl StructurePowerSpawn {
    // pub fn create_power_creep(&self, name: &str) -> ! {
    //     unimplemented!()
    // }

    pub fn process_power(&self) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.processPower()}
    }
}
