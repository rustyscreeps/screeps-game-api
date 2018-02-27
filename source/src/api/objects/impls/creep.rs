use stdweb::Reference;
use stdweb::unstable::TryInto;

use api::{Direction, Part, ReturnCode};
use api::objects::{Attackable, ConstructionSite, Creep, HasPosition, Resource,
                   RoomObjectProperties, Source, StructureController, StructureProperties,
                   Transferable, Withdrawable};
use api::ResourceType;

impl Creep {
    pub fn carry_total(&self) -> i32 {
        js_unwrap!(_.sum(@{&self.0}.carry))
    }

    pub fn carry_types(&self) -> Vec<ResourceType> {
        js_unwrap!(Object.keys(@{&self.0}).map(__resource_type_str_to_num))
    }

    pub fn carry_of(&self, ty: ResourceType) -> i32 {
        js_unwrap!(@{&self.0}.carry[__resource_type_num_to_str(@{ty as i32})] || 0)
    }

    pub fn energy(&self) -> i32 {
        js_unwrap!(@{&self.0}.carry[RESOURCE_ENERGY])
    }

    pub fn cancel_order(&self, name: &str) -> ReturnCode {
        js_unwrap!(@{&self.0}.cancelOrder(@{name}))
    }

    pub fn move_direction(&self, dir: Direction) -> ReturnCode {
        js_unwrap!(@{&self.0}.move(@{dir as i32}))
    }

    pub fn move_to_xy(&self, x: i32, y: i32) -> ReturnCode {
        js_unwrap!(@{&self.0}.moveTo(@{x}, @{y}))
    }

    pub fn say(&self, msg: &str, public: bool) -> ReturnCode {
        js_unwrap!(@{&self.0}.say(@{msg}, @{public}))
    }

    pub fn sign_controller<T>(&self, target: &T, text: &str) -> ReturnCode
    where
        T: AsRef<Reference>,
    {
        js_unwrap!(@{&self.0}.signController(@{target.as_ref()}, @{text}))
    }

    pub fn suicide(&self) -> ReturnCode {
        js_unwrap!(@{&self.0}.suicide())
    }

    pub fn parts(&self) -> Vec<Part> {
        js_unwrap!(@{&self.0}.body.map(|p| __part_str_to_num(p)))
    }

    pub fn get_active_bodyparts(&self, ty: Part) -> i32 {
        js_unwrap!(@{&self.0}.getActiveBodyparts(__part_str_to_num(@{ty as i32})))
    }

    pub fn has_active_bodyparts(&self, ty: Part) -> i32 {
        js_unwrap!(_hasActiveBodyparts(@{&self.0}, __part_str_to_num(@{ty as i32})))
    }

    pub fn move_to<T: HasPosition>(&self, target: &T) -> ReturnCode {
        let p = target.pos();
        js_unwrap!(@{&self.0}.moveTo(@{&p.0}))
    }

    pub fn ranged_mass_attack(&self) -> ReturnCode {
        js_unwrap!(@{&self.0}.rangedMassAttack())
    }

    pub fn transfer_amount<T>(&self, target: &T, ty: ResourceType, amount: i32) -> ReturnCode
    where
        T: Transferable,
    {
        js_unwrap!(@{&self.0}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32}),
            @{amount}
        ))
    }

    pub fn transfer_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: Transferable,
    {
        js_unwrap!(@{&self.0}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32})
        ))
    }

    pub fn withdraw_amount<T>(&self, target: &T, ty: ResourceType, amount: i32) -> ReturnCode
    where
        T: Withdrawable,
    {
        js_unwrap!(@{&self.0}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32}),
            @{amount}
        ))
    }

    pub fn withdraw_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: Withdrawable,
    {
        js_unwrap!(@{&self.0}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as i32})
        ))
    }
}

macro_rules! creep_simple_generic_action {
    ($(($method:ident($trait:ident) -> $js_name:ident)),* $(,)*) => (
        impl Creep {
            $(
                pub fn $method<T>(&self, target: &T) -> ReturnCode
                where
                    T: $trait,
                {
                    js_unwrap!(@{&self.0}.$js_name(@{target.as_ref()}))
                }
            )*
        }
    )
}
macro_rules! creep_simple_concrete_action {
    ($(($method:ident($type:ty) -> $js_name:ident)),* $(,)*) => (
        impl Creep {
            $(
                pub fn $method(&self, target: &$type) -> ReturnCode
                {
                    js_unwrap!(@{&self.0}.$js_name(@{target.as_ref()}))
                }
            )*
        }
    )
}

simple_accessors! {
    Creep;
    (carry_capacity -> carryCapacity -> i32),
    (fatigue -> fatigue -> i32),
    (hits -> hits -> i32),
    (hits_max -> hitsMax -> i32),
    (name -> name -> String),
    (my -> my -> bool),
    (spawning -> spawning -> bool),
    (ticks_to_live -> ticksToLive -> i32),
}

creep_simple_generic_action! {
    // TODO: 'attackable' trait for structures + creeps?
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
