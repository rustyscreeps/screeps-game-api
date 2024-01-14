//! Traits associated with how specific [game objects] can be used.
//!
//! [game objects]: crate::objects
use std::str::FromStr;

use enum_dispatch::enum_dispatch;
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

use crate::{
    constants::*,
    enums::*,
    local::{ObjectId, Position, RawObjectId, RoomName, RoomXY},
    objects::*,
    pathfinder::SingleRoomCostResult,
    prelude::*,
};

pub trait FromReturnCode {
    type Error;

    fn result_from_i8(val: i8) -> Result<(), Self::Error>;

    fn try_result_from_i8(val: i8) -> Option<Result<(), Self::Error>>;

    fn try_result_from_jsvalue(val: &JsValue) -> Option<Result<(), Self::Error>> {
        val.as_f64().and_then(|f| Self::try_result_from_i8(f as i8))
    }
}

#[enum_dispatch]
pub trait HasHits {
    /// Retrieve the current hits of this object.
    fn hits(&self) -> u32;

    /// Retrieve the maximum hits of this object.
    fn hits_max(&self) -> u32;
}

#[enum_dispatch]
pub trait CanDecay {
    /// The number of ticks until the object will decay, losing hits.
    fn ticks_to_decay(&self) -> u32;
}

#[enum_dispatch]
pub trait HasCooldown {
    /// The number of ticks until the object can be used again.
    fn cooldown(&self) -> u32;
}

/// Trait for all game objects which have an associated unique identifier.
pub trait HasId: MaybeHasId {
    /// Object ID of the object stored in Rust memory, which can be used to
    /// efficiently fetch a fresh reference to the object on subsequent
    /// ticks.
    fn id(&self) -> ObjectId<Self>
    where
        Self: Sized,
    {
        self.raw_id().into()
    }

    /// Object ID of the object stored in Rust memory, without its associated
    /// type information.
    fn raw_id(&self) -> RawObjectId {
        let id: String = self.js_raw_id().into();

        RawObjectId::from_str(&id).expect("expected object ID to be parseable")
    }

    /// Object ID of the object stored in JavaScript memory, which can be used
    /// to efficiently fetch a fresh reference to the object on subsequent
    /// ticks.
    fn js_id(&self) -> JsObjectId<Self>
    where
        Self: Sized,
    {
        self.js_raw_id().into()
    }

    /// Object ID of the object stored in JavaScript memory, without its
    /// associated type information.
    fn js_raw_id(&self) -> JsString;
}

/// Trait for all game objects which may (or may not) have an associated unique
/// identifier.
pub trait MaybeHasId {
    /// Object ID of the object, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks, or `None` if the
    /// object doesn't currently have an ID.
    fn try_id(&self) -> Option<ObjectId<Self>>
    where
        Self: Sized,
    {
        self.try_raw_id().map(Into::into)
    }

    /// Object ID of the object, without its associated type information, or
    /// `None` if the object doesn't currently have an ID.
    fn try_raw_id(&self) -> Option<RawObjectId> {
        self.try_js_raw_id()
            .map(String::from)
            .map(|id| RawObjectId::from_str(&id).expect("expected object ID to be parseable"))
    }

    /// Object ID of the object stored in JavaScript memory, which can be used
    /// to efficiently fetch a fresh reference to the object on subsequent
    /// ticks, or `None` if the object doesn't currently have an ID.
    fn try_js_id(&self) -> Option<JsObjectId<Self>>
    where
        Self: Sized,
    {
        self.try_js_raw_id().map(Into::into)
    }

    /// Object ID of the object stored in JavaScript memory, without its
    /// associated type information, or `None` if the object doesn't currently
    /// have an ID.
    fn try_js_raw_id(&self) -> Option<JsString>;
}

impl<T> MaybeHasId for T
where
    T: HasId,
{
    fn try_js_raw_id(&self) -> Option<JsString> {
        Some(self.js_raw_id())
    }
}

#[enum_dispatch]
pub trait HasPosition {
    /// Position of the object.
    fn pos(&self) -> Position;
}

#[enum_dispatch]
pub trait MaybeHasPosition {
    /// Position of the object, or `None` if the object is a power creep not
    /// spawned on the current shard.
    fn try_pos(&self) -> Option<Position>;
}

pub trait CostMatrixSet {
    fn set_xy(&mut self, xy: RoomXY, cost: u8);
}

pub trait CostMatrixGet {
    fn get_xy(&mut self, xy: RoomXY) -> u8;
}

#[enum_dispatch]
pub trait HasStore {
    /// The store of the object, containing information about the resources it
    /// is holding.
    fn store(&self) -> Store;
}

#[enum_dispatch]
pub trait OwnedStructureProperties {
    /// Whether this structure is owned by the player.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.my)
    fn my(&self) -> bool;

    /// The [`Owner`] of this structure that contains the owner's username, or
    /// `None` if it's an ownable structure currently not under a player's
    /// control.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#OwnedStructure.owner)
    fn owner(&self) -> Option<Owner>;
}

#[enum_dispatch]
pub trait RoomObjectProperties {
    /// Effects applied to the object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.effects)
    fn effects(&self) -> Vec<Effect>;

    /// Effects applied to the object.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.effects)
    fn effects_raw(&self) -> Option<Array>;

    /// A link to the room that the object is currently in, or `None` if the
    /// object is a power creep not spawned on the current shard, or a flag or
    /// construction site not in a visible room.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#RoomObject.room)
    fn room(&self) -> Option<Room>;
}

#[enum_dispatch]
pub trait SharedCreepProperties {
    /// A shortcut to the part of the `Memory` tree used for this creep by
    /// default
    fn memory(&self) -> JsValue;

    /// Sets a new value to the memory object shortcut for this creep.
    fn set_memory(&self, val: &JsValue);

    /// Whether this creep is owned by the player.
    fn my(&self) -> bool;

    /// The creep's name as a [`String`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.name)
    fn name(&self) -> String;

    /// The creep's name as a [`JsString`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.name)
    fn name_jsstring(&self) -> JsString;

    /// The [`Owner`] of this creep that contains the owner's username.
    fn owner(&self) -> Owner;

    /// What the creep said last tick.
    fn saying(&self) -> Option<JsString>;

    /// The number of ticks the creep has left to live.
    fn ticks_to_live(&self) -> Option<u32>;

    /// Cancel an a successfully called creep function from earlier in the tick,
    /// with a [`JsString`] that must contain the JS version of the function
    /// name.
    fn cancel_order(&self, target: &JsString) -> Result<(), ErrorCode>;

    /// Drop a resource on the ground from the creep's [`Store`].
    fn drop(&self, ty: ResourceType, amount: Option<u32>) -> Result<(), ErrorCode>;

    /// Move one square in the specified direction.
    fn move_direction(&self, direction: Direction) -> Result<(), ErrorCode>;

    /// Move the creep along a previously determined path returned from a
    /// pathfinding function, in array or serialized string form.
    fn move_by_path(&self, path: &JsValue) -> Result<(), ErrorCode>;

    /// Move the creep toward the specified goal, either a [`RoomPosition`] or
    /// [`RoomObject`]. Note that using this function will store data in
    /// `Memory.creeps[creep_name]` and enable the default serialization
    /// behavior of the `Memory` object, which may hamper attempts to directly
    /// use `RawMemory`.
    fn move_to<T>(&self, target: T) -> Result<(), ErrorCode>
    where
        T: HasPosition;

    /// Move the creep toward the specified goal, either a [`RoomPosition`] or
    /// [`RoomObject`]. Note that using this function will store data in
    /// `Memory.creeps[creep_name]` and enable the default serialization
    /// behavior of the `Memory` object, which may hamper attempts to directly
    /// use `RawMemory`.
    fn move_to_with_options<T, F>(
        &self,
        target: T,
        options: Option<MoveToOptions<F>>,
    ) -> Result<(), ErrorCode>
    where
        T: HasPosition,
        F: FnMut(RoomName, CostMatrix) -> SingleRoomCostResult;

    /// Whether to send an email notification when this creep is attacked.
    fn notify_when_attacked(&self, enabled: bool) -> Result<(), ErrorCode>;

    /// Pick up a [`Resource`] in melee range (or at the same position as the
    /// creep).
    fn pickup(&self, target: &Resource) -> Result<(), ErrorCode>;

    /// Display a string in a bubble above the creep next tick. 10 character
    /// limit.
    fn say(&self, message: &str, public: bool) -> Result<(), ErrorCode>;

    /// Immediately kill the creep.
    fn suicide(&self) -> Result<(), ErrorCode>;

    /// Transfer a resource from the creep's store to [`Structure`],
    /// [`PowerCreep`], or another [`Creep`].
    fn transfer<T>(
        &self,
        target: &T,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> Result<(), ErrorCode>
    where
        T: Transferable + ?Sized;

    /// Withdraw a resource from a [`Structure`], [`Tombstone`], or [`Ruin`].
    fn withdraw<T>(
        &self,
        target: &T,
        ty: ResourceType,
        amount: Option<u32>,
    ) -> Result<(), ErrorCode>
    where
        T: Withdrawable + ?Sized;
}

#[enum_dispatch]
pub trait StructureProperties {
    fn structure_type(&self) -> StructureType;

    fn destroy(&self) -> Result<(), ErrorCode>;

    fn is_active(&self) -> bool;

    fn notify_when_attacked(&self, val: bool) -> Result<(), ErrorCode>;
}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.transfer`.
///
/// # Contracts
///
/// The reference returned from `AsRef<RoomObject>::as_ref` must be a valid
/// target for `Creep.transfer`.
#[enum_dispatch]
pub trait Transferable: AsRef<RoomObject> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.withdraw`.
///
/// # Contracts
///
/// The reference returned from `AsRef<RoomObject>::as_ref` must be a valid
/// target for `Creep.withdraw`.
pub trait Withdrawable: AsRef<RoomObject> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.harvest`.
///
/// # Contracts
///
/// The reference returned from `AsRef<RoomObject>::as_ref` must be a valid
/// target for `Creep.harvest`.
pub trait Harvestable: AsRef<RoomObject> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.attack`.
///
/// # Contracts
///
/// The reference returned from `AsRef<RoomObject>::as_ref` must be a valid
/// target for `Creep.attack`.
pub trait Attackable: HasHits + AsRef<RoomObject> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.dismantle`.
///
/// # Contracts
///
/// The reference returned from `AsRef<Structure>::as_ref` must be a valid
/// target for `Creep.dismantle`.
pub trait Dismantleable: HasHits + AsRef<Structure> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.repair` or `StructureTower.repair`.
///
/// # Contracts
///
/// The reference returned from `AsRef<Structure>::as_ref` must be a valid
/// target for repair.
pub trait Repairable: HasHits + AsRef<Structure> {}

/// Trait for all wrappers over Screeps JavaScript objects which can be the
/// target of `Creep.heal`.
///
/// # Contracts
///
/// The reference returned from `AsRef<RoomObject>::as_ref` must be a valid
/// target for `Creep.heal`.
pub trait Healable: AsRef<RoomObject> {}
