use js_sys::JsString;
use wasm_bindgen::prelude::*;

use crate::{
    local::{Position, RoomName},
    objects::{RoomObject, RoomPosition, Structure},
    prelude::*,
};

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePortal`], which allows movement
    /// between remote locations or other shards.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePortal)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    #[derive(Clone, Debug)]
    pub type StructurePortal;

    #[wasm_bindgen(method, getter = destination)]
    fn destination_internal(this: &StructurePortal) -> JsValue;

    /// The number of ticks until the portal will decay, if it's unstable, or 0
    /// if it's stable.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePortal.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructurePortal) -> u32;
}

impl StructurePortal {
    pub fn destination(&self) -> PortalDestination {
        let dest = Self::destination_internal(self);
        match dest.dyn_ref::<RoomPosition>() {
            Some(room_pos) => PortalDestination::InterRoom(room_pos.into()),
            None => PortalDestination::InterShard(dest.unchecked_into()),
        }
    }
}

impl CanDecay for StructurePortal {
    fn ticks_to_decay(&self) -> u32 {
        Self::ticks_to_decay(self)
    }
}

pub enum PortalDestination {
    InterRoom(Position),
    InterShard(InterShardPortalDestination),
}

#[wasm_bindgen]
extern "C" {
    /// An object which contains the destination shard and room of an
    /// inter-shard portal.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePortal.destination)
    #[wasm_bindgen]
    pub type InterShardPortalDestination;

    #[wasm_bindgen(method, getter = room)]
    fn room_internal(this: &InterShardPortalDestination) -> JsString;

    #[wasm_bindgen(method, getter)]
    pub fn shard(this: &InterShardPortalDestination) -> String;
}

impl InterShardPortalDestination {
    pub fn room(&self) -> RoomName {
        Self::room_internal(self)
            .try_into()
            .expect("expected parseable room name")
    }
}
