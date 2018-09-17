use std::cmp::{
    Eq, 
    PartialEq
};

use stdweb::unstable::TryInto;

use {
    constants::{
        Color, 
        Direction, 
        FindConstant,
        LookConstant, 
        ReturnCode,
    },
    game,
    objects::{
        FindOptions,
        HasPosition, 
        Path,
        RoomPosition,
        StructureType,
    },
    pathfinder::CostMatrix,
    positions::LocalRoomPosition,
};

impl RoomPosition {
    pub fn new(x: u8, y: u8, room_name: &str) -> Self {
        js_unwrap!(new RoomPosition(@{x}, @{y}, @{room_name}))
    }

    pub fn x(&self) -> u8 {
        js_unwrap!(@{self.as_ref()}.x)
    }

    pub fn y(&self) -> u8 {
        js_unwrap!(@{self.as_ref()}.y)
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
            @{self.as_ref()}.createConstructionSite(__structure_type_num_to_str(@{ty as i32}))
        )
    }

    pub fn create_flag(&self, name: &str, main_color: Color, secondary_color: Color) -> ReturnCode {
        // TODO: determine if ERR_NOT_IN_RANGE is the best choice here
        (js! {
            var flag = @{self.as_ref()};
            if (flag.roomName in Game.rooms) {
                return flag.createFlag(@{name}, @{main_color as i32}, @{secondary_color as i32});
            } else {
                return ERR_NOT_IN_RANGE;
            }
        }).try_into()
            .expect("expected Flag.createFlag to return ReturnCode")
    }

    pub fn find_closest_by_range<T>(&self, ty: T) -> Option<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(@{self.as_ref()}.findClosestByRange(
            __structure_type_num_to_str(@{ty.find_code()}))
        )
    }

    pub fn find_in_range<T>(&self, ty: T, range: i32) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(@{self.as_ref()}.findInRange(
            __structure_type_num_to_str(@{ty.find_code()}),
            @{range}
        ))
    }

    pub fn find_path_to<'a, F, T>(&self, target: &T, opts: FindOptions<'a, F>) -> Path 
    where
        F: Fn(String, CostMatrix) -> Option<CostMatrix<'a>> + 'a,
        T: HasPosition,
    {
        let self_room = game::rooms::get(&self.room_name()).unwrap();
        self_room.find_path(self, target, opts)
    }

    pub fn find_path_to_xy<'a, F>(&self, x: u8, y: u8, opts: FindOptions<'a, F>) -> Path 
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

    pub fn get_range_to<T>(&self, target: &T) -> i32
    where
        T: HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.getRangeTo(@{&target.pos().0}))
    }

    pub fn in_range_to<T>(&self, target: &T, range: i32) -> bool
    where
        T: HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.inRangeTo(@{&target.pos().0}, @{range}))
    }

    pub fn is_equal_to<T>(&self, target: &T) -> bool
    where
        T: HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.isEqualTo(@{&target.pos().0}))
    }

    pub fn is_equal_to_xy(&self, x: u32, y: u32) -> bool {
        js_unwrap!{return @{self.as_ref()}.isEqualTo(@{x}, @{y});}
    }

    pub fn is_near_to<T>(&self, target: &T) -> bool
    where
        T: HasPosition,
    {
        js_unwrap!(@{self.as_ref()}.isNearTo(@{&target.pos().0}))
    }

    pub fn look_for<T>(&self, ty: T) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        T::convert_and_check_items(js_unwrap!{
            @{self.as_ref()}.lookFor(__look_num_to_str(@{ty.look_code() as i32}))
        })
    }
}

impl<T: HasPosition> PartialEq<T> for RoomPosition {
    fn eq(&self, other: &T) -> bool {
        (js!{
            var a = @{self.as_ref()};
            var b = @{&other.pos().0};
            return a.x == b.x && a.y == b.y && a.roomName == b.roomName;
        }).try_into()
            .expect("expected a boolean to be a boolean")
    }
}

impl Eq for RoomPosition {}
