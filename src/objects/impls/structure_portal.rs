use crate::objects::{RoomObject, Structure};
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    /// An object representing a [`StructurePortal`], which allows movement
    /// between remote locations or other shards.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePortal)
    #[wasm_bindgen(extends = RoomObject, extends = Structure)]
    pub type StructurePortal;

    // todo: destination

    /// The number of ticks until the portal will decay, if it's unstable.
    ///
    /// [Screeps documentation](https://docs.screeps.com/api/#StructurePortal.ticksToDecay)
    #[wasm_bindgen(method, getter = ticksToDecay)]
    pub fn ticks_to_decay(this: &StructurePortal) -> Option<u32>;
}

// use serde::Deserialize;
// use stdweb::Value;

// use crate::{
//     local::{Position, RoomName},
//     objects::StructurePortal,
//     traits::TryInto,
// };

// #[derive(Deserialize, Debug)]
// pub struct InterShardPortalDestination {
//     shard: String,
//     room: RoomName,
// }
// js_deserializable!(InterShardPortalDestination);

// pub enum PortalDestination {
//     InterRoom(Position),
//     InterShard(InterShardPortalDestination),
// }

// impl StructurePortal {
//     pub fn destination(&self) -> PortalDestination {
//         let v = js! {
//             let destination = @{self.as_ref()}.destination;
//             if (destination instanceof Position) {
//                 return destination.__packedPos;
//             } else {
//                 return destination;
//             }
//         };

//         match v {
//             Value::Number(_) => PortalDestination::InterRoom(
//                 v.try_into()
//                     .expect("expected Position::try_from(pos.__packedPos) to succeed"),
//             ),
//             _ => PortalDestination::InterShard(
//                 v.try_into()
//                     .expect("Value couldn't be converted into an InterShardPortalDestination"),
//             ),
//         }
//     }
// }
