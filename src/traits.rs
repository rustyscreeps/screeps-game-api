//! Traits associated with how specific [game objects] can be used.
//!
//! [game objects]: crate::objects
use std::{borrow::Borrow, str::FromStr};

use enum_dispatch::enum_dispatch;
use js_sys::{Array, JsString};
use wasm_bindgen::prelude::*;

use crate::{
    constants::*,
    enums::*,
    local::{ObjectId, Position, RawObjectId, RoomName},
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

pub trait HasNativeId {
    fn native_id(&self) -> JsString;
}

pub trait MaybeHasNativeId {
    fn try_native_id(&self) -> Option<JsString>;
}

impl<T> MaybeHasNativeId for T
where
    T: HasNativeId,
{
    fn try_native_id(&self) -> Option<JsString> {
        Some(<Self as HasNativeId>::native_id(self))
    }
}

pub trait Resolvable: From<JsValue> {}

impl<T> Resolvable for T where T: MaybeHasTypedId<T> + From<JsValue> {}

#[enum_dispatch]
pub trait HasId {
    /// Object ID of the object, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    fn raw_id(&self) -> RawObjectId;
}

impl<T> HasId for T
where
    T: HasNativeId,
{
    fn raw_id(&self) -> RawObjectId {
        let id: String = self.native_id().into();

        RawObjectId::from_str(&id).expect("expected object ID to be parseable")
    }
}

#[enum_dispatch]
pub trait HasTypedId<T> {
    /// Object ID of the object, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks.
    fn id(&self) -> ObjectId<T>;

    fn js_id(&self) -> JsObjectId<T>;
}

impl<T> HasTypedId<T> for T
where
    T: HasId + HasNativeId,
{
    fn id(&self) -> ObjectId<T> {
        self.raw_id().into()
    }

    fn js_id(&self) -> JsObjectId<T> {
        self.native_id().into()
    }
}

impl<T> HasTypedId<T> for &T
where
    T: HasId + HasNativeId,
{
    fn id(&self) -> ObjectId<T> {
        self.raw_id().into()
    }

    fn js_id(&self) -> JsObjectId<T> {
        self.native_id().into()
    }
}

#[enum_dispatch]
pub trait MaybeHasId {
    /// Object ID of the object, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks, or `None` if the
    /// object doesn't currently have an id.
    fn try_raw_id(&self) -> Option<RawObjectId>;
}

impl<T> MaybeHasId for T
where
    T: MaybeHasNativeId,
{
    fn try_raw_id(&self) -> Option<RawObjectId> {
        self.try_native_id()
            .map(String::from)
            .map(|id| RawObjectId::from_str(&id).expect("expected object ID to be parseable"))
    }
}

#[enum_dispatch]
pub trait MaybeHasTypedId<T> {
    /// Object ID of the object, which can be used to efficiently fetch a
    /// fresh reference to the object on subsequent ticks, or `None` if the
    /// object doesn't currently have an id.
    fn try_id(&self) -> Option<ObjectId<T>>;
}

impl<T> MaybeHasTypedId<T> for T
where
    T: MaybeHasId,
{
    fn try_id(&self) -> Option<ObjectId<T>> {
        self.try_raw_id().map(Into::into)
    }
}

impl<T> MaybeHasTypedId<T> for &T
where
    T: MaybeHasId,
{
    fn try_id(&self) -> Option<ObjectId<T>> {
        self.try_raw_id().map(Into::into)
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

pub trait HasLocalPosition {
    fn x(&self) -> u8;
    fn y(&self) -> u8;
}

pub trait CostMatrixSet {
    fn set<P, V>(&mut self, position: P, cost: V)
    where
        P: HasLocalPosition,
        V: Borrow<u8>;

    fn set_multi<D, B, P, V>(&mut self, data: D)
    where
        D: IntoIterator<Item = B>,
        B: Borrow<(P, V)>,
        P: HasLocalPosition,
        V: Borrow<u8>;
}

#[inline]
const fn pos_as_idx(x: u8, y: u8) -> usize {
    (x as usize) * ROOM_SIZE as usize + (y as usize)
}

impl CostMatrixSet for CostMatrix {
    fn set<P, V>(&mut self, position: P, cost: V)
    where
        P: HasLocalPosition,
        V: Borrow<u8>,
    {
        CostMatrix::set(self, position.x(), position.y(), *cost.borrow());
    }

    fn set_multi<D, B, P, V>(&mut self, data: D)
    where
        D: IntoIterator<Item = B>,
        B: Borrow<(P, V)>,
        P: HasLocalPosition,
        V: Borrow<u8>,
    {
        let matrix_buffer = self.get_bits();

        for entry in data.into_iter() {
            let (pos, cost) = entry.borrow();

            let offset = pos_as_idx(pos.x(), pos.y());

            matrix_buffer.set_index(offset as u32, *cost.borrow());
        }
    }
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

    /// The creep's name as an owned reference to a [`String`].
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#Creep.name)
    fn name(&self) -> String;

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
/// target of `Creep.heal`.
///
/// # Contracts
///
/// The reference returned from `AsRef<RoomObject>::as_ref` must be a valid
/// target for `Creep.heal`.
pub trait Healable: AsRef<RoomObject> {}

// NOTE: keep impls for Structure* in sync with accessor methods in
// src/objects/structure.rs

impl Transferable for StructureExtension {}
impl Transferable for Creep {}
impl Transferable for StructureContainer {}
impl Transferable for StructureFactory {}
impl Transferable for StructureLab {}
impl Transferable for StructureLink {}
impl Transferable for StructureNuker {}
impl Transferable for StructureSpawn {}
impl Transferable for StructureStorage {}
impl Transferable for StructureTower {}
impl Transferable for StructurePowerSpawn {}
impl Transferable for StructureTerminal {}
impl Transferable for PowerCreep {}
#[cfg(feature = "score")]
impl Transferable for ScoreCollector {}
#[cfg(feature = "symbols")]
impl Transferable for SymbolDecoder {}
#[cfg(feature = "thorium")]
impl Transferable for Reactor {}

// NOTE: keep impls for Structure* in sync with accessor methods in
// src/objects/structure.rs

impl Withdrawable for Ruin {}
impl Withdrawable for StructureExtension {}
impl Withdrawable for StructureContainer {}
impl Withdrawable for StructureFactory {}
impl Withdrawable for StructureLab {}
impl Withdrawable for StructureLink {}
impl Withdrawable for StructureSpawn {}
impl Withdrawable for StructureStorage {}
impl Withdrawable for StructureTower {}
impl Withdrawable for StructurePowerSpawn {}
impl Withdrawable for StructureTerminal {}
impl Withdrawable for Tombstone {}
#[cfg(feature = "score")]
impl Withdrawable for ScoreContainer {}
#[cfg(feature = "symbols")]
impl Withdrawable for SymbolContainer {}

impl Harvestable for Deposit {}
impl Harvestable for Mineral {}
impl Harvestable for Source {}

// NOTE: keep impls for Structure* in sync with accessor methods in
// src/objects/structure.rs

impl Attackable for Creep {}
impl Attackable for OwnedStructure {}
impl Attackable for StructureContainer {}
impl Attackable for StructureExtension {}
impl Attackable for StructureExtractor {}
impl Attackable for StructureFactory {}
impl Attackable for StructureInvaderCore {}
impl Attackable for StructureKeeperLair {}
impl Attackable for StructureLab {}
impl Attackable for StructureLink {}
impl Attackable for StructureNuker {}
impl Attackable for StructureObserver {}
impl Attackable for StructurePowerBank {}
impl Attackable for StructurePowerSpawn {}
impl Attackable for StructureRampart {}
impl Attackable for StructureRoad {}
impl Attackable for StructureSpawn {}
impl Attackable for StructureStorage {}
impl Attackable for StructureTerminal {}
impl Attackable for StructureTower {}
impl Attackable for StructureWall {}
impl Attackable for PowerCreep {}

// NOTE: keep impls for Structure* in sync with accessor methods in
// src/objects/structure.rs

impl Dismantleable for StructureContainer {}
impl Dismantleable for StructureExtension {}
impl Dismantleable for StructureExtractor {}
impl Dismantleable for StructureFactory {}
impl Dismantleable for StructureLab {}
impl Dismantleable for StructureLink {}
impl Dismantleable for StructureNuker {}
impl Dismantleable for StructureObserver {}
impl Dismantleable for StructurePowerBank {}
impl Dismantleable for StructurePowerSpawn {}
impl Dismantleable for StructureRampart {}
impl Dismantleable for StructureRoad {}
impl Dismantleable for StructureSpawn {}
impl Dismantleable for StructureStorage {}
impl Dismantleable for StructureTerminal {}
impl Dismantleable for StructureTower {}
impl Dismantleable for StructureWall {}

impl Healable for Creep {}
impl Healable for PowerCreep {}

//TODO: wiarchbe: Add Repairable trait.
