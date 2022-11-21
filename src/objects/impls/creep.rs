use crate::{
    constants::{Direction, Part, ResourceType, ReturnCode},
    js_collections::JsCollectionFromValue,
    objects::{
        ConstructionSite, Owner, Resource, RoomObject, Store, Structure, StructureController,
    },
    prelude::*,
    CostMatrix, MoveToOptions, RoomName, RoomPosition, SingleRoomCostResult,
};
use js_sys::{Array, JsString};
use wasm_bindgen::{prelude::*, JsCast};

#[wasm_bindgen]
extern "C" {
    /// A [`Creep`] unit in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Creep;

    #[wasm_bindgen(method, getter = body)]
    fn body_internal(this: &Creep) -> Array;

    /// The amount of fatigue the creep has. If greater than 0, it cannot move
    /// this tick without being pulled.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.fatigue)
    #[wasm_bindgen(method, getter)]
    pub fn fatigue(this: &Creep) -> u32;

    /// Retrieve the current hits of this creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.hits)
    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &Creep) -> u32;

    /// Retrieve the maximum hits of this creep, which generally equals 50 per
    /// body part.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.hitsMax)
    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hits_max(this: &Creep) -> u32;

    /// Object ID of the creep, which can be used to efficiently fetch a fresh
    /// reference to the object on subsequent ticks.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &Creep) -> Option<JsString>;

    /// A shortcut to `Memory.creeps[creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.memory)
    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &Creep) -> JsValue;

    /// Sets a new value to `Memory.creeps[creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.memory)
    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &Creep, val: &JsValue);

    /// Whether this creep is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &Creep) -> bool;

    #[wasm_bindgen(method, getter = name)]
    fn name_internal(this: &Creep) -> JsString;

    /// The [`Owner`] of this creep that contains the owner's username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &Creep) -> Owner;

    /// What the creep said last tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.saying)
    #[wasm_bindgen(method, getter)]
    pub fn saying(this: &Creep) -> Option<JsString>;

    /// Whether the creep is still spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.spawning)
    #[wasm_bindgen(method, getter)]
    pub fn spawning(this: &Creep) -> bool;

    /// The [`Store`] of the creep, which contains information about what
    /// resources it is it carrying.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.store)
    #[wasm_bindgen(final, method, getter)]
    pub fn store(this: &Creep) -> Store;

    /// The number of ticks the creep has left to live
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.ticksToLive)
    #[wasm_bindgen(method, getter = ticksToLive)]
    pub fn ticks_to_live(this: &Creep) -> Option<u32>;

    #[wasm_bindgen(final, method, js_name = attack)]
    fn attack_internal(this: &Creep, target: &RoomObject) -> ReturnCode;

    /// Attack a [`StructureController`] in melee range using a creep's claim
    /// parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.attackController)
    #[wasm_bindgen(final, method, js_name = attackController)]
    pub fn attack_controller(this: &Creep, target: &StructureController) -> ReturnCode;

    /// Use a creep's work parts to consume carried energy, putting it toward
    /// progress in a [`ConstructionSite`] in range 3.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.build)
    #[wasm_bindgen(final, method)]
    pub fn build(this: &Creep, target: &ConstructionSite) -> ReturnCode;

    /// Cancel an a successfully called creep function from earlier in the tick,
    /// with a [`JsString`] that must contain the JS version of the function
    /// name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.cancelOrder)
    #[wasm_bindgen(final, method, js_name = cancelOrder)]
    pub fn cancel_order(this: &Creep, target: &JsString) -> ReturnCode;

    /// Claim an unowned [`StructureController`] in melee range as your own
    /// using a creep's claim parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.claimController)
    #[wasm_bindgen(final, method, js_name = claimController)]
    pub fn claim_controller(this: &Creep, target: &StructureController) -> ReturnCode;

    #[wasm_bindgen(final, method, js_name = dismantle)]
    fn dismantle_internal(this: &Creep, target: &Structure) -> ReturnCode;

    /// Drop a resource on the ground from the creep's [`Store`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.drop)
    #[wasm_bindgen(final, method)]
    pub fn drop(this: &Creep, ty: ResourceType, amount: Option<u32>) -> ReturnCode;

    // todo constant link SAFE_MODE_COST
    /// Consume [`ResourceType::Ghodium`] from the creep's [`Store`] to add a
    /// safe mode activation to a [`StructureController`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.generateSafeMode)
    #[wasm_bindgen(final, method, js_name = generateSafeMode)]
    pub fn generate_safe_mode(this: &Creep, target: &StructureController) -> ReturnCode;

    /// Get the number of parts of the given type the creep has in its body,
    /// excluding fully damaged parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.getActiveBodyparts)
    #[wasm_bindgen(final, method, js_name = getActiveBodyparts)]
    pub fn get_active_bodyparts(this: &Creep, ty: Part) -> u8;

    #[wasm_bindgen(final, method, js_name = harvest)]
    fn harvest_internal(this: &Creep, target: &RoomObject) -> ReturnCode;

    #[wasm_bindgen(final, method, js_name = heal)]
    fn heal_internal(this: &Creep, target: &RoomObject) -> ReturnCode;

    /// Move one square in the specified direction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.move)
    #[wasm_bindgen(final, method, js_name = move)]
    pub fn move_direction(this: &Creep, direction: Direction) -> ReturnCode;

    /// Accept an attempt by another creep to pull this one.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.move)
    #[wasm_bindgen(final, method, js_name = move)]
    pub fn move_pulled_by(this: &Creep, target: &Creep) -> ReturnCode;

    /// Move the creep along a previously determined path returned from a
    /// pathfinding function, in array or serialized string form.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.moveByPath)
    #[wasm_bindgen(final, method, js_name = moveByPath)]
    pub fn move_by_path(this: &Creep, path: &JsValue) -> ReturnCode;

    /// Move the creep toward the specified goal, either a [`RoomPosition`] or
    /// [`RoomObject`]. Note that using this function will store data in
    /// `Memory.creeps[creep_name]` and enable the default serialization
    /// behavior of the `Memory` object, which may hamper attempts to directly
    /// use `RawMemory`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.moveByPath)
    #[wasm_bindgen(final, method, js_name = moveTo)]
    fn move_to_internal(this: &Creep, target: &JsValue, options: &JsValue) -> ReturnCode;

    /// Whether to send an email notification when this creep is attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.notifyWhenAttacked)
    #[wasm_bindgen(final, method, js_name = notifyWhenAttacked)]
    pub fn notify_when_attacked(this: &Creep, enabled: bool) -> ReturnCode;

    /// Pick up a [`Resource`] in melee range (or at the same position as the
    /// creep).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.pickup)
    #[wasm_bindgen(final, method)]
    pub fn pickup(this: &Creep, target: &Resource) -> ReturnCode;

    /// Help another creep to move by pulling, if the second creep accepts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.pull)
    #[wasm_bindgen(final, method)]
    pub fn pull(this: &Creep, target: &Creep) -> ReturnCode;

    #[wasm_bindgen(final, method, js_name = rangedAttack)]
    fn ranged_attack_internal(this: &Creep, target: &RoomObject) -> ReturnCode;

    #[wasm_bindgen(final, method, js_name = rangedHeal)]
    fn ranged_heal_internal(this: &Creep, target: &RoomObject) -> ReturnCode;

    /// Attack all enemy targets in range using a creep's ranged attack parts,
    /// with lower damage depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedMassAttack)
    #[wasm_bindgen(final, method, js_name = rangedMassAttack)]
    pub fn ranged_mass_attack(this: &Creep) -> ReturnCode;

    /// Repair a target in range 3 using carried energy and the creep's work
    /// parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.repair)
    #[wasm_bindgen(final, method)]
    pub fn repair(this: &Creep, target: &RoomObject) -> ReturnCode;

    /// Reserve an unowned [`StructureController`] in melee range using a
    /// creep's claim parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.reserveController)
    #[wasm_bindgen(final, method, js_name = reserveController)]
    pub fn reserve_controller(this: &Creep, target: &StructureController) -> ReturnCode;

    /// Display a string in a bubble above the creep next tick. 10 character
    /// limit.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.say)
    #[wasm_bindgen(final, method)]
    pub fn say(this: &Creep, message: &str, public: bool) -> ReturnCode;

    /// Add (or remove, using an empty string) a sign to a
    /// [`StructureController`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.signController)
    #[wasm_bindgen(final, method, js_name = signController)]
    pub fn sign_controller(this: &Creep, target: &StructureController, text: &str) -> ReturnCode;

    /// Immediately kill the creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.suicide)
    #[wasm_bindgen(final, method)]
    pub fn suicide(this: &Creep) -> ReturnCode;

    /// Transfer a resource from the creep's store to [`Structure`],
    /// [`PowerCreep`], or another [`Creep`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.transfer)
    #[wasm_bindgen(final, method, js_name = transfer)]
    fn transfer_internal(
        this: &Creep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> ReturnCode;

    /// Upgrade a [`StructureController`] in range 3 using carried energy and
    /// the creep's work parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.upgradeController)
    #[wasm_bindgen(final, method, js_name = upgradeController)]
    pub fn upgrade_controller(this: &Creep, target: &StructureController) -> ReturnCode;

    /// Withdraw a resource from a [`Structure`], [`Tombstone`], or [`Ruin`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.withdraw)
    #[wasm_bindgen(final, method, js_name = withdraw)]
    fn withdraw_internal(
        this: &Creep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> ReturnCode;
}

#[wasm_bindgen]
extern "C" {
    /// A [`BodyPart`] of a creep.
    ///
    /// [Screeps documentation](https://docs-ptr.screeps.com/api/#Creep.body)
    #[wasm_bindgen]
    pub type BodyPart;

    #[wasm_bindgen(method, getter)]
    pub fn boost(this: &BodyPart) -> Option<ResourceType>;

    #[wasm_bindgen(method, getter = type)]
    pub fn part(this: &BodyPart) -> Part;

    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &BodyPart) -> u32;
}

impl Creep {
    /// Retrieve a [`Vec<BodyPart>`] containing details about the creep's body
    /// parts and boosts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.body)
    pub fn body(&self) -> Vec<BodyPart> {
        self.body_internal().iter().map(BodyPart::from).collect()
    }

    /// Attack a target in melee range using a creep's attack parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.attack)
    pub fn attack<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Attackable,
    {
        Self::attack_internal(self, target.as_ref())
    }

    // todo constant links - REPAIR_POWER, DISMANTLE_POWER, and buildable types
    // which I think we have
    /// Dismantle a [`Structure`] in melee range, giving the creep energy
    /// equivalent to half of the cost to repair the same hits. Must be a type
    /// of structure that can be constructed.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.dismantle)
    pub fn dismantle<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Dismantleable,
    {
        Self::dismantle_internal(self, target.as_ref())
    }
    
    /// Harvest from a [`Source`], [`Mineral`], or [`Deposit`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.harvest)
    ///
    /// [`Source`]: crate::objects::Source
    /// [`Mineral`]: crate::objects::Mineral
    /// [`Deposit`]: crate::objects::Deposit
    pub fn harvest<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Harvestable,
    {
        Self::harvest_internal(self, target.as_ref())
    }

    /// Heal a [`Creep`] or [`PowerCreep`] in melee range, including itself.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.heal)
    ///
    /// [`PowerCreep`]: crate::objects::PowerCreep
    pub fn heal<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Healable,
    {
        Self::heal_internal(self, target.as_ref())
    }

    /// Attack a target in range 3 using a creep's ranged attack parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedAttack)
    pub fn ranged_attack<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Attackable,
    {
        Self::ranged_attack_internal(self, target.as_ref())
    }

    /// Heal a target in range 3 using a creep's heal parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedHeal)
    pub fn ranged_heal<T>(&self, target: &T) -> ReturnCode
    where
        T: ?Sized + Healable,
    {
        Self::ranged_heal_internal(self, target.as_ref())
    }
}

impl JsCollectionFromValue for Creep {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

impl HasHits for Creep {
    fn hits(&self) -> u32 {
        Self::hits(self)
    }

    fn hits_max(&self) -> u32 {
        Self::hits_max(self)
    }
}

impl MaybeHasNativeId for Creep {
    fn try_native_id(&self) -> Option<JsString> {
        Self::id_internal(self)
    }
}

impl HasStore for Creep {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

impl SharedCreepProperties for Creep {
    fn memory(&self) -> JsValue {
        Self::memory(self)
    }

    fn set_memory(&self, val: &JsValue) {
        Self::set_memory(self, val)
    }

    fn my(&self) -> bool {
        Self::my(self)
    }

    fn name(&self) -> String {
        Self::name_internal(self).into()
    }

    fn owner(&self) -> Owner {
        Self::owner(self)
    }

    fn saying(&self) -> Option<JsString> {
        Self::saying(self)
    }

    fn ticks_to_live(&self) -> Option<u32> {
        Self::ticks_to_live(self)
    }

    fn cancel_order(&self, target: &JsString) -> ReturnCode {
        Self::cancel_order(self, target)
    }

    fn drop(&self, ty: ResourceType, amount: Option<u32>) -> ReturnCode {
        Self::drop(self, ty, amount)
    }

    fn move_direction(&self, direction: Direction) -> ReturnCode {
        Self::move_direction(self, direction)
    }

    fn move_by_path(&self, path: &JsValue) -> ReturnCode {
        Self::move_by_path(self, path)
    }

    fn move_to<T>(&self, target: T) -> ReturnCode
    where
        T: HasPosition,
    {
        let target: RoomPosition = target.pos().into();
        Self::move_to_internal(self, &target, &JsValue::UNDEFINED)
    }

    fn move_to_with_options<T, F>(&self, target: T, options: Option<MoveToOptions<F>>) -> ReturnCode
    where
        T: HasPosition,
        F: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult,
    {
        let target: RoomPosition = target.pos().into();

        if let Some(options) = options {
            options.into_js_options(|js_options| Self::move_to_internal(self, &target, js_options))
        } else {
            Self::move_to_internal(self, &target, &JsValue::UNDEFINED)
        }
    }

    fn notify_when_attacked(&self, enabled: bool) -> ReturnCode {
        Self::notify_when_attacked(self, enabled)
    }

    fn pickup(&self, target: &Resource) -> ReturnCode {
        Self::pickup(self, target)
    }

    fn say(&self, message: &str, public: bool) -> ReturnCode {
        Self::say(self, message, public)
    }

    fn suicide(&self) -> ReturnCode {
        Self::suicide(self)
    }

    fn transfer<T>(&self, target: &T, ty: ResourceType, amount: Option<u32>) -> ReturnCode
    where
        T: Transferable,
    {
        Self::transfer_internal(self, target.as_ref(), ty, amount)
    }

    fn withdraw<T>(&self, target: &T, ty: ResourceType, amount: Option<u32>) -> ReturnCode
    where
        T: Withdrawable,
    {
        Self::withdraw_internal(self, target.as_ref(), ty, amount)
    }
}
