use js_sys::{JsString, Object};
use wasm_bindgen::{prelude::*, JsCast};

use crate::{
    constants::{Direction, PowerCreepClass, PowerType, ResourceType, ReturnCode},
    local::RoomName,
    objects::{
        CostMatrix, MoveToOptions, Owner, Resource, RoomObject, RoomPosition, Store,
        StructureController, StructurePowerSpawn,
    },
    pathfinder::SingleRoomCostResult,
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// A [`PowerCreep`] unit in the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep)
    #[wasm_bindgen(extends = RoomObject)]
    #[derive(Clone, Debug)]
    pub type PowerCreep;

    /// Create a new power creep in your account. Note that it will not
    /// initially be spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.create)
    #[wasm_bindgen(static_method_of = PowerCreep)]
    pub fn create(name: &JsString, class: PowerCreepClass) -> ReturnCode;

    /// Retrieve this power creep's [`PowerCreepClass`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.className)
    #[wasm_bindgen(method, getter = className)]
    pub fn class(this: &PowerCreep) -> PowerCreepClass;

    /// Retrieve the current hits of this power creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.hits)
    #[wasm_bindgen(method, getter)]
    pub fn hits(this: &PowerCreep) -> u32;

    /// Retrieve the maximum hits of this power creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.hitsMax)
    #[wasm_bindgen(method, getter = hitsMax)]
    pub fn hits_max(this: &PowerCreep) -> u32;

    /// Object ID of the power creep, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks, or `None` if not
    /// spawned on the current shard.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.id)
    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &PowerCreep) -> JsString;

    /// Current level of the power creep, which can be increased with
    /// [`PowerCreep::upgrade`] if you have unspent GPL.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &PowerCreep) -> u32;

    /// A shortcut to `Memory.powerCreeps[power_creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.memory)
    #[wasm_bindgen(method, getter)]
    pub fn memory(this: &PowerCreep) -> JsValue;

    /// Sets a new value to `Memory.powerCreeps[power_creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.memory)
    #[wasm_bindgen(method, setter)]
    pub fn set_memory(this: &PowerCreep, val: &JsValue);

    /// Whether this power creep is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.my)
    #[wasm_bindgen(method, getter)]
    pub fn my(this: &PowerCreep) -> bool;

    /// The power creep's name as an owned reference to a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.name)
    #[wasm_bindgen(method, getter = name)]
    fn name_internal(this: &PowerCreep) -> JsString;

    /// The [`Owner`] of this power creep that contains the owner's username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.owner)
    #[wasm_bindgen(method, getter)]
    pub fn owner(this: &PowerCreep) -> Owner;

    #[wasm_bindgen(method, getter = powers)]
    fn powers_internal(this: &PowerCreep) -> Object;

    /// What the power creep said last tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.saying)
    #[wasm_bindgen(method, getter)]
    pub fn saying(this: &PowerCreep) -> Option<JsString>;

    /// The [`Store`] of the power creep, which contains information about what
    /// resources it is it carrying.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.store)
    #[wasm_bindgen(method, getter)]
    pub fn store(this: &PowerCreep) -> Store;

    /// The shard the power creep is currently spawned on, if spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.shard)
    #[wasm_bindgen(method, getter)]
    pub fn shard(this: &PowerCreep) -> Option<JsString>;

    /// The number of ticks the power creep has left to live, which can be
    /// renewed at a [`StructurePowerSpawn`] or [`StructurePowerBank`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.ticksToLive)
    #[wasm_bindgen(method, getter = ticksToLive)]
    pub fn ticks_to_live(this: &PowerCreep) -> Option<u32>;

    /// Cancel an a successfully called power creep function from earlier in the
    /// tick, with a [`JsString`] that must contain the JS version of the
    /// function name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.cancelOrder)
    #[wasm_bindgen(method, js_name = cancelOrder)]
    pub fn cancel_order(this: &PowerCreep, target: &JsString) -> ReturnCode;

    /// Drop a resource on the ground from the power creep's [`Store`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.drop)
    #[wasm_bindgen(method)]
    pub fn drop(this: &PowerCreep, ty: ResourceType, amount: Option<u32>) -> ReturnCode;

    /// Enable powers to be used in this room on a [`StructureController`] in
    /// melee range. You do not need to own the controller.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.enableRoom)
    #[wasm_bindgen(method, js_name = enableRoom)]
    pub fn enable_room(this: &PowerCreep, target: &StructureController) -> ReturnCode;

    /// Move one square in the specified direction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.move)
    #[wasm_bindgen(method, js_name = move)]
    pub fn move_direction(this: &PowerCreep, direction: Direction) -> ReturnCode;

    /// Move the power creep along a previously determined path returned from a
    /// pathfinding function, in array or serialized string form.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.moveByPath)
    #[wasm_bindgen(method, js_name = moveByPath)]
    pub fn move_by_path(this: &PowerCreep, path: &JsValue) -> ReturnCode;

    /// Move the creep toward the specified goal, either a [`RoomPosition`] or
    /// [`RoomObject`]. Note that using this function will store data in
    /// `Memory.creeps[creep_name]` and enable the default serialization
    /// behavior of the `Memory` object, which may hamper attempts to directly
    /// use `RawMemory`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.moveByPath)
    #[wasm_bindgen(method, js_name = moveTo)]
    fn move_to_internal(this: &PowerCreep, target: &JsValue, options: &JsValue) -> ReturnCode;

    /// Whether to send an email notification when this power creep is attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.notifyWhenAttacked)
    #[wasm_bindgen(method, js_name = notifyWhenAttacked)]
    pub fn notify_when_attacked(this: &PowerCreep, enabled: bool) -> ReturnCode;

    /// Pick up a [`Resource`] in melee range (or at the same position as the
    /// creep).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.pickup)
    #[wasm_bindgen(method)]
    pub fn pickup(this: &PowerCreep, target: &Resource) -> ReturnCode;

    /// Renew the power creep's TTL using a [`StructurePowerSpawn`] or
    /// [`StructurePowerBank`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.renew)
    #[wasm_bindgen(method)]
    pub fn renew(this: &PowerCreep, target: &RoomObject) -> ReturnCode;

    /// Display a string in a bubble above the power creep next tick. 10
    /// character limit.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.say)
    #[wasm_bindgen(method)]
    pub fn say(this: &PowerCreep, message: &str, public: bool) -> ReturnCode;

    /// Immediately kill the power creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.suicide)
    #[wasm_bindgen(method)]
    pub fn suicide(this: &PowerCreep) -> ReturnCode;

    /// Transfer a resource from the power creep's store to [`Structure`],
    /// [`Creep`], or another [`PowerCreep`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.transfer)
    #[wasm_bindgen(method)]
    fn transfer_internal(
        this: &PowerCreep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> ReturnCode;

    /// Use one of the power creep's powers.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.usePower)
    #[wasm_bindgen(method, js_name = usePower)]
    pub fn use_power(
        this: &PowerCreep,
        power: PowerType,
        target: Option<&RoomObject>,
    ) -> ReturnCode;

    /// Withdraw a resource from a [`Structure`], [`Tombstone`], or [`Ruin`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.withdraw)
    #[wasm_bindgen(method)]
    fn withdraw_internal(
        this: &PowerCreep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> ReturnCode;
}

impl PowerCreep {
    /// The levels of this power creep's abilities, with [`PowerType`] keys and
    /// values containing power level and cooldown.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.powers)
    pub fn powers(&self) -> JsHashMap<PowerType, PowerInfo> {
        self.powers_internal().into()
    }
}

impl HasHits for PowerCreep {
    fn hits(&self) -> u32 {
        Self::hits(self)
    }

    fn hits_max(&self) -> u32 {
        Self::hits_max(self)
    }
}

impl HasNativeId for PowerCreep {
    fn native_id(&self) -> JsString {
        Self::id_internal(self)
    }
}

impl HasStore for PowerCreep {
    fn store(&self) -> Store {
        Self::store(self)
    }
}

// todo
// impl TryFrom<AccountPowerCreep> for PowerCreep

impl SharedCreepProperties for PowerCreep {
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

#[wasm_bindgen]
extern "C" {
    /// A [`PowerCreep`] unit that may or may not be spawned in the current
    /// shard of the game world.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep)
    #[derive(Clone, Debug)]
    pub type AccountPowerCreep;

    /// Retrieve this power creep's [`PowerCreepClass`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.className)
    #[wasm_bindgen(method, getter = className)]
    pub fn class(this: &AccountPowerCreep) -> PowerCreepClass;

    // todo should be u64 but seems to panic at the moment, follow up
    /// The timestamp, in milliseconds since epoch, when the [`PowerCreep`] will
    /// be permanently deleted due to [`PowerCreep::delete`]. Can be cancelled
    /// with the same function until then.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.deleteTime)
    #[wasm_bindgen(method, getter = deleteTime)]
    pub fn delete_time(this: &AccountPowerCreep) -> Option<f64>;

    /// Current level of the power creep, which can be increased with
    /// [`PowerCreep::upgrade`] if you have unspent GPL.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.level)
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &AccountPowerCreep) -> u32;

    /// The power creep's name as an owned reference to a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.name)
    #[wasm_bindgen(method, getter = name)]
    fn name_internal(this: &AccountPowerCreep) -> JsString;

    #[wasm_bindgen(method, getter = powers)]
    fn powers_internal(this: &AccountPowerCreep) -> Object;

    /// The shard the power creep is currently spawned on, if spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.shard)
    #[wasm_bindgen(method, getter)]
    pub fn shard(this: &AccountPowerCreep) -> Option<JsString>;

    // todo should be u64 but seems to panic at the moment, follow up
    /// The timestamp, in milliseconds since epoch, when the power creep will be
    /// allowed to spawn again after dying.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.spawnCooldownTime)
    #[wasm_bindgen(method, getter = spawnCooldownTime)]
    pub fn spawn_cooldown_time(this: &AccountPowerCreep) -> Option<f64>;

    /// Set a power creep that is not currently spawned to be deleted. Can be
    /// cancelled with `true` for the cancel paramater.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.delete)
    #[wasm_bindgen(method)]
    pub fn delete(this: &AccountPowerCreep, cancel: bool) -> ReturnCode;

    /// Change the name of the power creep. Must not be spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.rename)
    #[wasm_bindgen(method)]
    pub fn rename(this: &AccountPowerCreep, name: &JsString) -> ReturnCode;

    /// Spawn the power creep at a [`StructurePowerSpawn`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.spawn)
    #[wasm_bindgen(method)]
    pub fn spawn(this: &AccountPowerCreep, target: &StructurePowerSpawn) -> ReturnCode;

    /// Upgrade this power creep, consuming one available GPL and adding a new
    /// level to one of its powers.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.upgrade)
    #[wasm_bindgen(method)]
    pub fn upgrade(this: &AccountPowerCreep, power: PowerType) -> ReturnCode;
}

impl AccountPowerCreep {
    /// The levels of this power creep's abilities, with [`PowerType`] keys and
    /// values containing power level and cooldown.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.powers)
    pub fn powers(&self) -> JsHashMap<PowerType, PowerInfo> {
        self.powers_internal().into()
    }
}

impl JsCollectionFromValue for AccountPowerCreep {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen]
    #[derive(Debug)]
    pub type PowerInfo;
    #[wasm_bindgen(method, getter)]
    pub fn cooldown(this: &PowerInfo) -> u32;
    #[wasm_bindgen(method, getter)]
    pub fn level(this: &PowerInfo) -> u8;
}

impl JsCollectionFromValue for PowerInfo {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}
