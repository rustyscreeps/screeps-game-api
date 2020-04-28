//! Game method implementations on `Position`
use crate::{
    constants::{Color, FindConstant, LookConstant, ReturnCode, StructureType},
    game,
    local::RoomName,
    objects::{FindOptions, Flag, HasPosition, LookResult, Path},
    pathfinder::CostMatrix,
};

use super::Position;

impl Position {
    pub fn create_construction_site(self, ty: StructureType) -> ReturnCode {
        js_unwrap!(
            pos_from_packed(@{self.packed_repr()})
                .createConstructionSite(__structure_type_num_to_str(@{ty as u32}))
        )
    }

    pub fn create_named_construction_site(self, ty: StructureType, name: &str) -> ReturnCode {
        js_unwrap!(
            pos_from_packed(@{self.packed_repr()})
                .createConstructionSite(__structure_type_num_to_str(@{ty as u32}), @{name})
        )
    }

    pub fn create_flag(
        self,
        name: &str,
        main_color: Color,
        secondary_color: Color,
    ) -> Result<String, ReturnCode> {
        // TODO: determine if ERR_NOT_IN_RANGE is the best choice here
        //
        // JavaScript code simply throws an error on unknown rooms, which isn't ideal.
        Flag::interpret_creation_ret_value(js! {
            let pos = pos_from_packed(@{self.packed_repr()});
            if (pos.roomName in Game.rooms) {
                return pos.createFlag(@{name}, @{main_color as u32}, @{secondary_color as u32});
            } else {
                return ERR_NOT_IN_RANGE;
            }
        })
        .expect("expected RoomPosition.createFlag to return ReturnCode or String name")
    }

    pub fn find_closest_by_range<T>(self, ty: T) -> Option<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(
            pos_from_packed(@{self.packed_repr()})
                .findClosestByRange(@{ty.find_code()})
        )
    }

    pub fn find_in_range<T>(self, ty: T, range: u32) -> Vec<T::Item>
    where
        T: FindConstant,
    {
        js_unwrap_ref!(
            pos_from_packed(@{self.packed_repr()})
                .findInRange(@{ty.find_code()}, @{range})
        )
    }

    pub fn find_path_to<'a, F, T>(self, target: &T, opts: FindOptions<'a, F>) -> Path
    where
        F: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'a>> + 'a,
        T: ?Sized + HasPosition,
    {
        let self_room = game::rooms::get(self.room_name()).unwrap();
        self_room.find_path(&self, target, opts)
    }

    pub fn find_path_to_xy<'a, F>(self, x: u32, y: u32, opts: FindOptions<'a, F>) -> Path
    where
        F: Fn(RoomName, CostMatrix<'_>) -> Option<CostMatrix<'a>> + 'a,
    {
        let target = Position::new(x, y, self.room_name());
        self.find_path_to(&target, opts)
    }

    pub fn look(self) -> Vec<LookResult> {
        js_unwrap!(pos_from_packed(@{self.packed_repr()}).look())
    }

    pub fn look_for<T>(self, ty: T) -> Vec<T::Item>
    where
        T: LookConstant,
    {
        T::convert_and_check_items(js_unwrap! {
            pos_from_packed(@{self.packed_repr()})
            .lookFor(__look_num_to_str(@{ty.look_code() as u32}))
        })
    }
}
