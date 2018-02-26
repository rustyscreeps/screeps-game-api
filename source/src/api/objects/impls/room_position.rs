use std::cmp::{Eq, PartialEq};

use stdweb::unstable::TryInto;

use api::objects::RoomPosition;
use api::HasPosition;

impl RoomPosition {
    pub fn is_near_to<T>(&self, target: &T) -> bool
    where
        T: HasPosition,
    {
        js_unwrap!(@{&self.0}.isNearTo(@{&target.pos().0}))
    }

    // TODO: StructureType so we can do createConstructionSite
    pub fn x(&self) -> i32 {
        js_unwrap!(@{&self.0}.x)
    }

    pub fn y(&self) -> i32 {
        js_unwrap!(@{&self.0}.y)
    }

    pub fn room_name(&self) -> String {
        js_unwrap!(@{&self.0}.roomName)
    }

    pub fn get_range_to<T>(&self, target: &T) -> i32
    where
        T: HasPosition,
    {
        js_unwrap!(@{&self.0}.getRangeTo(@{&target.pos().0}))
    }

    pub fn in_range_to<T>(&self, target: &T, range: i32) -> bool
    where
        T: HasPosition,
    {
        js_unwrap!(@{&self.0}.inRangeTo(@{&target.pos().0}, @{range}))
    }
}

impl<T: HasPosition> PartialEq<T> for RoomPosition {
    fn eq(&self, other: &T) -> bool {
        (js!{
            var a = @{&self.0};
            var b = @{&other.pos().0};
            return a.x == b.x && a.y == b.y && a.roomName == b.roomName;
        }).try_into()
            .expect("expected a boolean to be a boolean")
    }
}

impl Eq for RoomPosition {}
