use stdweb::unstable::TryInto;

use {
    {RoomPosition, StructurePortal},
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

        let is_inter_room: bool = js_unwrap!{
            @{self.as_ref()}.destination instanceof RoomPosition
        };

        if is_inter_room {
            PortalDestination::InterRoom(v.try_into().unwrap())
        } else {
            PortalDestination::InterShard(v.try_into().unwrap())
        }
    }
}
