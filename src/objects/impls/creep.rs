use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

use crate::{
    constants::{Direction, ErrorCode, Part, ResourceType},
    objects::{
        ConstructionSite, Owner, Resource, RoomObject, Store, Structure, StructureController,
    },
    pathfinder::SingleRoomCostResult,
    prelude::*,
    CostMatrix, MoveToOptions, RoomName, RoomPosition,
};

#[cfg(feature = "seasonal-season-5")]
use crate::objects::Reactor;

#[wasm_bindgen]
extern "C" {
    /// A [`Creep`] unit in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type Creep;

    // explicitly structural getters here, as these may have varying
    // getter methods (notably on the first tick a creep is spawning):
    // https://github.com/screeps/engine/blob/c6c4fc9e656f160e0e0174b0dd9a817d2dd18976/src/game/structures.js#L1134-L1213
    #[wasm_bindgen(structural, method, getter = body)]
    fn body_internal(this: &Creep) -> Array;

    #[wasm_bindgen(structural, method, getter = fatigue)]
    fn fatigue_internal(this: &Creep) -> u32;

    #[wasm_bindgen(structural, method, getter = hits)]
    fn hits_internal(this: &Creep) -> u32;

    #[wasm_bindgen(structural, method, getter = hitsMax)]
    fn hits_max_internal(this: &Creep) -> u32;

    #[wasm_bindgen(structural, method, getter = id)]
    fn id_internal(this: &Creep) -> Option<JsString>;

    #[wasm_bindgen(structural, method, getter = memory)]
    fn memory_internal(this: &Creep) -> JsValue;

    #[wasm_bindgen(structural, method, setter = memory)]
    fn set_memory_internal(this: &Creep, val: &JsValue);

    #[wasm_bindgen(structural, method, getter = my)]
    fn my_internal(this: &Creep) -> bool;

    #[wasm_bindgen(structural, method, getter = name)]
    fn name_internal(this: &Creep) -> String;

    #[wasm_bindgen(structural, method, getter = name)]
    fn name_jsstring_internal(this: &Creep) -> JsString;

    #[wasm_bindgen(structural, method, getter = owner)]
    fn owner_internal(this: &Creep) -> Owner;

    #[wasm_bindgen(structural, method, getter = saying)]
    fn saying_internal(this: &Creep) -> Option<JsString>;

    #[wasm_bindgen(structural, method, getter = spawning)]
    fn spawning_internal(this: &Creep) -> bool;

    #[wasm_bindgen(structural, method, getter = store)]
    fn store_internal(this: &Creep) -> Store;

    #[wasm_bindgen(structural, method, getter = ticksToLive)]
    fn ticks_to_live_internal(this: &Creep) -> Option<u32>;

    #[wasm_bindgen(final, method, js_name = attack)]
    fn attack_internal(this: &Creep, target: &RoomObject) -> i8;

    #[wasm_bindgen(final, method, js_name = attackController)]
    fn attack_controller_internal(this: &Creep, target: &StructureController) -> i8;

    #[wasm_bindgen(final, method, js_name = build)]
    fn build_internal(this: &Creep, target: &ConstructionSite) -> i8;

    #[wasm_bindgen(final, method, js_name = cancelOrder)]
    fn cancel_order_internal(this: &Creep, target: &JsString) -> i8;

    #[wasm_bindgen(final, method, js_name = claimController)]
    fn claim_controller_internal(this: &Creep, target: &StructureController) -> i8;

    #[cfg(feature = "seasonal-season-5")]
    #[wasm_bindgen(final, method, js_name = claimReactor)]
    fn claim_reactor_internal(this: &Creep, target: &Reactor) -> i8;

    #[wasm_bindgen(final, method, js_name = dismantle)]
    fn dismantle_internal(this: &Creep, target: &Structure) -> i8;

    #[wasm_bindgen(final, method, js_name = drop)]
    fn drop_internal(this: &Creep, ty: ResourceType, amount: Option<u32>) -> i8;

    #[wasm_bindgen(final, method, js_name = generateSafeMode)]
    fn generate_safe_mode_internal(this: &Creep, target: &StructureController) -> i8;

    #[wasm_bindgen(final, method, js_name = getActiveBodyparts)]
    fn get_active_bodyparts_internal(this: &Creep, ty: Part) -> u8;

    #[wasm_bindgen(final, method, js_name = harvest)]
    fn harvest_internal(this: &Creep, target: &RoomObject) -> i8;

    #[wasm_bindgen(final, method, js_name = heal)]
    fn heal_internal(this: &Creep, target: &RoomObject) -> i8;

    #[wasm_bindgen(final, method, js_name = move)]
    fn move_direction_internal(this: &Creep, direction: Direction) -> i8;

    #[wasm_bindgen(final, method, js_name = move)]
    fn move_pulled_by_internal(this: &Creep, target: &Creep) -> i8;

    #[wasm_bindgen(final, method, js_name = moveByPath)]
    fn move_by_path_internal(this: &Creep, path: &JsValue) -> i8;

    #[wasm_bindgen(final, method, js_name = moveTo)]
    fn move_to_internal(this: &Creep, target: &JsValue, options: &JsValue) -> i8;

    #[wasm_bindgen(final, method, js_name = notifyWhenAttacked)]
    fn notify_when_attacked_internal(this: &Creep, enabled: bool) -> i8;

    #[wasm_bindgen(final, method, js_name = pickup)]
    fn pickup_internal(this: &Creep, target: &Resource) -> i8;

    #[wasm_bindgen(final, method, js_name = pull)]
    fn pull_internal(this: &Creep, target: &Creep) -> i8;

    #[wasm_bindgen(final, method, js_name = rangedAttack)]
    fn ranged_attack_internal(this: &Creep, target: &RoomObject) -> i8;

    #[wasm_bindgen(final, method, js_name = rangedHeal)]
    fn ranged_heal_internal(this: &Creep, target: &RoomObject) -> i8;

    #[wasm_bindgen(final, method, js_name = rangedMassAttack)]
    fn ranged_mass_attack_internal(this: &Creep) -> i8;

    #[wasm_bindgen(final, method, js_name = repair)]
    fn repair_internal(this: &Creep, target: &RoomObject) -> i8;

    #[wasm_bindgen(final, method, js_name = reserveController)]
    fn reserve_controller_internal(this: &Creep, target: &StructureController) -> i8;

    #[wasm_bindgen(final, method, js_name = say)]
    fn say_internal(this: &Creep, message: &str, public: bool) -> i8;

    #[wasm_bindgen(final, method, js_name = signController)]
    fn sign_controller_internal(this: &Creep, target: &StructureController, text: &str) -> i8;

    #[wasm_bindgen(final, method, js_name = suicide)]
    fn suicide_internal(this: &Creep) -> i8;

    #[wasm_bindgen(final, method, js_name = transfer)]
    fn transfer_internal(
        this: &Creep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> i8;

    #[wasm_bindgen(final, method, js_name = upgradeController)]
    fn upgrade_controller_internal(this: &Creep, target: &StructureController) -> i8;

    #[wasm_bindgen(final, method, js_name = withdraw)]
    fn withdraw_internal(
        this: &Creep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> i8;
}

impl Creep {
    /// Retrieve a [`Vec<BodyPart>`] containing details about the creep's body
    /// parts and boosts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.body)
    pub fn body(&self) -> Vec<BodyPart> {
        self.body_internal().iter().map(BodyPart::from).collect()
    }

    /// The amount of fatigue the creep has. If greater than 0, it cannot move
    /// this tick without being pulled.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.fatigue)
    pub fn fatigue(&self) -> u32 {
        self.fatigue_internal()
    }

    /// Retrieve the current hits of this creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.hits)
    pub fn hits(&self) -> u32 {
        self.hits_internal()
    }

    /// Retrieve the maximum hits of this creep, which generally equals 50 per
    /// body part.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.hitsMax)
    pub fn hits_max(&self) -> u32 {
        self.hits_max_internal()
    }

    /// A shortcut to `Memory.creeps[creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.memory)
    pub fn memory(&self) -> JsValue {
        self.memory_internal()
    }

    /// Sets a new value to `Memory.creeps[creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.memory)
    pub fn set_memory(&self, val: &JsValue) {
        self.set_memory_internal(val)
    }

    /// Whether this creep is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.my)
    pub fn my(&self) -> bool {
        self.my_internal()
    }

    /// The [`Owner`] of this creep that contains the owner's username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.owner)
    pub fn owner(&self) -> Owner {
        self.owner_internal()
    }

    /// What the creep said last tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.saying)
    pub fn saying(&self) -> Option<JsString> {
        self.saying_internal()
    }

    /// Whether the creep is still spawning.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.spawning)
    pub fn spawning(&self) -> bool {
        self.spawning_internal()
    }

    /// The [`Store`] of the creep, which contains information about what
    /// resources it is it carrying.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.store)
    pub fn store(&self) -> Store {
        self.store_internal()
    }

    /// The number of ticks the creep has left to live
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.ticksToLive)
    pub fn ticks_to_live(&self) -> Option<u32> {
        self.ticks_to_live_internal()
    }

    /// Attack a target in melee range using a creep's attack parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.attack)
    pub fn attack<T>(&self, target: &T) -> Result<(), ErrorCode>
    where
        T: ?Sized + Attackable,
    {
        ErrorCode::result_from_i8(self.attack_internal(target.as_ref()))
    }

    /// Attack a [`StructureController`] in melee range using a creep's claim
    /// parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.attackController)
    pub fn attack_controller(&self, target: &StructureController) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.attack_controller_internal(target))
    }

    /// Use a creep's work parts to consume carried energy, putting it toward
    /// progress in a [`ConstructionSite`] in range 3.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.build)
    pub fn build(&self, target: &ConstructionSite) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.build_internal(target))
    }

    /// Cancel an a successfully called creep function from earlier in the tick,
    /// with a [`JsString`] that must contain the JS version of the function
    /// name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.cancelOrder)
    pub fn cancel_order(&self, target: &JsString) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.cancel_order_internal(target))
    }

    /// Claim an unowned [`StructureController`] in melee range as your own
    /// using a creep's claim parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.claimController)
    pub fn claim_controller(&self, target: &StructureController) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.claim_controller_internal(target))
    }

    /// Claim a [`Reactor`] in melee range as your own using a creep's claim
    /// parts.
    ///
    /// [Screeps documentation](https://docs-season.screeps.com/api/#Creep.claimReactor)
    #[cfg(feature = "seasonal-season-5")]
    pub fn claim_reactor(&self, target: &Reactor) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.claim_reactor_internal(target))
    }

    /// Dismantle a [`Structure`] in melee range, removing [`DISMANTLE_POWER`]
    /// hits per effective work part, giving the creep energy equivalent to half
    /// of the cost to repair the same hits. Can only be used against types
    /// of structures that can be constructed; if
    /// [`StructureType::construction_cost`] is `None`, dismantling is
    /// impossible.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.dismantle)
    ///
    /// [`DISMANTLE_POWER`]: crate::constants::DISMANTLE_POWER
    /// [`StructureType::construction_cost`]: crate::constants::StructureType::construction_cost
    pub fn dismantle<T>(&self, target: &T) -> Result<(), ErrorCode>
    where
        T: ?Sized + Dismantleable,
    {
        ErrorCode::result_from_i8(self.dismantle_internal(target.as_ref()))
    }

    /// Drop a resource on the ground from the creep's [`Store`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.drop)
    pub fn drop(&self, ty: ResourceType, amount: Option<u32>) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.drop_internal(ty, amount))
    }

    /// Consume [`ResourceType::Ghodium`] (in the amount of [`SAFE_MODE_COST`])
    /// from the creep's [`Store`] to add a safe mode activation to a
    /// [`StructureController`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.generateSafeMode)
    ///
    /// [`SAFE_MODE_COST`]: crate::constants::SAFE_MODE_COST
    pub fn generate_safe_mode(&self, target: &StructureController) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.generate_safe_mode_internal(target))
    }

    /// Get the number of parts of the given type the creep has in its body,
    /// excluding fully damaged parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.getActiveBodyparts)
    pub fn get_active_bodyparts(&self, ty: Part) -> u8 {
        self.get_active_bodyparts_internal(ty)
    }

    /// Harvest from a [`Source`], [`Mineral`], or [`Deposit`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.harvest)
    ///
    /// [`Source`]: crate::objects::Source
    /// [`Mineral`]: crate::objects::Mineral
    /// [`Deposit`]: crate::objects::Deposit
    pub fn harvest<T>(&self, target: &T) -> Result<(), ErrorCode>
    where
        T: ?Sized + Harvestable,
    {
        ErrorCode::result_from_i8(self.harvest_internal(target.as_ref()))
    }

    /// Heal a [`Creep`] or [`PowerCreep`] in melee range, including itself.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.heal)
    ///
    /// [`PowerCreep`]: crate::objects::PowerCreep
    pub fn heal<T>(&self, target: &T) -> Result<(), ErrorCode>
    where
        T: ?Sized + Healable,
    {
        ErrorCode::result_from_i8(self.heal_internal(target.as_ref()))
    }

    /// Move one square in the specified direction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.move)
    pub fn move_direction(&self, direction: Direction) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.move_direction_internal(direction))
    }

    /// Accept an attempt by another creep to pull this one.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.move)
    pub fn move_pulled_by(&self, target: &Creep) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.move_pulled_by_internal(target))
    }

    /// Move the creep along a previously determined path returned from a
    /// pathfinding function, in array or serialized string form.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.moveByPath)
    pub fn move_by_path(&self, path: &JsValue) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.move_by_path_internal(path))
    }

    /// Whether to send an email notification when this creep is attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.notifyWhenAttacked)
    pub fn notify_when_attacked(&self, enabled: bool) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.notify_when_attacked_internal(enabled))
    }

    /// Pick up a [`Resource`] in melee range (or at the same position as the
    /// creep).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.pickup)
    pub fn pickup(&self, target: &Resource) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.pickup_internal(target))
    }

    /// Help another creep to move by pulling, if the second creep accepts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.pull)
    pub fn pull(&self, target: &Creep) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.pull_internal(target))
    }

    /// Attack a target in range 3 using a creep's ranged attack parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedAttack)
    pub fn ranged_attack<T>(&self, target: &T) -> Result<(), ErrorCode>
    where
        T: ?Sized + Attackable,
    {
        ErrorCode::result_from_i8(self.ranged_attack_internal(target.as_ref()))
    }

    /// Heal a target in range 3 using a creep's heal parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedHeal)
    pub fn ranged_heal<T>(&self, target: &T) -> Result<(), ErrorCode>
    where
        T: ?Sized + Healable,
    {
        ErrorCode::result_from_i8(self.ranged_heal_internal(target.as_ref()))
    }

    /// Attack all enemy targets in range using a creep's ranged attack parts,
    /// with lower damage depending on range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.rangedMassAttack)
    pub fn ranged_mass_attack(&self) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.ranged_mass_attack_internal())
    }

    /// Repair a target in range 3 using carried energy and the creep's work
    /// parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.repair)
    pub fn repair<T>(&self, target: &T) -> Result<(), ErrorCode>
    where
        T: ?Sized + Repairable,
    {
        ErrorCode::result_from_i8(self.repair_internal(target.as_ref()))
    }

    /// Reserve an unowned [`StructureController`] in melee range using a
    /// creep's claim parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.reserveController)
    pub fn reserve_controller(&self, target: &StructureController) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.reserve_controller_internal(target))
    }

    /// Display a string in a bubble above the creep next tick. 10 character
    /// limit.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.say)
    pub fn say(&self, message: &str, public: bool) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.say_internal(message, public))
    }

    /// Add (or remove, using an empty string) a sign to a
    /// [`StructureController`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.signController)
    pub fn sign_controller(
        &self,
        target: &StructureController,
        text: &str,
    ) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.sign_controller_internal(target, text))
    }

    /// Immediately kill the creep.
    ///
    /// Actions taken by the creep earlier in the tick may be cancelled.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.suicide)
    pub fn suicide(&self) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.suicide_internal())
    }

    /// Upgrade a [`StructureController`] in range 3 using carried energy and
    /// the creep's work parts.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.upgradeController)
    pub fn upgrade_controller(&self, target: &StructureController) -> Result<(), ErrorCode> {
        ErrorCode::result_from_i8(self.upgrade_controller_internal(target))
    }
}

impl JsCollectionFromValue for Creep {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

impl HasHits for Creep {
    fn hits(&self) -> u32 {
        self.hits()
    }

    fn hits_max(&self) -> u32 {
        self.hits_max()
    }
}

impl MaybeHasId for Creep {
    /// The Object ID of the [`Creep`], or `None` if it began spawning this
    /// tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.id)
    fn try_js_raw_id(&self) -> Option<JsString> {
        self.id_internal()
    }
}

impl HasStore for Creep {
    fn store(&self) -> Store {
        self.store()
    }
}

impl Attackable for Creep {}
impl Healable for Creep {}
impl Transferable for Creep {}

impl SharedCreepProperties for Creep {
    fn memory(&self) -> JsValue {
        self.memory()
    }

    fn set_memory(&self, val: &JsValue) {
        self.set_memory(val)
    }

    fn my(&self) -> bool {
        self.my()
    }

    fn name(&self) -> String {
        self.name_internal()
    }

    fn name_jsstring(&self) -> JsString {
        self.name_jsstring_internal()
    }

    fn owner(&self) -> Owner {
        self.owner()
    }

    fn saying(&self) -> Option<JsString> {
        self.saying()
    }

    fn ticks_to_live(&self) -> Option<u32> {
        self.ticks_to_live()
    }

    fn cancel_order(&self, target: &JsString) -> Result<(), ErrorCode> {
        self.cancel_order(target)
    }

    fn drop(&self, ty: ResourceType, amount: Option<u32>) -> Result<(), ErrorCode> {
        self.drop(ty, amount)
    }

    fn move_direction(&self, direction: Direction) -> Result<(), ErrorCode> {
        self.move_direction(direction)
    }

    fn move_by_path(&self, path: &JsValue) -> Result<(), ErrorCode> {
        self.move_by_path(path)
    }

    fn move_to<T>(&self, target: T) -> Result<(), ErrorCode>
    where
        T: HasPosition,
    {
        let target: RoomPosition = target.pos().into();
        ErrorCode::result_from_i8(self.move_to_internal(&target, &JsValue::UNDEFINED))
    }

    fn move_to_with_options<T, F>(
        &self,
        target: T,
        options: Option<MoveToOptions<F>>,
    ) -> Result<(), ErrorCode>
    where
        T: HasPosition,
        F: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult,
    {
        let target: RoomPosition = target.pos().into();

        if let Some(options) = options {
            options.into_js_options(|js_options| {
                ErrorCode::result_from_i8(self.move_to_internal(&target, js_options))
            })
        } else {
            ErrorCode::result_from_i8(self.move_to_internal(&target, &JsValue::UNDEFINED))
        }
    }

    fn notify_when_attacked(&self, enabled: bool) -> Result<(), ErrorCode> {
        self.notify_when_attacked(enabled)
    }

    fn pickup(&self, target: &Resource) -> Result<(), ErrorCode> {
        self.pickup(target)
    }

    fn say(&self, message: &str, public: bool) -> Result<(), ErrorCode> {
        self.say(message, public)
    }

    fn suicide(&self) -> Result<(), ErrorCode> {
        self.suicide()
    }

    fn transfer<T>(
        &self,
        target: &T,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> Result<(), ErrorCode>
    where
        T: Transferable + ?Sized,
    {
        ErrorCode::result_from_i8(self.transfer_internal(target.as_ref(), ty, amount))
    }

    fn withdraw<T>(
        &self,
        target: &T,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> Result<(), ErrorCode>
    where
        T: Withdrawable + ?Sized,
    {
        ErrorCode::result_from_i8(self.withdraw_internal(target.as_ref(), ty, amount))
    }
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
