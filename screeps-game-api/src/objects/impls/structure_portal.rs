use crate::{
    objects::{RoomPosition, StructurePortal},
    traits::TryInto,
};

#[derive(Deserialize, Debug)]
pub struct InterShardPortalDestination {
    shard: String,
    room: String,
}
js_deserializable!(InterShardPortalDestination);

pub enum PortalDestination {
    InterRoom(RoomPosition),
    InterShard(InterShardPortalDestination),
}

impl StructurePortal {
    pub fn destination(&self) -> PortalDestination {
        let v = js! {return @{self.as_ref()}.destination;};

        let is_inter_room: bool = js_unwrap! {
            @{&v} instanceof RoomPosition
        };

        if is_inter_room {
            PortalDestination::InterRoom(v.try_into().expect(
                "The inter room portal destination couldn't be converted to a RoomPosition",
            ))
        } else {
            PortalDestination::InterShard(
                v.try_into()
                    .expect("Value couldn't be converted into an InterShardPortalDestination"),
            )
        }
    }
}
