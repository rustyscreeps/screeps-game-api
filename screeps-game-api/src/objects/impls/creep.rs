use stdweb::Value;

use {
    constants::{Direction, Part, ResourceType, ReturnCode},
    memory::MemoryReference,
    objects::{
        Attackable, ConstructionSite, Creep, HasPosition, Resource, Source, StructureController,
        StructureProperties, Transferable, Withdrawable,
    },
    pathfinder::SearchResults,
    traits::TryFrom,
};

use super::room::Step;

impl Creep {
    pub fn body(&self) -> Vec<Bodypart> {
        // Has to be deconstructed manually to avoid converting strings from js to rust
        let len: u32 = js_unwrap!(@{self.as_ref()}.body.length);
        let mut body_parts: Vec<Bodypart> = Vec::with_capacity(len as usize);

        for i in 0..len {
            let boost_v =
                js!(return __resource_type_str_to_num(@{self.as_ref()}.body[@{i}].boost););
            let boost = match boost_v {
                Value::Number(_) => {
                    Some(ResourceType::try_from(boost_v).expect("Creep boost resource unknown."))
                }
                _ => None,
            };
            let part: Part = js_unwrap!(__part_str_to_num(@{self.as_ref()}.body[@{i}].type));
            let hits: u32 = js_unwrap!(@{self.as_ref()}.body[@{i}].hits);

            body_parts.push(Bodypart { boost, part, hits });
        }
        body_parts
    }

    pub fn carry_total(&self) -> i32 {
        js_unwrap!(_.sum(@{self.as_ref()}.carry))
    }

    pub fn carry_types(&self) -> Vec<ResourceType> {
        js_unwrap!(Object.keys(@{self.as_ref()}.carry).map(__resource_type_str_to_num))
    }

    pub fn carry_of(&self, ty: ResourceType) -> i32 {
        js_unwrap!(@{self.as_ref()}.carry[__resource_type_num_to_str(@{ty as i32})] || 0)
    }

    pub fn drop(&self, ty: ResourceType, amount: Option<u32>) -> ReturnCode {
        match amount {
            Some(v) => {
                js_unwrap!(@{self.as_ref()}.drop(__resource_type_num_to_str(@{ty as i32}), @{v}))
            }
            None => js_unwrap!(@{self.as_ref()}.drop(__resource_type_num_to_str(@{ty as i32}))),
        }
    }

    pub fn energy(&self) -> i32 {
        js_unwrap!(@{self.as_ref()}.carry[RESOURCE_ENERGY])
    }

    pub fn owner_name(&self) -> String {
        js_unwrap!(@{self.as_ref()}.owner.username)
    }

    pub fn cancel_order(&self, name: &str) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.cancelOrder(@{name}))
    }

    pub fn move_direction(&self, dir: Direction) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.move(@{dir as i32}))
    }

    pub fn move_to_xy(&self, x: i32, y: i32) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveTo(@{x}, @{y}))
    }

    pub fn move_by_path_serialized(&self, path: &str) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveByPath(@{path}))
    }

    pub fn move_by_path_steps(&self, path: &[Step]) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveByPath(@{path}))
    }

    pub fn move_by_path_search_result(&self, path: &SearchResults) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveByPath(@{path.opaque_path()}))
    }

    pub fn memory(&self) -> MemoryReference {
        js_unwrap!(@{self.as_ref()}.memory)
    }

    pub fn notify_when_attacked(&self, notify_when_attacked: bool) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.notifyWhenAttacked(@{notify_when_attacked}))
    }

    pub fn say(&self, msg: &str, public: bool) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.say(@{msg}, @{public}))
    }

    pub fn sign_controller(&self, target: &StructureController, text: &str) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.signController(@{target.as_ref()}, @{text}))
    }

    pub fn suicide(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.suicide())
    }

    pub fn get_active_bodyparts(&self, ty: Part) -> i32 {
        js_unwrap!(@{self.as_ref()}.getActiveBodyparts(__part_str_to_num(@{ty as i32})))
    }

    pub fn move_to<T: HasPosition>(&self, target: &T) -> ReturnCode {
        let p = target.pos();
        js_unwrap!(@{self.as_ref()}.moveTo(@{&p.0}))
    }

    pub fn ranged_mass_attack(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.rangedMassAttack())
    }

    pub fn transfer_amount<T>(&self, target: &T, ty: ResourceType, amount: i32) -> ReturnCode
    where
        T: Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32}),
            @{amount}
        ))
    }

    pub fn transfer_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32})
        ))
    }

    pub fn withdraw_amount<T>(&self, target: &T, ty: ResourceType, amount: i32) -> ReturnCode
    where
        T: Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32}),
            @{amount}
        ))
    }

    pub fn withdraw_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32})
        ))
    }
}

#[derive(Clone, Debug)]
pub struct Bodypart {
    boost: Option<ResourceType>,
    part: Part,
    hits: u32,
}

simple_accessors! {
    Creep;
    (carry_capacity -> carryCapacity -> i32),
    (fatigue -> fatigue -> i32),
    (name -> name -> String),
    (my -> my -> bool),
    (saying -> saying -> String),
    (spawning -> spawning -> bool),
    (ticks_to_live -> ticksToLive -> i32),
}

creep_simple_generic_action! {
    (attack(Attackable) -> attack),
    (dismantle(StructureProperties) -> dismantle),
    (ranged_attack(Attackable) -> rangedAttack),
    (repair(StructureProperties) -> repair),
}

creep_simple_concrete_action! {
    (attack_controller(StructureController) -> attackController),
    (build(ConstructionSite) -> build),
    (claim_controller(StructureController) -> claimController),
    (generate_safe_mode(StructureController) -> generateSafeMode),
    (harvest(Source) -> harvest),
    (heal(Creep) -> heal),
    (pickup(Resource) -> pickup),
    (ranged_heal(Creep) -> rangedHeal),
    (reserve_controller(StructureController) -> reserveController),
    (upgrade_controller(StructureController) -> upgradeController),
}
