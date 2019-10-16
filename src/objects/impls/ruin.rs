use crate::{
    macros::*,
    objects::{Ruin, Structure},
};

simple_accessors! {
    impl Ruin {
        pub fn structure() -> Structure = structure;
        pub fn destroy_time() -> u32 = destroyTime;
    }
}
