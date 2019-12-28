use stdweb::Value;

use crate::{
    constants::{Part, ResourceType, ReturnCode},
    objects::{
        Attackable, ConstructionSite, Creep, Harvestable, SharedCreepProperties,
        StructureController, StructureProperties, Transferable, Withdrawable,
    },
    traits::TryFrom,
};

impl Creep {
    pub fn body(&self) -> Vec<Bodypart> {
        // Has to be deconstructed manually to avoid converting strings from js to rust
        let len: u32 = js_unwrap!(@{self.as_ref()}.body.length);
        let mut body_parts: Vec<Bodypart> = Vec::with_capacity(len as usize);

        for i in 0..len {
            let boost_v = js!(const b=@{self.as_ref()}.body[@{i}].boost||null;return b&&__resource_type_str_to_num(b););
            let boost = match boost_v {
                Value::Number(_) => {
                    Some(ResourceType::try_from(boost_v).expect("Creep boost resource unknown."))
                }
                _ => None,
            };
            let part: Part = js_unwrap!(__part_str_to_num(@{self.as_ref()}.body[@{i}].type));
            let hits: u32 = js_unwrap!(@{self.as_ref()}.body[@{i}].hits);

            body_parts.push(Bodypart {
                boost,
                part,
                hits,
                _non_exhaustive: (),
            });
        }
        body_parts
    }

    pub fn sign_controller(&self, target: &StructureController, text: &str) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.signController(@{target.as_ref()}, @{text}))
    }

    pub fn get_active_bodyparts(&self, ty: Part) -> u32 {
        js_unwrap!(@{self.as_ref()}.getActiveBodyparts(__part_num_to_str(@{ty as u32})))
    }

    pub fn ranged_mass_attack(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.rangedMassAttack())
    }

    pub fn transfer_amount<T>(&self, target: &T, ty: ResourceType, amount: u32) -> ReturnCode
    where
        T: ?Sized + Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32}),
            @{amount}
        ))
    }

    pub fn transfer_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: ?Sized + Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32})
        ))
    }

    pub fn withdraw_amount<T>(&self, target: &T, ty: ResourceType, amount: u32) -> ReturnCode
    where
        T: ?Sized + Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32}),
            @{amount}
        ))
    }

    pub fn withdraw_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: ?Sized + Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32})
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Bodypart {
    pub boost: Option<ResourceType>,
    pub part: Part,
    pub hits: u32,
    _non_exhaustive: (),
}

simple_accessors! {
    impl Creep {
        pub fn fatigue() -> u32 = fatigue;
        pub fn spawning() -> bool = spawning;
    }
}

creep_simple_generic_action! {
    impl Creep {
        pub fn attack(Attackable) = attack();
        pub fn dismantle(StructureProperties) = dismantle();
        pub fn harvest(Harvestable) = harvest();
        pub fn heal(SharedCreepProperties) = heal();
        pub fn ranged_attack(Attackable) = rangedAttack();
        pub fn ranged_heal(SharedCreepProperties) = rangedHeal();
        pub fn repair(StructureProperties) = repair();
    }
}

creep_simple_concrete_action! {
    impl Creep {
        pub fn attack_controller(StructureController) = attackController();
        pub fn build(ConstructionSite) = build();
        pub fn claim_controller(StructureController) = claimController();
        pub fn generate_safe_mode(StructureController) = generateSafeMode();
        pub fn move_pulled_by(Creep) = move();
        pub fn pull(Creep) = pull();
        pub fn reserve_controller(StructureController) = reserveController();
        pub fn upgrade_controller(StructureController) = upgradeController();
    }
}
