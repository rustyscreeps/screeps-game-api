use crate::{constants::ResourceType, objects::Deposit};

simple_accessors! {
    impl Deposit {
        pub fn last_cooldown() -> u32 = lastCooldown;
    }
}

impl Deposit {
    pub fn deposit_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.depositType))
    }
}
