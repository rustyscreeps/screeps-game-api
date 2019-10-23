use crate::{
    constants::{PowerCreepClass, PowerType, ReturnCode},
    objects::{
        PowerCreep, RoomObjectProperties, StructureController, StructurePowerSpawn,
        StructureProperties,
    },
};

impl PowerCreep {
    pub fn create(name: String, class: PowerCreepClass) -> ReturnCode {
        js_unwrap!(PowerCreep.create(@{name}, __power_creep_class_num_to_str(@{class as u32})))
    }

    pub fn class(&self) -> PowerCreepClass {
        js_unwrap!(__power_creep_class_str_to_num(@{self.as_ref()}.className))
    }

    pub fn delete(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.delete())
    }

    pub fn power_cooldown(&self, power_type: PowerType) -> Option<u32> {
        js_unwrap!((@{self.as_ref()}.powers[@{power_type as u32}] || {}).cooldown)
    }

    pub fn power_keys(&self) -> Vec<PowerType> {
        js_unwrap!(Object.keys(@{self.as_ref()}.powers).map(Number))
    }

    pub fn power_level(&self, power_type: PowerType) -> Option<u8> {
        js_unwrap!((@{self.as_ref()}.powers[@{power_type as u32}] || {}).level)
    }

    pub fn use_power<T>(&self, power_type: PowerType, target: Option<&T>) -> ReturnCode
    where
        T: ?Sized + RoomObjectProperties,
    {
        match target {
            Some(v) => js_unwrap!(@{self.as_ref()}.usePower(@{power_type as u32}, @{v.as_ref()})),
            None => js_unwrap!(@{self.as_ref()}.usePower(@{power_type as u32})),
        }
    }

    pub fn rename(&self, new_name: String) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.rename(@{new_name}))
    }

    pub fn upgrade(&self, power_type: PowerType) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.upgrade(@{power_type as u32}))
    }
}

simple_accessors! {
    impl PowerCreep {
        pub fn delete_time() -> Option<u64> = deleteTime;
        pub fn level() -> u32 = level;
        pub fn shard() -> Option<String> = shard;
        pub fn spawn_cooldown_time() -> Option<u64> = spawnCooldownTime;
    }
}

creep_simple_generic_action! {
    impl PowerCreep {
        pub fn renew(StructureProperties) = renew();
    }
}

creep_simple_concrete_action! {
    impl PowerCreep {
        pub fn enable_room(StructureController) = enableRoom();
        pub fn spawn(StructurePowerSpawn) = spawn();
    }
}
