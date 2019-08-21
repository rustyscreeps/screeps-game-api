use crate::{macros::*, objects::StructurePowerBank};

simple_accessors! {
    impl StructurePowerBank {
        pub fn power() -> u32 = power;
    }
}
