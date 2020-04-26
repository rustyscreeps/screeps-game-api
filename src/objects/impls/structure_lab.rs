use crate::{
    constants::{ResourceType, ReturnCode},
    objects::{Creep, StructureLab},
};

impl StructureLab {
    pub fn mineral_type(&self) -> ResourceType {
        js_unwrap!(__resource_type_str_to_num(@{self.as_ref()}.mineralType))
    }

    pub fn boost_creep(&self, creep: &Creep, body_part_count: Option<u32>) -> ReturnCode {
        match body_part_count {
            None => js_unwrap! {@{self.as_ref()}.boostCreep(@{creep.as_ref()})},
            Some(count) => js_unwrap! {@{self.as_ref()}.boostCreep(@{creep.as_ref()}, @{count})},
        }
    }

    pub fn run_reaction(&self, lab1: &StructureLab, lab2: &StructureLab) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.runReaction(@{lab1.as_ref()}, @{lab2.as_ref()})}
    }

    pub fn reverse_reaction(&self, lab1: &StructureLab, lab2: &StructureLab) -> ReturnCode {
        js_unwrap! {@{self.as_ref()}.reverseReaction(@{lab1.as_ref()}, @{lab2.as_ref()})}
    }

    pub fn unboost_creep(&self, creep: &Creep) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.unboostCreep(@{creep.as_ref()}))
    }
}
