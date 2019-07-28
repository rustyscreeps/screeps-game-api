use serde::Deserialize;
use stdweb::Value;

use crate::{local::LocalRoomPosition, macros::*, objects::StructurePortal, traits::TryInto};

#[derive(Deserialize, Debug)]
pub struct InterShardPortalDestination {
    shard: String,
    room: String,
}
js_deserializable!(InterShardPortalDestination);

pub enum PortalDestination {
    InterRoom(LocalRoomPosition),
    InterShard(InterShardPortalDestination),
}

impl StructurePortal {
    pub fn destination(&self) -> PortalDestination {
        let v = js! {
            let destination = @{self.as_ref()}.destination;
            if (destination instanceof RoomPosition) {
                return destination.__packedPos;
            } else {
                return destination;
            }
        };

        match v {
            Value::Number(_) => PortalDestination::InterRoom(
                v.try_into()
                    .expect("expected LocalRoomPosition::try_from(pos.__packedPos) to succeed"),
            ),
            _ => PortalDestination::InterShard(
                v.try_into()
                    .expect("Value couldn't be converted into an InterShardPortalDestination"),
            ),
        }
    }
}
