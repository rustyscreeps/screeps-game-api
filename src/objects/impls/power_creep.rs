use std::{error::Error, fmt};

use js_sys::{JsString, Object};
use wasm_bindgen::prelude::*;

use crate::{
    constants::{Direction, PowerCreepClass, PowerType, ResourceType},
    enums::action_error_codes::{
        DeleteErrorCode, DropErrorCode, EnableRoomErrorCode, NotifyWhenAttackedErrorCode,
        PickupErrorCode, PowerCreepCancelOrderErrorCode, PowerCreepCreateErrorCode,
        PowerCreepMoveByPathErrorCode, PowerCreepMoveDirectionErrorCode, PowerCreepMoveToErrorCode,
        RenameErrorCode, RenewErrorCode, SayErrorCode, SpawnErrorCode, SuicideErrorCode,
        TransferErrorCode, UpgradeErrorCode, UsePowerErrorCode, WithdrawErrorCode,
    },
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

    #[wasm_bindgen(static_method_of = PowerCreep)]
    fn create_internal(name: &JsString, class: PowerCreepClass) -> i8;

    #[wasm_bindgen(method, getter = className)]
    fn class_internal(this: &PowerCreep) -> PowerCreepClass;

    #[wasm_bindgen(method, getter = hits)]
    fn hits_internal(this: &PowerCreep) -> u32;

    #[wasm_bindgen(method, getter = hitsMax)]
    fn hits_max_internal(this: &PowerCreep) -> u32;

    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &PowerCreep) -> JsString;

    #[wasm_bindgen(method, getter = level)]
    fn level_internal(this: &PowerCreep) -> u32;

    #[wasm_bindgen(method, getter = memory)]
    fn memory_internal(this: &PowerCreep) -> JsValue;

    #[wasm_bindgen(method, setter = memory)]
    fn set_memory_internal(this: &PowerCreep, val: &JsValue);

    #[wasm_bindgen(method, getter = my)]
    fn my_internal(this: &PowerCreep) -> bool;

    #[wasm_bindgen(method, getter = name)]
    fn name_internal(this: &PowerCreep) -> String;

    #[wasm_bindgen(method, getter = name)]
    fn name_jsstring_internal(this: &PowerCreep) -> JsString;

    #[wasm_bindgen(method, getter = owner)]
    fn owner_internal(this: &PowerCreep) -> Owner;

    #[wasm_bindgen(method, getter = powers)]
    fn powers_internal(this: &PowerCreep) -> Object;

    #[wasm_bindgen(method, getter = saying)]
    fn saying_internal(this: &PowerCreep) -> Option<JsString>;

    #[wasm_bindgen(method, getter = store)]
    fn store_internal(this: &PowerCreep) -> Store;

    #[wasm_bindgen(method, getter = shard)]
    fn shard_internal(this: &PowerCreep) -> Option<JsString>;

    #[wasm_bindgen(method, getter = ticksToLive)]
    fn ticks_to_live_internal(this: &PowerCreep) -> Option<u32>;

    #[wasm_bindgen(method, js_name = cancelOrder)]
    fn cancel_order_internal(this: &PowerCreep, target: &JsString) -> i8;

    #[wasm_bindgen(method, js_name = drop)]
    fn drop_internal(this: &PowerCreep, ty: ResourceType, amount: Option<u32>) -> i8;

    #[wasm_bindgen(method, js_name = enableRoom)]
    fn enable_room_internal(this: &PowerCreep, target: &StructureController) -> i8;

    #[wasm_bindgen(method, js_name = move)]
    fn move_direction_internal(this: &PowerCreep, direction: Direction) -> i8;

    #[wasm_bindgen(method, js_name = moveByPath)]
    fn move_by_path_internal(this: &PowerCreep, path: &JsValue) -> i8;

    #[wasm_bindgen(method, js_name = moveTo)]
    fn move_to_internal(this: &PowerCreep, target: &JsValue, options: &JsValue) -> i8;

    #[wasm_bindgen(method, js_name = notifyWhenAttacked)]
    fn notify_when_attacked_internal(this: &PowerCreep, enabled: bool) -> i8;

    #[wasm_bindgen(method, js_name = pickup)]
    fn pickup_internal(this: &PowerCreep, target: &Resource) -> i8;

    #[wasm_bindgen(method, js_name = renew)]
    fn renew_internal(this: &PowerCreep, target: &RoomObject) -> i8;

    #[wasm_bindgen(method, js_name = say)]
    fn say_internal(this: &PowerCreep, message: &str, public: bool) -> i8;

    #[wasm_bindgen(method, js_name = suicide)]
    fn suicide_internal(this: &PowerCreep) -> i8;

    #[wasm_bindgen(method, js_name = transfer)]
    fn transfer_internal(
        this: &PowerCreep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> i8;

    #[wasm_bindgen(method, js_name = usePower)]
    fn use_power_internal(this: &PowerCreep, power: PowerType, target: Option<&RoomObject>) -> i8;

    #[wasm_bindgen(method, js_name = withdraw)]
    fn withdraw_internal(
        this: &PowerCreep,
        target: &RoomObject,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> i8;
}

impl PowerCreep {
    /// Create a new power creep in your account. Note that it will not
    /// initially be spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.create)
    pub fn create(
        name: &JsString,
        class: PowerCreepClass,
    ) -> Result<(), PowerCreepCreateErrorCode> {
        PowerCreepCreateErrorCode::result_from_i8(Self::create_internal(name, class))
    }

    /// Retrieve this power creep's [`PowerCreepClass`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.className)
    pub fn class(&self) -> PowerCreepClass {
        self.class_internal()
    }

    /// Retrieve the current hits of this power creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.hits)
    pub fn hits(&self) -> u32 {
        self.hits_internal()
    }

    /// Retrieve the maximum hits of this power creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.hitsMax)
    pub fn hits_max(&self) -> u32 {
        self.hits_max_internal()
    }

    /// Current level of the power creep, which can be increased with
    /// [`AccountPowerCreep::upgrade`] if you have unspent GPL.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.level)
    pub fn level(&self) -> u32 {
        self.level_internal()
    }

    /// A shortcut to `Memory.powerCreeps[power_creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.memory)
    pub fn memory(&self) -> JsValue {
        self.memory_internal()
    }

    /// Sets a new value to `Memory.powerCreeps[power_creep.name]`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.memory)
    pub fn set_memory(&self, val: &JsValue) {
        self.set_memory_internal(val)
    }

    /// Whether this power creep is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.my)
    pub fn my(&self) -> bool {
        self.my_internal()
    }

    /// The [`Owner`] of this power creep that contains the owner's username.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.owner)
    pub fn owner(&self) -> Owner {
        self.owner_internal()
    }

    /// The levels of this power creep's abilities, with [`PowerType`] keys and
    /// values containing power level and cooldown.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.powers)
    pub fn powers(&self) -> JsHashMap<PowerType, PowerInfo> {
        self.powers_internal().into()
    }

    /// What the power creep said last tick.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.saying)
    pub fn saying(&self) -> Option<JsString> {
        self.saying_internal()
    }

    /// The [`Store`] of the power creep, which contains information about what
    /// resources it is it carrying.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.store)
    pub fn store(&self) -> Store {
        self.store_internal()
    }

    /// The shard the power creep is currently spawned on, if spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.shard)
    pub fn shard(&self) -> Option<JsString> {
        self.shard_internal()
    }

    /// The number of ticks the power creep has left to live, which can be
    /// renewed at a [`StructurePowerSpawn`] or [`StructurePowerBank`]
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.ticksToLive)
    ///
    /// [`StructurePowerBank`]: crate::objects::StructurePowerBank
    pub fn ticks_to_live(&self) -> Option<u32> {
        self.ticks_to_live_internal()
    }

    /// Cancel an a successfully called power creep function from earlier in the
    /// tick, with a [`JsString`] that must contain the JS version of the
    /// function name.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.cancelOrder)
    pub fn cancel_order(&self, target: &JsString) -> Result<(), PowerCreepCancelOrderErrorCode> {
        PowerCreepCancelOrderErrorCode::result_from_i8(self.cancel_order_internal(target))
    }

    /// Drop a resource on the ground from the power creep's [`Store`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.drop)
    pub fn drop(&self, ty: ResourceType, amount: Option<u32>) -> Result<(), DropErrorCode> {
        DropErrorCode::result_from_i8(self.drop_internal(ty, amount))
    }

    /// Enable powers to be used in this room on a [`StructureController`] in
    /// melee range. You do not need to own the controller.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.enableRoom)
    pub fn enable_room(&self, target: &StructureController) -> Result<(), EnableRoomErrorCode> {
        EnableRoomErrorCode::result_from_i8(self.enable_room_internal(target))
    }

    /// Move one square in the specified direction.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.move)
    pub fn move_direction(
        &self,
        direction: Direction,
    ) -> Result<(), PowerCreepMoveDirectionErrorCode> {
        PowerCreepMoveDirectionErrorCode::result_from_i8(self.move_direction_internal(direction))
    }

    /// Move the power creep along a previously determined path returned from a
    /// pathfinding function, in array or serialized string form.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.moveByPath)
    pub fn move_by_path(&self, path: &JsValue) -> Result<(), PowerCreepMoveByPathErrorCode> {
        PowerCreepMoveByPathErrorCode::result_from_i8(self.move_by_path_internal(path))
    }

    /// Whether to send an email notification when this power creep is attacked.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.notifyWhenAttacked)
    pub fn notify_when_attacked(&self, enabled: bool) -> Result<(), NotifyWhenAttackedErrorCode> {
        NotifyWhenAttackedErrorCode::result_from_i8(self.notify_when_attacked_internal(enabled))
    }

    /// Pick up a [`Resource`] in melee range (or at the same position as the
    /// creep).
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.pickup)
    pub fn pickup(&self, target: &Resource) -> Result<(), PickupErrorCode> {
        PickupErrorCode::result_from_i8(self.pickup_internal(target))
    }

    /// Renew the power creep's TTL using a [`StructurePowerSpawn`] or
    /// [`StructurePowerBank`] in melee range.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.renew)
    ///
    /// [`StructurePowerBank`]: crate::objects::StructurePowerBank
    pub fn renew(&self, target: &RoomObject) -> Result<(), RenewErrorCode> {
        RenewErrorCode::result_from_i8(self.renew_internal(target))
    }

    /// Display a string in a bubble above the power creep next tick. 10
    /// character limit.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.say)
    pub fn say(&self, message: &str, public: bool) -> Result<(), SayErrorCode> {
        SayErrorCode::result_from_i8(self.say_internal(message, public))
    }

    /// Immediately kill the power creep.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.suicide)
    pub fn suicide(&self) -> Result<(), SuicideErrorCode> {
        SuicideErrorCode::result_from_i8(self.suicide_internal())
    }

    /// Use one of the power creep's powers.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.usePower)
    pub fn use_power(
        &self,
        power: PowerType,
        target: Option<&RoomObject>,
    ) -> Result<(), UsePowerErrorCode> {
        UsePowerErrorCode::result_from_i8(self.use_power_internal(power, target))
    }

    /// Move the power creep toward the specified goal, either a
    /// [`RoomPosition`] or [`RoomObject`]. Note that using this function
    /// will store data in `Memory.creeps[creep_name]` and enable the
    /// default serialization behavior of the `Memory` object, which may
    /// hamper attempts to directly use `RawMemory`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.moveTo)
    pub fn move_to<T>(&self, target: T) -> Result<(), PowerCreepMoveToErrorCode>
    where
        T: HasPosition,
    {
        let target: RoomPosition = target.pos().into();
        PowerCreepMoveToErrorCode::result_from_i8(
            self.move_to_internal(&target, &JsValue::UNDEFINED),
        )
    }

    /// Move the power creep toward the specified goal, either a
    /// [`RoomPosition`] or [`RoomObject`]. Note that using this function
    /// will store data in `Memory.creeps[creep_name]` and enable the
    /// default serialization behavior of the `Memory` object, which may
    /// hamper attempts to directly use `RawMemory`.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.moveTo)
    pub fn move_to_with_options<T, F>(
        &self,
        target: T,
        options: Option<MoveToOptions<F>>,
    ) -> Result<(), PowerCreepMoveToErrorCode>
    where
        T: HasPosition,
        F: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult,
    {
        let target: RoomPosition = target.pos().into();

        if let Some(options) = options {
            options.into_js_options(|js_options| {
                PowerCreepMoveToErrorCode::result_from_i8(
                    self.move_to_internal(&target, js_options),
                )
            })
        } else {
            PowerCreepMoveToErrorCode::result_from_i8(
                self.move_to_internal(&target, &JsValue::UNDEFINED),
            )
        }
    }
}

impl HasHits for PowerCreep {
    fn hits(&self) -> u32 {
        self.hits()
    }

    fn hits_max(&self) -> u32 {
        self.hits_max()
    }
}

impl HasId for PowerCreep {
    fn js_raw_id(&self) -> JsString {
        self.id_internal()
    }
}

impl HasStore for PowerCreep {
    fn store(&self) -> Store {
        self.store()
    }
}

impl Attackable for PowerCreep {}
impl Healable for PowerCreep {}
impl Transferable for PowerCreep {}

impl SharedCreepProperties for PowerCreep {
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

    fn drop(&self, ty: ResourceType, amount: Option<u32>) -> Result<(), DropErrorCode> {
        self.drop(ty, amount)
    }

    fn notify_when_attacked(&self, enabled: bool) -> Result<(), NotifyWhenAttackedErrorCode> {
        NotifyWhenAttackedErrorCode::result_from_i8(self.notify_when_attacked_internal(enabled))
    }

    fn pickup(&self, target: &Resource) -> Result<(), PickupErrorCode> {
        PickupErrorCode::result_from_i8(self.pickup_internal(target))
    }

    fn say(&self, message: &str, public: bool) -> Result<(), SayErrorCode> {
        SayErrorCode::result_from_i8(self.say_internal(message, public))
    }

    fn suicide(&self) -> Result<(), SuicideErrorCode> {
        SuicideErrorCode::result_from_i8(self.suicide_internal())
    }

    fn transfer<T>(
        &self,
        target: &T,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> Result<(), TransferErrorCode>
    where
        T: Transferable + ?Sized,
    {
        TransferErrorCode::result_from_i8(self.transfer_internal(target.as_ref(), ty, amount))
    }

    fn withdraw<T>(
        &self,
        target: &T,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> Result<(), WithdrawErrorCode>
    where
        T: Withdrawable + ?Sized,
    {
        WithdrawErrorCode::result_from_i8(self.withdraw_internal(target.as_ref(), ty, amount))
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

    #[wasm_bindgen(method, getter = id)]
    fn id_internal(this: &AccountPowerCreep) -> Option<JsString>;

    #[wasm_bindgen(method, getter = className)]
    fn class_internal(this: &AccountPowerCreep) -> PowerCreepClass;

    #[wasm_bindgen(method, getter = deleteTime)]
    fn delete_time_internal(this: &AccountPowerCreep) -> Option<f64>;

    #[wasm_bindgen(method, getter = level)]
    fn level_internal(this: &AccountPowerCreep) -> u32;

    #[wasm_bindgen(method, getter = name)]
    fn name_internal(this: &AccountPowerCreep) -> String;

    #[wasm_bindgen(method, getter = name)]
    fn name_jsstring_internal(this: &AccountPowerCreep) -> JsString;

    #[wasm_bindgen(method, getter = powers)]
    fn powers_internal(this: &AccountPowerCreep) -> Object;

    #[wasm_bindgen(method, getter = shard)]
    fn shard_internal(this: &AccountPowerCreep) -> Option<JsString>;

    #[wasm_bindgen(method, getter = spawnCooldownTime)]
    fn spawn_cooldown_time_internal(this: &AccountPowerCreep) -> Option<f64>;

    #[wasm_bindgen(method, js_name = delete)]
    fn delete_internal(this: &AccountPowerCreep, cancel: bool) -> i8;

    #[wasm_bindgen(method, js_name = rename)]
    fn rename_internal(this: &AccountPowerCreep, name: &JsString) -> i8;

    #[wasm_bindgen(method, js_name = spawn)]
    fn spawn_internal(this: &AccountPowerCreep, target: &StructurePowerSpawn) -> i8;

    #[wasm_bindgen(method, js_name = upgrade)]
    fn upgrade_internal(this: &AccountPowerCreep, power: PowerType) -> i8;
}

impl AccountPowerCreep {
    /// Retrieve this power creep's [`PowerCreepClass`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.className)
    pub fn class(&self) -> PowerCreepClass {
        self.class_internal()
    }

    // todo should be u64 but seems to panic at the moment, follow up
    /// The timestamp, in milliseconds since epoch, when the [`PowerCreep`] will
    /// be permanently deleted due to [`AccountPowerCreep::delete`]. Can be
    /// cancelled with the same function until then.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.deleteTime)
    pub fn delete_time(&self) -> Option<f64> {
        self.delete_time_internal()
    }

    /// Current level of the power creep, which can be increased with
    /// [`AccountPowerCreep::upgrade`] if you have unspent GPL.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.level)
    pub fn level(&self) -> u32 {
        self.level_internal()
    }

    /// The power creep's name as a [`String`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.name)
    pub fn name(&self) -> String {
        self.name_internal()
    }

    /// The power creep's name as a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.name)
    pub fn name_jsstring(&self) -> JsString {
        self.name_jsstring_internal()
    }

    /// The levels of this power creep's abilities, with [`PowerType`] keys and
    /// values containing power level and cooldown.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.powers)
    pub fn powers(&self) -> JsHashMap<PowerType, PowerInfo> {
        self.powers_internal().into()
    }

    /// The shard the power creep is currently spawned on, if spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.shard)
    pub fn shard(&self) -> Option<JsString> {
        self.shard_internal()
    }

    // todo should be u64 but seems to panic at the moment, follow up
    /// The timestamp, in milliseconds since epoch, when the power creep will be
    /// allowed to spawn again after dying.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.spawnCooldownTime)
    pub fn spawn_cooldown_time(&self) -> Option<f64> {
        self.spawn_cooldown_time_internal()
    }

    /// Set a power creep that is not currently spawned to be deleted. Can be
    /// cancelled with `true` for the cancel paramater.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.delete)
    pub fn delete(&self, cancel: bool) -> Result<(), DeleteErrorCode> {
        DeleteErrorCode::result_from_i8(self.delete_internal(cancel))
    }

    /// Change the name of the power creep. Must not be spawned.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.rename)
    pub fn rename(&self, name: &JsString) -> Result<(), RenameErrorCode> {
        RenameErrorCode::result_from_i8(self.rename_internal(name))
    }

    /// Spawn the power creep at a [`StructurePowerSpawn`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.spawn)
    pub fn spawn(&self, target: &StructurePowerSpawn) -> Result<(), SpawnErrorCode> {
        SpawnErrorCode::result_from_i8(self.spawn_internal(target))
    }

    /// Upgrade this power creep, consuming one available GPL and adding a new
    /// level to one of its powers.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#PowerCreep.upgrade)
    pub fn upgrade(&self, power: PowerType) -> Result<(), UpgradeErrorCode> {
        UpgradeErrorCode::result_from_i8(self.upgrade_internal(power))
    }
}

impl JsCollectionFromValue for AccountPowerCreep {
    fn from_value(val: JsValue) -> Self {
        val.unchecked_into()
    }
}

#[derive(Debug, Clone)]
pub struct PowerCreepNotSpawned {}

impl fmt::Display for PowerCreepNotSpawned {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "PowerCreep not currently spawned")
    }
}

impl Error for PowerCreepNotSpawned {}

impl TryFrom<AccountPowerCreep> for PowerCreep {
    type Error = PowerCreepNotSpawned;

    fn try_from(account_power_creep: AccountPowerCreep) -> Result<Self, Self::Error> {
        account_power_creep
            .id_internal()
            .map(|_| account_power_creep.unchecked_into())
            .ok_or(PowerCreepNotSpawned {})
    }
}

impl AsRef<AccountPowerCreep> for PowerCreep {
    fn as_ref(&self) -> &AccountPowerCreep {
        self.unchecked_ref()
    }
}

impl From<PowerCreep> for AccountPowerCreep {
    fn from(power_creep: PowerCreep) -> Self {
        power_creep.unchecked_into()
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
