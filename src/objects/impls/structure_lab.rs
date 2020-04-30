use stdweb::Value;

use crate::{
    constants::{ResourceType, ReturnCode},
    objects::{Creep, StructureLab},
    traits::TryFrom,
};

impl StructureLab {
    pub fn mineral_type(&self) -> Option<ResourceType> {
        let mineral_v = js! {
            const mineral = @{self.as_ref()}.mineralType;
            if (mineral) {
                return __resource_type_str_to_num(mineral);
            }
        };
        match mineral_v {
            Value::Number(_) => {
                Some(ResourceType::try_from(mineral_v).expect("lab resource unknown."))
            }
            _ => None,
        }
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
