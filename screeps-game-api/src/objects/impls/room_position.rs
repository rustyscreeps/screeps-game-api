use std::cmp::{Eq, PartialEq};

use {
    constants::{Color, Direction, FindConstant, LookConstant, ReturnCode},
    game,
    objects::{Flag, HasPosition, LookResult, RoomPosition, StructureType},
    pathfinder::CostMatrix,
    positions::LocalRoomPosition,
    traits::TryInto,
};

use super::room::{FindOptions, Path};

impl RoomPosition {
    pub fn new(x: u32, y: u32, room_name: &str) -> Self {
        js_unwrap!(new RoomPosition(@{x}, @{y}, @{room_name}))
    }

    pub fn x(&self) -> u32 {
        js_unwrap!(@{self.as_ref()}.x)
    }

    pub fn y(&self) -> u32 {
        js_unwrap!(@{self.as_ref()}.y)
    }

    pub fn coords(&self) -> (u32, u32) {
        (self.x(), self.y())
    }

    pub fn coords_signed(&self) -> (i32, i32) {
        (self.x() as i32, self.y() as i32)
    }

    pub fn room_name(&self) -> String {
        js_unwrap!(@{self.as_ref()}.roomName)
    }

    pub fn local(&self) -> LocalRoomPosition {
        LocalRoomPosition {
            room_name: js_unwrap!(@{self.as_ref()}.roomName),
            x: self.x(),
            y: self.y(),
        }
    }

    pub fn create_construction_site(&self, ty: StructureType) -> ReturnCode {
        js_unwrap!(
            @{self.as_ref()}.createConstructionSite(__structure_type_num_to_str(@{ty as u32}))
        )
    }

    pub fn create_named_construction_site(&self, ty: StructureType, name: &str) -> ReturnCode {
        js_unwrap!(
            @{self.as_ref()}.createConstructionSite(__structure_type_num_to_str(@{ty as u32}),
                                                    @{name})
        )
    }

    pub fn create_flag(
        &self,
        name: &str,
        main_color: Color,
        secondary_color: Color,
    ) -> Result<String, ReturnCode> {
        // TODO: determine if ERR_NOT_IN_RANGE is the best choice here
        //
        // JavaScript code simply throws an error on unknown rooms, which isn't ideal.
        Flag::interpret_creation_ret_value(js! {
            var pos = @{self.as_ref()};
            if (pos.roomName in Game.rooms) {
                return pos.createFlag(@{name}, @{main_color as u32}, @{secondary_color as u32});
            } else {
                return ERR_NOT_IN_RANGE;
            }
        })
        .expect("expected RoomPosition.createFlag to return ReturnCode or String name")
    }

    pub fn find_closest_by_range<T>(&self, ty: T) -> Option<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(@{self.as_ref()}.findClosestByRange(@{ty.find_code()}))
    }

    pub fn find_in_range<T>(&self, ty: T, range: u32) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(@{self.as_ref()}.findInRange(@{ty.find_code()}, @{range}))
    }

    pub fn find_path_to<'a, F, T>(&self, target: &T, opts: FindOptions<'a, F>) -> Path
    where
        F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>> + 'a,
        T: ?Sized + HasPosition,
    {
        let self_room = game::rooms::get(&self.room_name()).unwrap();
        self_room.find_path(self, target, opts)
    }

    pub fn find_path_to_xy<'a, F>(&self, x: u32, y: u32, opts: FindOptions<'a, F>) -> Path
    where
        F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>> + 'a,
    {
        let target = RoomPosition::new(x, y, &self.room_name());
        self.find_path_to(&target, opts)
    }

    pub fn get_direction_to<T>(&self, target: &T) -> Direction
    where
        T: HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.getDirectionTo(@{&target.pos().0}))
    }

    pub fn get_range_to<T>(&self, target: &T) -> u32
    where
        T: HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.getRangeTo(@{&target.pos().0}))
    }

    pub fn in_range_to<T>(&self, target: &T, range: u32) -> bool
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.inRangeTo(@{&target.pos().0}, @{range}))
    }

    pub fn is_equal_to<T>(&self, target: &T) -> bool
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.isEqualTo(@{&target.pos().0}))
    }

    pub fn is_equal_to_xy(&self, x: u32, y: u32) -> bool {
        js_unwrap! {return @{self.as_ref()}.isEqualTo(@{x}, @{y});}
    }

    pub fn is_near_to<T>(&self, target: &T) -> bool
    where
        T: ?Sized + HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.isNearTo(@{&target.pos().0}))
    }

    pub fn look(&self) -> Vec<LookResult> {
        js_unwrap!(@{self.as_ref()}.look())
    }

    pub fn look_for<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        T::convert_and_check_items(js_unwrap! {
            @{self.as_ref()}.lookFor(__look_num_to_str(@{ty.look_code() as u32}))
        })
    }
}

impl<T: HasPosition> PartialEq<T> for RoomPosition {
    fn eq(&self, other: &T) -> bool {
        (js! {
            var a = @{self.as_ref()};
            var b = @{&other.pos().0};
            return a.x == b.x && a.y == b.y && a.roomName == b.roomName;
        })
        .try_into()
        .expect("expected a boolean to be a boolean")
    }
}

impl Eq for RoomPosition {}

impl Into<(u8, u8)> for RoomPosition {
    fn into(self) -> (u8, u8) {
        (self.x() as u8, self.y() as u8)
    }
}

impl Into<(u16, u16)> for RoomPosition {
    fn into(self) -> (u16, u16) {
        (self.x() as u16, self.y() as u16)
    }
}

impl Into<(u32, u32)> for RoomPosition {
    fn into(self) -> (u32, u32) {
        (self.x(), self.y())
    }
}

impl Into<(u64, u64)> for RoomPosition {
    fn into(self) -> (u64, u64) {
        (self.x() as u64, self.y() as u64)
    }
}

impl Into<(i8, i8)> for RoomPosition {
    fn into(self) -> (i8, i8) {
        (self.x() as i8, self.y() as i8)
    }
}

impl Into<(i16, i16)> for RoomPosition {
    fn into(self) -> (i16, i16) {
        (self.x() as i16, self.y() as i16)
    }
}

impl Into<(i32, i32)> for RoomPosition {
    fn into(self) -> (i32, i32) {
        (self.x() as i32, self.y() as i32)
    }
}

impl Into<(i64, i64)> for RoomPosition {
    fn into(self) -> (i64, i64) {
        (self.x() as i64, self.y() as i64)
    }
}
