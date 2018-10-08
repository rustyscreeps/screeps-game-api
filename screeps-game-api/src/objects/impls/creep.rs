use std::{marker::PhantomData, mem};

use stdweb::{Value, Reference};

use {
    constants::{Direction, Part, ResourceType, ReturnCode},
    memory::MemoryReference,
    objects::{
        Attackable, ConstructionSite, Creep, FindOptions, HasPosition,
        Resource, Source, StructureController, StructureProperties, Transferable, Withdrawable,
    },

    pathfinder::{CostMatrix, SearchResults},
    traits::TryFrom,

};

use super::room::Step;

scoped_thread_local!(static COST_CALLBACK: Box<Fn(String, Reference) -> Option<Reference>>);

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

    pub fn carry_total(&self) -> u32 {
        js_unwrap!(_.sum(@{self.as_ref()}.carry))
    }

    pub fn carry_types(&self) -> Vec<ResourceType> {
        js_unwrap!(Object.keys(@{self.as_ref()}.carry).map(__resource_type_str_to_num))
    }

    pub fn carry_of(&self, ty: ResourceType) -> u32 {
        js_unwrap!(@{self.as_ref()}.carry[__resource_type_num_to_str(@{ty as u32})] || 0)
    }

    pub fn drop(&self, ty: ResourceType, amount: Option<u32>) -> ReturnCode {
        match amount {
            Some(v) => {
                js_unwrap!(@{self.as_ref()}.drop(__resource_type_num_to_str(@{ty as u32}), @{v}))
            }
            None => js_unwrap!(@{self.as_ref()}.drop(__resource_type_num_to_str(@{ty as u32}))),
        }
    }

    pub fn energy(&self) -> u32 {
        js_unwrap!(@{self.as_ref()}.carry[RESOURCE_ENERGY])
    }

    pub fn owner_name(&self) -> String {
        js_unwrap!(@{self.as_ref()}.owner.username)
    }

    pub fn cancel_order(&self, name: &str) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.cancelOrder(@{name}))
    }

    pub fn move_direction(&self, dir: Direction) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.move(@{dir as u32}))
    }

    pub fn move_to_xy(&self, x: u32, y: u32) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.moveTo(@{x}, @{y}))
    }

    pub fn move_to_xy_options<'a, F>(
        &self,
        x: u32,
        y: u32,
        reuse_path: Option<u32>,
        serialize_memory: Option<bool>,
        no_path_finding: Option<bool>,
        find_options: Option<FindOptions<'a, F>>,
    ) -> ReturnCode
    where
        F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>> + 'a,
    {
        let rp = reuse_path.unwrap_or(5u32);
        let sm = serialize_memory.unwrap_or(true);
        let pf = no_path_finding.unwrap_or(false);
        let fo: FindOptions<'a, F> = find_options.unwrap_or(FindOptions::default());

        let FindOptions {
            ignore_creeps,
            ignore_destructible_structures,
            cost_callback,
            max_ops,
            heuristic_weight,
            serialize,
            max_rooms,
            range,
            plain_cost,
            swamp_cost,
        } = fo;

        // This callback is the one actually passed to JavaScript.
        fn callback(room_name: String, cost_matrix: Reference) -> Option<Reference> {
            COST_CALLBACK.with(|callback| callback(room_name, cost_matrix))
        }

        // User provided callback: rust String, CostMatrix -> Option<CostMatrix>
        let raw_callback = cost_callback;

        // Wrapped user callback: rust String, Reference -> Option<Reference>
        let callback_boxed = move |room_name, cost_matrix_ref| {
            let cmatrix = CostMatrix {
                inner: cost_matrix_ref,
                lifetime: PhantomData,
            };
            raw_callback(room_name, cmatrix).map(|cm| cm.inner)
        };

        // Type erased and boxed callback: no longer a type specific to the closure passed in,
        // now unified as Box<Fn>
        let callback_type_erased: Box<Fn(String, Reference) -> Option<Reference> + 'a> =
            Box::new(callback_boxed);

        // Overwrite lifetime of box inside closure so it can be stuck in scoped_thread_local storage:
        // now pretending to be static data so that it can be stuck in scoped_thread_local. This should
        // be entirely safe because we're only sticking it in scoped storage and we control the only use
        // of it, but it's still necessary because "some lifetime above the current scope but otherwise
        // unknown" is not a valid lifetime to have PF_CALLBACK have.
        let callback_lifetime_erased: Box<
            Fn(String, Reference) -> Option<Reference> + 'static,
        > = unsafe { mem::transmute(callback_type_erased) };

        // Store callback_lifetime_erased in COST_CALLBACK for the duration of the PathFinder call and
        // make the call to PathFinder.
        //
        // See https://docs.rs/scoped-tls/0.1/scoped_tls/
        COST_CALLBACK.set(&callback_lifetime_erased, || {
            js_unwrap!{
                @{ self.as_ref() }.moveTo(
                    @{x},
                    @{y},
                    {
                        reusePath: @{rp},
                        serializeMemory: @{sm},
                        noPathFinding: @{pf},
                        visualizePathStyle: undefined,  // todo
                        ignoreCreeps: @{ignore_creeps},
                        ignoreDestructibleStructures: @{ignore_destructible_structures}
                        costCallback: @{callback},
                        maxOps: @{max_ops},
                        heuristicWeight: @{heuristic_weight},
                        serialize: @{serialize},
                        maxRooms: @{max_rooms},
                        range: @{range},
                        plainCost: @{plain_cost},
                        swampCost: @{swamp_cost},
                    }
                )
            }
        })
    }

    pub fn move_to<T: HasPosition>(&self, target: &T) -> ReturnCode {
        let p = target.pos();
        js_unwrap!(@{self.as_ref()}.moveTo(@{&p.0}))
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

    pub fn get_active_bodyparts(&self, ty: Part) -> u32 {
        js_unwrap!(@{self.as_ref()}.getActiveBodyparts(__part_str_to_num(@{ty as u32})))
    }

    pub fn ranged_mass_attack(&self) -> ReturnCode {
        js_unwrap!(@{self.as_ref()}.rangedMassAttack())
    }

    pub fn transfer_amount<T>(&self, target: &T, ty: ResourceType, amount: u32) -> ReturnCode
    where
        T: Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32}),
            @{amount}
        ))
    }

    pub fn transfer_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: Transferable,
    {
        js_unwrap!(@{self.as_ref()}.transfer(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32})
        ))
    }

    pub fn withdraw_amount<T>(&self, target: &T, ty: ResourceType, amount: u32) -> ReturnCode
    where
        T: Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32}),
            @{amount}
        ))
    }

    pub fn withdraw_all<T>(&self, target: &T, ty: ResourceType) -> ReturnCode
    where
        T: Withdrawable,
    {
        js_unwrap!(@{self.as_ref()}.withdraw(
            @{target.as_ref()},
            __resource_type_num_to_str(@{ty as u32})
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
    (carry_capacity -> carryCapacity -> u32),
    (fatigue -> fatigue -> u32),
    (name -> name -> String),
    (my -> my -> bool),
    (saying -> saying -> String),
    (spawning -> spawning -> bool),
    (ticks_to_live -> ticksToLive -> u32),
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
