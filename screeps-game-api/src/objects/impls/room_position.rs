use std::cmp::{Eq, PartialEq};

use stdweb::unstable::TryInto;

use {Color, Direction, FindConstant, HasPosition, LookConstant, ReturnCode, RoomPosition,
     StructureType};

impl RoomPosition {
    pub fn x(&self) -> i32 {
        js_unwrap!(@{&self.0}.x)
    }

    pub fn y(&self) -> i32 {
        js_unwrap!(@{&self.0}.y)
    }

    pub fn room_name(&self) -> String {
        js_unwrap!(@{&self.0}.roomName)
    }

    pub fn create_construction_site(&self, ty: StructureType) -> ReturnCode {
        js_unwrap!(@{&self.0}.createConstructionSite(__structure_type_num_to_str(@{ty as i32})))
    }

    pub fn create_flag(&self, name: &str, main_color: Color, secondary_color: Color) -> ReturnCode {
        // TODO: determine if ERR_NOT_IN_RANGE is the best choice here
        (js! {
            var flag = @{&self.0};
            if (flag.roomName in Game.rooms) {
                return flag.createFlag(@{name}, @{main_color as i32}, @{secondary_color as i32});
            } else {
                return ERR_NOT_IN_RANGE;
            }
        }).try_into()
            .expect("expected Flag.createFlag to return ReturnCode")
    }

    pub fn find_closest_by_range<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_array!(@{&self.0}.findClosestByRange(
            __structure_type_num_to_str(@{ty.find_code()}
        )))
    }

    pub fn find_in_range<T>(&self, ty: T, range: i32) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_array!(@{&self.0}.findInRange(
            __structure_type_num_to_str(@{ty.find_code()}),
            @{range}
        ))
    }

    pub fn get_direction_to<T>(&self, target: &T) -> Direction
    where
        T: HasPosition,
    {
        js_unwrap!(@{&self.0}.getDirectionTo(@{&target.pos().0}))
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

    pub fn is_equal_to<T>(&self, target: &T) -> bool
    where
        T: HasPosition,
    {
        js_unwrap!(@{&self.0}.isEqualTo(@{&target.pos().0}))
    }

    pub fn is_near_to<T>(&self, target: &T) -> bool
    where
        T: HasPosition,
    {
        js_unwrap!(@{&self.0}.isNearTo(@{&target.pos().0}))
    }

    pub fn look_for<T, U>(&self, ty: T) -> Vec<T::Item>
    where
        T: LookConstant,
        U: HasPosition,
    {
        js_unwrap_array!(@{&self.0}.lookFor(__look_num_to_str(@{ty.look_code() as i32})))
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
