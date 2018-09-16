use {
    constants::ReturnCode,
    StructurePowerSpawn,
};

simple_accessors! {
    StructurePowerSpawn;
    (power -> power -> u32),
    (power_capacity -> powerCapacity -> u32),
}

impl StructurePowerSpawn {
    /// Method still under development
    #[allow(unused_variables)]
    pub fn create_power_creep(&self, name: &str) -> ! {
        unimplemented!()
    }

    pub fn process_power(&self) -> ReturnCode {
        js_unwrap!{@{self.as_ref()}.processPower()}
    }
}
