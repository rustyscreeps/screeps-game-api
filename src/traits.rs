use enum_dispatch::enum_dispatch;

use crate::{
    enums::*,
    objects::*,
};

#[enum_dispatch]
pub trait Attackable {}

#[enum_dispatch]
pub trait HasStore {
    fn store(&self) -> Store;
}

#[enum_dispatch]
pub trait IsStructure {}
